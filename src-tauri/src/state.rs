use std::sync::{atomic::AtomicUsize, Arc, Mutex};

pub struct AppState {
    pub file_paths: Mutex<Vec<String>>,
    pub search_generation: Arc<AtomicUsize>,
}
