use crate::error::TachyonError;
use crate::index::entry::{EntryKind, FileEntry};
use crate::index::{EntryMap, Index, SCHEMA_VERSION};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time;

pub(crate) struct IndexBuilder {
    root: PathBuf,
}

impl IndexBuilder {
    pub fn new(drive: &Path) -> Self {
        Self {
            root: drive.to_owned(),
        }
    }

    /// Builds an index of the directory tree rooted at `root` and writes it to `dest`.
    pub fn build(&mut self) -> Result<Index, TachyonError> {
        let file_map = walk_directory(&self.root)?;

        // Return the collected entries.
        Ok(Index {
            entries: file_map,
            root: self.root.clone(),
            built_at: time::SystemTime::now(),
            schema_version: SCHEMA_VERSION,
        })
    }
}

/// Recursively walks `root` and returns every file and directory entry found.
fn walk_directory(root: &Path) -> Result<EntryMap, TachyonError> {
    let mut files: EntryMap = BTreeMap::new();

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
                Ok(children) => files.extend(children),
                Err(_e) => (),
            }
        }

        // Add the entry to the results.
        files.entry(path).or_insert(FileEntry { name, kind });
    }

    // Return the collected entries.
    Ok(files)
}
