use crate::error::TachyonError;
use crate::index::entry::{EntryKind, FileEntry};
use std::path::{Path, PathBuf};

/// Recursively walks `root` and returns every file and directory entry found.
///
/// Errors on individual subdirectories (e.g. permission denied) are logged
/// as warnings and skipped — the walk continues rather than aborting.
pub fn walk_directory(root: &Path) -> Result<Vec<FileEntry>, TachyonError> {
    let mut files: Vec<FileEntry> = Vec::new();

    // Read the directory entries in `root`.
    let read_dir = std::fs::read_dir(root).map_err(|e| TachyonError::Io {
        path: root.to_path_buf(),
        source: e,
    })?;

    // Process each entry in the directory.
    for dir_entry in read_dir {
        // Check for errors reading the directory entry. If we can't read it skip it.
        let dir_entry = dir_entry.map_err(|e| TachyonError::Io {
            path: root.to_path_buf(),
            source: e,
        })?;

        // Get the path of the entry.
        let path: PathBuf = dir_entry.path();

        // Get the name of the entry as a UTF-8 string. If it contains non-UTF-8 characters, skip it.
        let name = dir_entry
            .file_name()
            .into_string()
            .map_err(TachyonError::NonUtf8Path)?;

        // Get the file type of the entry (file, directory, symlink, etc.).
        let file_type = dir_entry.file_type().map_err(|e| TachyonError::Io {
            path: path.clone(),
            source: e,
        })?;

        // Determine if the entry is a file or directory. Skip symlinks, junctions, devices, etc.
        let kind = if file_type.is_file() {
            EntryKind::File
        } else if file_type.is_dir() {
            EntryKind::Directory
        } else {
            continue; // skip symlinks, junctions, devices
        };

        // Recurse before pushing so children appear after parent — depth-first.
        if kind == EntryKind::Directory {
            match walk_directory(&path) {
                Ok(mut children) => files.append(&mut children),
                Err(e) => eprintln!("Warning: skipping '{}': {e}", path.display()),
            }
        }

        // Add the entry to the results.
        files.push(FileEntry { name, path, kind });
    }

    // Return the collected entries.
    Ok(files)
}
