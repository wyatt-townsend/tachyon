use crate::files::enumerate_files;
use crate::state::AppState;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use tauri::ipc::Channel;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum SearchEvent {
    Found { path_string: String },
}

#[tauri::command]
pub fn fuzzy_filter(
    pattern: String,
    state: tauri::State<AppState>,
    on_event: Channel<SearchEvent>,
) {
    let paths = state.file_paths.lock().unwrap();
    let needle = pattern.to_lowercase();

    state.cancel_search.store(false, Ordering::Relaxed);
    let cancel = state.cancel_search.clone();

    let _ = paths.par_iter().try_for_each(|f| {
        if cancel.load(Ordering::Relaxed) {
            return Err("Cancelled");
        }

        if f.to_lowercase().contains(&needle) {
            let _ = on_event.send(SearchEvent::Found {
                path_string: f.clone(),
            });
        }

        Ok(())
    });
}

#[tauri::command]
pub fn build_file_path_list(state: tauri::State<AppState>) {
    let mut paths = state.file_paths.lock().unwrap();
    paths.clear();
    paths.extend(enumerate_files());
}

#[tauri::command]
pub fn cancel_search(state: tauri::State<'_, AppState>) {
    state.cancel_search.store(true, Ordering::Relaxed);
}
