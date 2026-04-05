use std::sync::Mutex;

mod state;
use state::AppState;

mod files;

mod commands;
use commands::{build_file_path_list, fuzzy_filter};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage(AppState {
                file_paths: Mutex::new(Vec::new()),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![build_file_path_list, fuzzy_filter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
