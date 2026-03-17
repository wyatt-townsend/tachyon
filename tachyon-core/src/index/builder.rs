use crate::error::TachyonError;
use crate::index::entry::{EntryKind, FileEntry};
use bincode;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

/// Recursively walks `root` and returns every file and directory entry found.
pub(crate) fn walk_directory(root: &Path) -> Result<Vec<FileEntry>, TachyonError> {
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

/// Builds an index of the directory tree rooted at `root` and writes it to `dest`.
pub fn build_index(root: &Path, dest: &Path) -> Result<(), TachyonError> {
    let files = walk_directory(root)?;

    // Create or clear file
    let index = File::create(dest).map_err(|e| TachyonError::Io {
        path: dest.to_path_buf(),
        source: e,
    })?;

    // Write files to index
    let mut writer = BufWriter::new(index);

    // Serialize
    bincode::encode_into_std_write(&files, &mut writer, bincode::config::standard())
        .map_err(TachyonError::Serialization)?;

    Ok(())
}

/// Loads an index from `src` and returns the list of file entries.
pub fn load_index(src: &Path) -> Result<Vec<FileEntry>, TachyonError> {
    // Attempt to open index file for reading
    let index = File::open(src).map_err(|e| TachyonError::Io {
        path: src.to_path_buf(),
        source: e,
    })?;

    let mut reader = BufReader::new(index);

    // Deserialize
    let files = bincode::decode_from_std_read(&mut reader, bincode::config::standard())
        .map_err(TachyonError::Deserialization)?;

    Ok(files)
}
