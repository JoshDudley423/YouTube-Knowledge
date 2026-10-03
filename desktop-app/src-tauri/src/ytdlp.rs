use crate::models::VideoMeta;
use crate::transcript::vtt_to_text;
use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_shell::ShellExt;

/// Works around YouTube's "sign in to confirm you're not a bot" wall that
/// yt-dlp's default 'web' player client hits from a fresh IP.
const BOT_WORKAROUND_ARGS: [&str; 2] = ["--extractor-args", "youtube:player_client=android,tv,web_safari"];

/// Points at the channel's `/videos` tab specifically, so Shorts (which
/// live under `/shorts`) are never listed or fetched.
fn videos_tab_url(channel_url: &str) -> String {
    let trimmed = channel_url.trim().trim_end_matches('/');
    if trimmed.ends_with("/videos") || trimmed.ends_with("/shorts") || trimmed.ends_with("/streams") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/videos")
    }
}

pub struct VideoStub {
    pub id: String,
    pub title: String,
}

/// Lists every long-form video on a channel via a single flat-playlist
/// call (cheap: no per-video metadata fetch).
pub async fn list_channel_videos(app: &AppHandle, channel_url: &str) -> Result<Vec<VideoStub>, String> {
    let url = videos_tab_url(channel_url);
    let cmd = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| e.to_string())?
        .args(["--flat-playlist", "--dump-single-json", "--no-warnings", &url]);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(&stdout).map_err(|e| e.to_string())?;
    let entries = json
        .get("entries")
        .and_then(|e| e.as_array())
        .cloned()
        .unwrap_or_default();

    let mut out = Vec::new();
    for entry in entries {
        let id = entry.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
        if id.is_empty() {
            continue;
        }
        let title = entry.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
        out.push(VideoStub { id, title });
    }
    Ok(out)
}

/// Downloads captions (manual first, falling back to auto-generated) for a
/// single video and returns the cleaned transcript text plus metadata.
/// `has_transcript` is false (and text empty) when the video has no
/// captions available at all -- that's a normal, expected outcome.
pub async fn fetch_video_transcript(app: &AppHandle, video_id: &str) -> Result<(VideoMeta, String), String> {
    let video_url = format!("https://www.youtube.com/watch?v={video_id}");
    let tmp = std::env::temp_dir().join(format!("ytk-{video_id}-{}", uuid::Uuid::new_v4()));
    tokio::fs::create_dir_all(&tmp)
        .await
        .map_err(|e| e.to_string())?;
    let out_template = tmp.join("%(id)s.%(ext)s");

    let mut args: Vec<String> = vec![
        "--skip-download".into(),
        "--write-subs".into(),
        "--write-auto-subs".into(),
        "--sub-lang".into(),
        "en.*".into(),
        "--sub-format".into(),
        "vtt".into(),
        "--dump-json".into(),
        "--no-warnings".into(),
        "-o".into(),
        out_template.to_string_lossy().into_owned(),
    ];
    args.extend(BOT_WORKAROUND_ARGS.iter().map(|s| s.to_string()));
    args.push(video_url.clone());

    let cmd = app
        .shell()
        .sidecar("yt-dlp")
        .map_err(|e| e.to_string())?
        .args(args);

    let output = cmd.output().await.map_err(|e| e.to_string())?;
    if !output.status.success() {
        tokio::fs::remove_dir_all(&tmp).await.ok();
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let info: Value = stdout
        .lines()
        .find_map(|l| serde_json::from_str::<Value>(l).ok())
        .ok_or_else(|| "yt-dlp produced no metadata JSON".to_string())?;

    let title = info.get("title").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let upload_date = info
        .get("upload_date")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let duration = info.get("duration").and_then(|v| v.as_f64());

    let mut vtt_path = None;
    let mut entries = tokio::fs::read_dir(&tmp).await.map_err(|e| e.to_string())?;
    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("vtt") {
            vtt_path = Some(path);
            break;
        }
    }

    let (text, has_transcript) = match vtt_path {
        Some(path) => {
            let raw = tokio::fs::read_to_string(&path).await.unwrap_or_default();
            (vtt_to_text(&raw), true)
        }
        None => (String::new(), false),
    };

    tokio::fs::remove_dir_all(&tmp).await.ok();

    Ok((
        VideoMeta {
            id: video_id.to_string(),
            title,
            url: video_url,
            upload_date,
            duration,
            has_transcript,
        },
        text,
    ))
}
