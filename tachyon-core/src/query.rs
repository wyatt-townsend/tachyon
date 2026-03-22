use crate::{error::TachyonError, index::Index};
use std::path::PathBuf;

pub mod glob;
pub mod substring;
pub type QueryResult = Vec<PathBuf>;

pub enum QueryType {
    Substring {
        pattern: String,
        case_insensitive: bool,
    },
    Glob {
        pattern: String,
        case_insensitive: bool,
    },
}

pub trait Query {
    fn query(&self, index: &Index) -> Result<QueryResult, TachyonError>;
}
