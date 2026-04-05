// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use ignore::{WalkBuilder, WalkState};
use std::sync::mpsc;
use sysinfo::Disks;

pub fn enumerate_files() -> Vec<String> {
    // Setup
    let drives = get_drives();
    if drives.len() <= 0 {
        return Vec::new();
    }

    let root = &drives[0];
    let walker = WalkBuilder::new(root).build_parallel();
    let (tx, rx) = mpsc::channel();

    // Search for files
    walker.run(|| {
        let tx = tx.clone();
        Box::new(move |entry| {
            if let Ok(e) = entry {
                if e.file_type().map_or(false, |f| f.is_file()) {
                    let _ = tx.send(e.path().to_string_lossy().to_string());
                }
            }
            WalkState::Continue
        })
    });

    // Return results
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
