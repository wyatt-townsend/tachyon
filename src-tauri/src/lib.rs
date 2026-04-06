use std::sync::{atomic::AtomicBool, Arc, Mutex};

mod state;
use state::AppState;

mod files;

mod commands;
use commands::{build_file_path_list, cancel_search, fuzzy_filter};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState {
                file_paths: Mutex::new(Vec::new()),
                cancel_search: Arc::new(AtomicBool::new(false)),
            });

            #[cfg(target_os = "windows")]
            {
                use window_vibrancy::apply_mica;
                if let Some(window) = app.get_webview_window("main") {
                    let _ = apply_mica(&window, Some(true));
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            build_file_path_list,
            fuzzy_filter,
            cancel_search
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
