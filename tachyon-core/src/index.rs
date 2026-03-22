//! This module provides functionality for building and loading an index of a directory tree.
//! The index is a binary file that contains metadata about the files and directories in the
//! tree, such as their names, paths, and types (file or directory).
//! The `build_index` function recursively walks through the directory tree and collects this metadata,
//! while the `load_index` function reads the binary index file and reconstructs the metadata in memory.

pub mod builder;
pub mod entry;
use bincode::{Decode, Encode};
use std::fs::{File, create_dir_all};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::error::TachyonError;
pub use entry::EntryMap;

pub const MAGIC: &[u8; 4] = b"TACH";
pub const SCHEMA_VERSION: u16 = 1;

#[derive(Decode, Encode, Debug)]
pub struct Index {
    pub entries: EntryMap,
    pub root: PathBuf,
    pub built_at: SystemTime,
    pub schema_version: u16,
}

impl Index {
    /// Saves the index to a file at the given path.
    pub fn save(&self, dest: &Path) -> Result<(), TachyonError> {
        // Ensure parent directory exists
        if let Some(parent) = dest.parent() {
            create_dir_all(parent).map_err(|e| TachyonError::Io {
                path: parent.to_path_buf(),
                source: e,
            })?;
        }

        // Create or clear the file
        let index_file = File::create(dest).map_err(|e| TachyonError::Io {
            path: dest.to_path_buf(),
            source: e,
        })?;

        let mut writer = BufWriter::new(index_file);

        // Write magic bytes and schema version for validation
        writer.write_all(MAGIC).map_err(|e| TachyonError::Io {
            path: dest.to_path_buf(),
            source: e,
        })?;
        writer
            .write_all(&self.schema_version.to_le_bytes())
            .map_err(|e| TachyonError::Io {
                path: dest.to_path_buf(),
                source: e,
            })?;

        // Serialize the index data
        bincode::encode_into_std_write(&self, &mut writer, bincode::config::standard())
            .map_err(TachyonError::Serialization)?;

        Ok(())
    }

    /// Loads an index from a file at the given path.
    pub fn load(path: &Path) -> Result<Self, TachyonError> {
        let index_file = File::open(path).map_err(|e| TachyonError::Io {
            path: path.to_path_buf(),
            source: e,
        })?;

        let mut reader = BufReader::new(index_file);

        // Read and validate magic bytes
        let mut magic = [0u8; 4];
        reader
            .read_exact(&mut magic)
            .map_err(|e| TachyonError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;

        if &magic != MAGIC {
            return Err(TachyonError::InvalidIndexFile(
                "Invalid magic bytes".to_string(),
            ));
        }

        // Read and validate schema version
        let mut version_bytes = [0u8; 2];
        reader
            .read_exact(&mut version_bytes)
            .map_err(|e| TachyonError::Io {
                path: path.to_path_buf(),
                source: e,
            })?;

        let version = u16::from_le_bytes(version_bytes);
        if version != SCHEMA_VERSION {
            return Err(TachyonError::IncompatibleSchemaVersion {
                expected: SCHEMA_VERSION,
                found: version,
            });
        }

        // Deserialize the index
        let index: Index = bincode::decode_from_std_read(&mut reader, bincode::config::standard())
            .map_err(TachyonError::Deserialization)?;

        Ok(index)
    }
}
