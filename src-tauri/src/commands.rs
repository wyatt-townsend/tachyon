use crate::files::enumerate_files_with_progress;
use crate::state::AppState;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tauri::ipc::Channel;
use tokio::task::spawn_blocking;

const MAX_RESULTS: usize = 500;

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum IndexEvent {
    Progress { count: usize },
    Done { total: usize },
}

#[tauri::command]
pub async fn fuzzy_filter(
    pattern: String,
    state: tauri::State<'_, AppState>,
    on_event: Channel<SearchEvent>,
) -> Result<(), String> {
    let paths = state.file_paths.lock().unwrap().clone();
    let needle = pattern.to_lowercase();
    let gen_counter = state.search_generation.clone();
    let my_gen = gen_counter.fetch_add(1, Ordering::SeqCst) + 1;
    let result_count = Arc::new(AtomicUsize::new(0));

    spawn_blocking(move || {
        let _ = paths.par_iter().try_for_each(|f| {
            if gen_counter.load(Ordering::SeqCst) != my_gen {
                return Err("Cancelled");
            }
            if result_count.load(Ordering::Relaxed) >= MAX_RESULTS {
                return Err("Limit reached");
            }

            if f.to_lowercase().contains(&needle) {
                if result_count.fetch_add(1, Ordering::Relaxed) < MAX_RESULTS {
                    let _ = on_event.send(SearchEvent::Found {
                        path_string: f.clone(),
                    });
                } else {
                    return Err("Limit reached");
                }
            }

            Ok(())
        });
    })
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn build_file_path_list(
    state: tauri::State<'_, AppState>,
    on_event: Channel<IndexEvent>,
) -> Result<(), String> {
    let on_event_progress = on_event.clone();
    let paths = spawn_blocking(move || {
        enumerate_files_with_progress(move |count| {
            let _ = on_event_progress.send(IndexEvent::Progress { count });
        })
    })
    .await
    .map_err(|e| e.to_string())?;

    let total = paths.len();
    let mut state_paths = state.file_paths.lock().unwrap();
    state_paths.clear();
    state_paths.extend(paths);
    let _ = on_event.send(IndexEvent::Done { total });
    Ok(())
}

#[tauri::command]
pub fn cancel_search(state: tauri::State<'_, AppState>) {
    state.search_generation.fetch_add(1, Ordering::SeqCst);
}
