use crate::index::Index;
use crate::{error::TachyonError, index::builder::IndexBuilder};
use std::path::Path;

pub mod error;
mod index;

// Build
pub fn build_index(drive: &Path) -> Result<Index, TachyonError> {
    let mut builder = IndexBuilder::new(drive);
    builder.build()
}
