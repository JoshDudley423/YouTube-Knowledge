//! Manages a local `llamafile` subprocess (a single portable executable
//! bundling a quantized model + llama.cpp's OpenAI-compatible HTTP server)
//! so chat answers never leave the user's machine and never depend on a
//! cloud AI provider.

use serde::Serialize;
use serde_json::json;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::process::{Child, Command};
use tokio::sync::Mutex;

/// A small instruct model keeps CPU inference usable on an ordinary laptop.
/// Users who want a stronger local model can drop their own `.llamafile`
/// into the models directory under this same filename and it's used as-is.
pub const DEFAULT_MODEL_NAME: &str = "Qwen2.5-3B-Instruct.Q4_K_M.llamafile";
pub const DEFAULT_MODEL_URL: &str =
    "https://huggingface.co/Mozilla/Qwen2.5-3B-Instruct-llamafile/resolve/main/Qwen2.5-3B-Instruct.Q4_K_M.llamafile";
const LLAMAFILE_PORT: u16 = 8723;

/// llamafiles are Actually Portable Executables. Windows' loader only runs
/// files with a recognized executable extension, so the convention (per
/// the llamafile project) is to give the file a `.exe` suffix there.
pub fn model_filename() -> String {
    if cfg!(target_os = "windows") {
        format!("{DEFAULT_MODEL_NAME}.exe")
    } else {
        DEFAULT_MODEL_NAME.to_string()
    }
}

pub fn model_path(models_dir: &Path) -> PathBuf {
    models_dir.join(model_filename())
}

pub async fn is_downloaded(models_dir: &Path) -> bool {
    tokio::fs::metadata(model_path(models_dir))
        .await
        .map(|m| m.len() > 0)
        .unwrap_or(false)
}

/// Downloads the default model, invoking `on_progress(0.0..=1.0)` as bytes
/// arrive.
pub async fn download_default_model(
    models_dir: &Path,
    on_progress: impl Fn(f64) + Send + 'static,
) -> Result<(), String> {
    let dest = model_path(models_dir);
    let tmp = dest.with_extension("part");

    let client = reqwest::Client::new();
    let resp = client
        .get(DEFAULT_MODEL_URL)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        return Err(format!("model download failed: HTTP {}", resp.status()));
    }
    let total = resp.content_length().unwrap_or(0);

    use futures_util::StreamExt;
    use tokio::io::AsyncWriteExt;

    let mut file = tokio::fs::File::create(&tmp).await.map_err(|e| e.to_string())?;
    let mut stream = resp.bytes_stream();
    let mut downloaded: u64 = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        file.write_all(&chunk).await.map_err(|e| e.to_string())?;
        downloaded += chunk.len() as u64;
        if total > 0 {
            on_progress(downloaded as f64 / total as f64);
        }
    }
    file.flush().await.ok();
    drop(file);
    tokio::fs::rename(&tmp, &dest).await.map_err(|e| e.to_string())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = tokio::fs::metadata(&dest)
            .await
            .map_err(|e| e.to_string())?
            .permissions();
        perms.set_mode(0o755);
        tokio::fs::set_permissions(&dest, perms)
            .await
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

pub struct LlmManager {
    child: Mutex<Option<Child>>,
}

impl Default for LlmManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmManager {
    pub fn new() -> Self {
        LlmManager {
            child: Mutex::new(None),
        }
    }

    /// Starts the llamafile server if it isn't already running, and waits
    /// until it responds to health checks.
    pub async fn ensure_running(&self, models_dir: &Path) -> Result<(), String> {
        {
            let mut guard = self.child.lock().await;
            if let Some(child) = guard.as_mut() {
                if child.try_wait().map_err(|e| e.to_string())?.is_none() {
                    return Ok(());
                }
            }
        }

        let path = model_path(models_dir);
        if !path.exists() {
            return Err("local model not downloaded yet".to_string());
        }

        let mut cmd = Command::new(&path);
        cmd.args([
            "--server",
            "--nobrowser",
            "--host",
            "127.0.0.1",
            "--port",
            &LLAMAFILE_PORT.to_string(),
            "-c",
            "4096",
        ]);
        cmd.stdout(Stdio::null()).stderr(Stdio::null());

        let child = cmd
            .spawn()
            .map_err(|e| format!("failed to start local model server: {e}"))?;
        *self.child.lock().await = Some(child);

        let client = reqwest::Client::new();
        for _ in 0..60 {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            if client
                .get(format!("http://127.0.0.1:{LLAMAFILE_PORT}/health"))
                .send()
                .await
                .is_ok()
            {
                return Ok(());
            }
        }
        Err("timed out waiting for local model server to start".to_string())
    }

    pub async fn stop(&self) {
        if let Some(mut child) = self.child.lock().await.take() {
            let _ = child.kill().await;
        }
    }
}

#[derive(Serialize)]
struct ChatMsg<'a> {
    role: &'a str,
    content: &'a str,
}

/// Streams a chat completion from the local server's OpenAI-compatible
/// `/v1/chat/completions` endpoint, calling `on_token` per text delta.
pub async fn stream_chat(
    system_prompt: &str,
    user_prompt: &str,
    mut on_token: impl FnMut(String),
) -> Result<(), String> {
    let client = reqwest::Client::new();
    let body = json!({
        "model": "local",
        "messages": [
            ChatMsg { role: "system", content: system_prompt },
            ChatMsg { role: "user", content: user_prompt },
        ],
        "stream": true,
        "temperature": 0.3,
    });

    let resp = client
        .post(format!("http://127.0.0.1:{LLAMAFILE_PORT}/v1/chat/completions"))
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("local model server returned HTTP {}", resp.status()));
    }

    use futures_util::StreamExt;
    let mut stream = resp.bytes_stream();
    let mut buf = String::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| e.to_string())?;
        buf.push_str(&String::from_utf8_lossy(&chunk));
        while let Some(pos) = buf.find('\n') {
            let line = buf[..pos].trim().to_string();
            buf.drain(..=pos);
            let Some(data) = line.strip_prefix("data: ") else {
                continue;
            };
            if data == "[DONE]" {
                continue;
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
                if let Some(delta) = v["choices"][0]["delta"]["content"].as_str() {
                    on_token(delta.to_string());
                }
            }
        }
    }
    Ok(())
}
