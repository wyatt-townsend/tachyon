use std::sync::Mutex;

pub struct AppState {
    pub file_paths: Mutex<Vec<String>>,
}
