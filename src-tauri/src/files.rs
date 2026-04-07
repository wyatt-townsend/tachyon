// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use ignore::{WalkBuilder, WalkState};
use std::sync::{atomic::{AtomicUsize, Ordering}, mpsc, Arc};
use sysinfo::Disks;

pub fn enumerate_files_with_progress<F>(on_progress: F) -> Vec<String>
where
    F: Fn(usize) + Send + Sync + 'static,
{
    let drives = get_drives();
    if drives.is_empty() {
        return Vec::new();
    }

    let root = &drives[0];
    let walker = WalkBuilder::new(root).build_parallel();
    let (tx, rx) = mpsc::channel();
    let count = Arc::new(AtomicUsize::new(0));
    let on_progress = Arc::new(on_progress);

    walker.run(|| {
        let tx = tx.clone();
        let count = count.clone();
        let on_progress = on_progress.clone();
        Box::new(move |entry| {
            if let Ok(e) = entry {
                if e.file_type().map_or(false, |f| f.is_file()) {
                    let _ = tx.send(e.path().to_string_lossy().to_string());
                    let c = count.fetch_add(1, Ordering::Relaxed) + 1;
                    if c % 5000 == 0 {
                        on_progress(c);
                    }
                }
            }
            WalkState::Continue
        })
    });

    drop(tx);
    rx.into_iter().collect()
}

pub fn get_drives() -> Vec<String> {
    let mut results: Vec<String> = Vec::new();
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        results.push(disk.mount_point().to_string_lossy().to_string())
    }

    results
}
