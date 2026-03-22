// crates/tachyon-core/src/error.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TachyonError {
    #[error("I/O error at '{path}': {source}")]
    Io {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Path contains non-UTF-8 characters: {0:?}")]
    NonUtf8Path(std::ffi::OsString),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::error::EncodeError),

    #[error("Deserialization error: {0}")]
    Deserialization(#[from] bincode::error::DecodeError),

    #[error("Invalid index file: {0}")]
    InvalidIndexFile(String),

    #[error("Incompatible schema version: expected {expected}, found {found}")]
    IncompatibleSchemaVersion { expected: u16, found: u16 },
}
