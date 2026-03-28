// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use ignore::WalkBuilder;
use serde::{Deserialize, Serialize};
use tauri::ipc::Channel;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    tag = "event",
    content = "data"
)]
pub enum SearchEvent {
    Progress { path_string: String, is_dir: bool },
    Result { total: usize },
}

#[tauri::command]
pub async fn walk_directory(root: String, on_event: Channel<SearchEvent>) {
    let walker = WalkBuilder::new(root).hidden(false).ignore(true).build();
    let mut total_entries: usize = 0;

    for entry in walker {
        if let Ok(e) = entry {
            on_event
                .send(SearchEvent::Progress {
                    path_string: e.path().to_string_lossy().to_string(),
                    is_dir: e.file_type().unwrap().is_dir(),
                })
                .unwrap();

            total_entries += 1;
        }
    }

    on_event
        .send(SearchEvent::Result {
            total: total_entries,
        })
        .unwrap()
}
