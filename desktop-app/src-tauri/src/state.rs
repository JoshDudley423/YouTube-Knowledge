use crate::index::{chunk_transcript, SearchIndex};
use crate::llm::LlmManager;
use crate::models::Channel;
use crate::paths;
use tauri::AppHandle;
use tokio::sync::Mutex;

pub struct AppState {
    pub channels: Mutex<Vec<Channel>>,
    pub index: Mutex<Option<SearchIndex>>,
    pub llm: LlmManager,
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            channels: Mutex::new(Vec::new()),
            index: Mutex::new(None),
            llm: LlmManager::new(),
        }
    }
}

pub async fn load_channels(app: &AppHandle) -> Vec<Channel> {
    let path = paths::channels_file(app);
    match tokio::fs::read_to_string(&path).await {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}

pub async fn save_channels(app: &AppHandle, channels: &[Channel]) -> Result<(), String> {
    let path = paths::channels_file(app);
    let json = serde_json::to_string_pretty(channels).map_err(|e| e.to_string())?;
    tokio::fs::write(&path, json).await.map_err(|e| e.to_string())
}

/// Rebuilds the in-memory BM25 index from every transcript on disk for
/// every registered channel. Cheap enough (a few hundred videos) to just
/// redo in full after any sync or on startup, rather than maintaining
/// incremental updates.
pub async fn rebuild_index(app: &AppHandle, channels: &[Channel]) -> SearchIndex {
    let mut all_chunks = Vec::new();
    for channel in channels {
        let dir = paths::transcripts_dir(app, &channel.slug);
        let mut entries = match tokio::fs::read_dir(&dir).await {
            Ok(e) => e,
            Err(_) => continue,
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("txt") {
                continue;
            }
            let video_id = match path.file_stem().and_then(|s| s.to_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };
            let text = tokio::fs::read_to_string(&path).await.unwrap_or_default();
            if text.trim().is_empty() {
                continue;
            }
            let meta_path = paths::transcript_meta_path(app, &channel.slug, &video_id);
            let (title, url) = match tokio::fs::read_to_string(&meta_path).await {
                Ok(raw) => match serde_json::from_str::<crate::models::VideoMeta>(&raw) {
                    Ok(m) => (m.title, m.url),
                    Err(_) => (video_id.clone(), String::new()),
                },
                Err(_) => (video_id.clone(), String::new()),
            };
            all_chunks.extend(chunk_transcript(&channel.slug, &video_id, &title, &url, &text));
        }
    }
    SearchIndex::build(all_chunks)
}
