use crate::llm;
use crate::models::{Channel, ChatChunk, ModelStatus, RetrievedSource, SyncProgress, VideoMeta};
use crate::paths;
use crate::state::{self, AppState};
use crate::ytdlp;
use tauri::{AppHandle, Emitter, State};

fn slugify(name: &str) -> String {
    let mut slug: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();
    while slug.contains("--") {
        slug = slug.replace("--", "-");
    }
    slug.trim_matches('-').to_string()
}

#[tauri::command]
pub async fn list_channels(state: State<'_, AppState>) -> Result<Vec<Channel>, String> {
    Ok(state.channels.lock().await.clone())
}

#[tauri::command]
pub async fn add_channel(
    app: AppHandle,
    state: State<'_, AppState>,
    url: String,
    name: String,
    topic: String,
) -> Result<Channel, String> {
    let mut channels = state.channels.lock().await;

    let base_slug = slugify(&name);
    let mut slug = base_slug.clone();
    let mut n = 2;
    while channels.iter().any(|c| c.slug == slug) {
        slug = format!("{base_slug}-{n}");
        n += 1;
    }

    let channel = Channel {
        slug: slug.clone(),
        name,
        url,
        topic,
        added: chrono::Utc::now().to_rfc3339(),
        last_synced: None,
        video_count: 0,
    };

    channels.push(channel.clone());
    state::save_channels(&app, &channels).await?;
    Ok(channel)
}

#[tauri::command]
pub async fn remove_channel(app: AppHandle, state: State<'_, AppState>, slug: String) -> Result<(), String> {
    let mut channels = state.channels.lock().await;
    channels.retain(|c| c.slug != slug);
    state::save_channels(&app, &channels).await?;
    let dir = paths::transcripts_dir(&app, &slug);
    tokio::fs::remove_dir_all(&dir).await.ok();
    let rebuilt = state::rebuild_index(&app, &channels).await;
    *state.index.lock().await = Some(rebuilt);
    Ok(())
}

/// Lists the channel's videos, fetches captions for any not already on
/// disk, and rebuilds the search index. Emits `sync-progress` events as it
/// goes so the UI can show a progress bar.
#[tauri::command]
pub async fn sync_channel(app: AppHandle, state: State<'_, AppState>, slug: String) -> Result<(), String> {
    let channel_url = {
        let channels = state.channels.lock().await;
        channels
            .iter()
            .find(|c| c.slug == slug)
            .map(|c| c.url.clone())
            .ok_or_else(|| format!("no channel registered with slug '{slug}'"))?
    };

    let emit = |progress: &SyncProgress| {
        let _ = app.emit("sync-progress", progress.clone());
    };

    let videos = match ytdlp::list_channel_videos(&app, &channel_url).await {
        Ok(v) => v,
        Err(e) => {
            emit(&SyncProgress {
                slug: slug.clone(),
                done: 0,
                total: 0,
                current_title: String::new(),
                finished: true,
                error: Some(e.clone()),
            });
            return Err(e);
        }
    };

    let total = videos.len();
    let mut done = 0;
    for video in &videos {
        let txt_path = paths::transcript_txt_path(&app, &slug, &video.id);
        if tokio::fs::metadata(&txt_path).await.is_ok() {
            done += 1;
            continue;
        }

        emit(&SyncProgress {
            slug: slug.clone(),
            done,
            total,
            current_title: video.title.clone(),
            finished: false,
            error: None,
        });

        match ytdlp::fetch_video_transcript(&app, &video.id).await {
            Ok((meta, text)) => {
                let meta_path = paths::transcript_meta_path(&app, &slug, &video.id);
                let meta_json = serde_json::to_string_pretty(&meta).unwrap_or_default();
                tokio::fs::write(&meta_path, meta_json).await.ok();
                if meta.has_transcript {
                    tokio::fs::write(&txt_path, text).await.ok();
                }
            }
            Err(e) => {
                emit(&SyncProgress {
                    slug: slug.clone(),
                    done,
                    total,
                    current_title: format!("{} (skipped: {e})", video.title),
                    finished: false,
                    error: None,
                });
            }
        }
        done += 1;
    }

    {
        let mut channels = state.channels.lock().await;
        if let Some(c) = channels.iter_mut().find(|c| c.slug == slug) {
            c.last_synced = Some(chrono::Utc::now().to_rfc3339());
            c.video_count = total;
        }
        state::save_channels(&app, &channels).await?;
        let rebuilt = state::rebuild_index(&app, &channels).await;
        *state.index.lock().await = Some(rebuilt);
    }

    emit(&SyncProgress {
        slug,
        done,
        total,
        current_title: String::new(),
        finished: true,
        error: None,
    });

    Ok(())
}

#[tauri::command]
pub async fn model_status(app: AppHandle) -> Result<ModelStatus, String> {
    let dir = paths::models_dir(&app);
    let downloaded = llm::is_downloaded(&dir).await;
    Ok(ModelStatus {
        downloaded,
        running: false,
        model_path: if downloaded {
            Some(llm::model_path(&dir).to_string_lossy().into_owned())
        } else {
            None
        },
        download_progress: None,
    })
}

#[tauri::command]
pub async fn download_model(app: AppHandle) -> Result<(), String> {
    let dir = paths::models_dir(&app);
    let app_for_progress = app.clone();
    llm::download_default_model(&dir, move |frac| {
        let _ = app_for_progress.emit("model-download-progress", frac);
    })
    .await
}

/// Retrieves the most relevant transcript chunks for `question` (optionally
/// scoped to one channel), builds a grounded prompt, and streams the local
/// model's answer back as `chat-token` events. A `chat-sources` event lists
/// what was retrieved, and a final `chat-token` with `done: true` closes
/// the turn.
#[tauri::command]
pub async fn ask_question(
    app: AppHandle,
    state: State<'_, AppState>,
    slug: Option<String>,
    question: String,
) -> Result<(), String> {
    let models_dir = paths::models_dir(&app);
    state.llm.ensure_running(&models_dir).await?;

    let sources: Vec<RetrievedSource> = {
        let index_guard = state.index.lock().await;
        match index_guard.as_ref() {
            Some(index) => index
                .search(&question, 6, slug.as_deref())
                .into_iter()
                .map(|(chunk, _score)| RetrievedSource {
                    slug: chunk.slug.clone(),
                    video_id: chunk.video_id.clone(),
                    title: chunk.title.clone(),
                    url: chunk.url.clone(),
                    snippet: chunk.text.clone(),
                })
                .collect(),
            None => Vec::new(),
        }
    };

    let _ = app.emit("chat-sources", &sources);

    let context = if sources.is_empty() {
        "No relevant transcript passages were found in the knowledge base.".to_string()
    } else {
        sources
            .iter()
            .enumerate()
            .map(|(i, s)| format!("[{}] {} -- {}\n{}", i + 1, s.title, s.url, s.snippet))
            .collect::<Vec<_>>()
            .join("\n\n")
    };

    let system_prompt = "You are a helpful assistant that answers questions using ONLY the \
        transcript excerpts provided as context. Cite sources by their [number]. If the context \
        doesn't contain the answer, say so plainly instead of guessing.";
    let user_prompt = format!("Context:\n{context}\n\nQuestion: {question}");

    let app_for_tokens = app.clone();
    let result = llm::stream_chat(system_prompt, &user_prompt, move |token| {
        let _ = app_for_tokens.emit(
            "chat-token",
            ChatChunk {
                token,
                done: false,
            },
        );
    })
    .await;

    let _ = app.emit(
        "chat-token",
        ChatChunk {
            token: String::new(),
            done: true,
        },
    );

    result
}

#[tauri::command]
pub async fn get_video_meta(app: AppHandle, slug: String, video_id: String) -> Result<VideoMeta, String> {
    let path = paths::transcript_meta_path(&app, &slug, &video_id);
    let raw = tokio::fs::read_to_string(&path).await.map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}
