use bincode::{Decode, Encode};

/// Represents a file or directory entry in the search results
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
}

/// Represents a file or directory entry in the search results
#[derive(Encode, Decode, Debug, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: std::path::PathBuf,
    pub kind: EntryKind,
}
