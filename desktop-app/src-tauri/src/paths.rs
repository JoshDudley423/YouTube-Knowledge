use std::path::PathBuf;
use tauri::{AppHandle, Manager};

pub fn app_data_dir(app: &AppHandle) -> PathBuf {
    let dir = app
        .path()
        .app_data_dir()
        .expect("app data dir should be resolvable on all supported platforms");
    std::fs::create_dir_all(&dir).ok();
    dir
}

pub fn channels_file(app: &AppHandle) -> PathBuf {
    app_data_dir(app).join("channels.json")
}

pub fn transcripts_dir(app: &AppHandle, slug: &str) -> PathBuf {
    let dir = app_data_dir(app).join("transcripts").join(slug);
    std::fs::create_dir_all(&dir).ok();
    dir
}

pub fn models_dir(app: &AppHandle) -> PathBuf {
    let dir = app_data_dir(app).join("models");
    std::fs::create_dir_all(&dir).ok();
    dir
}

pub fn transcript_txt_path(app: &AppHandle, slug: &str, video_id: &str) -> PathBuf {
    transcripts_dir(app, slug).join(format!("{video_id}.txt"))
}

pub fn transcript_meta_path(app: &AppHandle, slug: &str, video_id: &str) -> PathBuf {
    transcripts_dir(app, slug).join(format!("{video_id}.json"))
}
