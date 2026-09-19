mod commands;
mod index;
mod llm;
mod models;
mod paths;
mod state;
mod transcript;
mod ytdlp;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(AppState::default())
        .setup(|app| {
            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let channels = state::load_channels(&handle).await;
                let index = state::rebuild_index(&handle, &channels).await;
                let app_state = handle.state::<AppState>();
                *app_state.channels.lock().await = channels;
                *app_state.index.lock().await = Some(index);
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { .. } = event {
                let app_state = window.state::<AppState>();
                tauri::async_runtime::block_on(app_state.llm.stop());
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::list_channels,
            commands::add_channel,
            commands::remove_channel,
            commands::sync_channel,
            commands::model_status,
            commands::download_model,
            commands::ask_question,
            commands::get_video_meta,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
