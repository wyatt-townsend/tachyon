use std::sync::{atomic::AtomicBool, Arc, Mutex};

pub struct AppState {
    pub file_paths: Mutex<Vec<String>>,
    pub cancel_search: Arc<AtomicBool>,
}
