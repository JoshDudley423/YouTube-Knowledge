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

const LLAMAFILE_PORT: u16 = 8723;

/// llamafiles are Actually Portable Executables. Windows' loader only runs
/// files with a recognized executable extension, so the convention (per
/// the llamafile project) is to give the file a `.llamafile.exe` suffix
/// there -- a plain browser download needs that rename on Windows.
fn looks_like_a_model(file_name: &str) -> bool {
    let lower = file_name.to_lowercase();
    lower.ends_with(".llamafile") || lower.ends_with(".llamafile.exe")
}

/// There's deliberately no single expected filename or auto-downloaded
/// default here: a hardcoded download URL is a single point of failure (a
/// model gets renamed, moved, or gated and the app breaks for everyone).
/// Instead, any `.llamafile` the user drops into the models directory --
/// downloaded themselves from wherever they like -- is picked up as-is.
pub async fn find_model(models_dir: &Path) -> Option<PathBuf> {
    let mut entries = tokio::fs::read_dir(models_dir).await.ok()?;
    while let Ok(Some(entry)) = entries.next_entry().await {
        let path = entry.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                if looks_like_a_model(name) {
                    return Some(path);
                }
            }
        }
    }
    None
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

        let path = find_model(models_dir)
            .await
            .ok_or_else(|| "no local model found in the models folder yet".to_string())?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Ok(meta) = tokio::fs::metadata(&path).await {
                let mut perms = meta.permissions();
                if perms.mode() & 0o111 == 0 {
                    perms.set_mode(perms.mode() | 0o755);
                    tokio::fs::set_permissions(&path, perms).await.ok();
                }
            }
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
