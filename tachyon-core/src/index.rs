//! This module provides functionality for building and loading an index of a directory tree.
//! The index is a binary file that contains metadata about the files and directories in the
//! tree, such as their names, paths, and types (file or directory).
//! The `build_index` function recursively walks through the directory tree and collects this metadata,
//! while the `load_index` function reads the binary index file and reconstructs the metadata in memory.

pub mod builder;
pub mod entry;
pub use builder::{build_index, load_index};
pub use entry::{EntryKind, FileEntry};
