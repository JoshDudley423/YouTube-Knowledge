use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub slug: String,
    pub name: String,
    pub url: String,
    pub topic: String,
    pub added: String,
    pub last_synced: Option<String>,
    #[serde(default)]
    pub video_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMeta {
    pub id: String,
    pub title: String,
    pub url: String,
    pub upload_date: Option<String>,
    pub duration: Option<f64>,
    #[serde(default)]
    pub has_transcript: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncProgress {
    pub slug: String,
    pub done: usize,
    pub total: usize,
    pub current_title: String,
    pub finished: bool,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChunk {
    pub token: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedSource {
    pub slug: String,
    pub video_id: String,
    pub title: String,
    pub url: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelStatus {
    pub downloaded: bool,
    pub running: bool,
    pub model_path: Option<String>,
}
