// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use ignore::{overrides::OverrideBuilder, WalkBuilder, WalkState};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use sysinfo::Disks;
use tauri::ipc::Channel;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum SearchEvent {
    Progress { path_string: String },
    Result { total: usize },
}

#[tauri::command]
pub async fn walk_directory(root: String, pattern: String, on_event: Channel<SearchEvent>) {
    // Setup
    let overrides = OverrideBuilder::new(&root)
        .add(&pattern)
        .unwrap()
        .build()
        .unwrap();

    let walker = WalkBuilder::new(&root)
        .overrides(overrides)
        .build_parallel();

    let total_entries = Arc::new(AtomicUsize::new(0));

    // Search for files
    walker.run(|| {
        let total_entries = Arc::clone(&total_entries);
        let chan = on_event.clone();
        Box::new(move |entry| {
            if let Ok(e) = entry {
                if e.file_type().map_or(false, |f| f.is_file()) {
                    chan.send(SearchEvent::Progress {
                        path_string: e.path().to_string_lossy().to_string(),
                    })
                    .unwrap();

                    total_entries.fetch_add(1, Ordering::Relaxed);
                }
            }
            WalkState::Continue
        })
    });

    // Send done
    let n = total_entries.load(Ordering::Relaxed);
    on_event.send(SearchEvent::Result { total: n }).unwrap()
}

#[tauri::command]
pub async fn get_drives() -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        results.push(disk.mount_point().to_string_lossy().to_string())
    }

    results
}
