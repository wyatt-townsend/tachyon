use crate::files::enumerate_files;
use crate::state::AppState;

#[tauri::command]
pub fn fuzzy_filter(pattern: String, state: tauri::State<AppState>) -> Vec<String> {
    let paths = state.file_paths.lock().unwrap();
    let needle = pattern.to_lowercase();

    paths
        .iter()
        .filter(|path| path.to_lowercase().contains(&needle))
        .cloned()
        .collect()
}

#[tauri::command]
pub fn build_file_path_list(state: tauri::State<AppState>) {
    let mut paths = state.file_paths.lock().unwrap();
    paths.clear();
    paths.extend(enumerate_files());
}
