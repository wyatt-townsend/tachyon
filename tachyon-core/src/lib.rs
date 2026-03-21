pub mod error;
pub mod index;
pub use index::{EntryKind, FileEntry};
pub use index::{build_index, load_index};

#[cfg(test)]
mod tests;
