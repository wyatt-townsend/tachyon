use crate::index::Index;
use crate::query::glob::GlobQuery;
use crate::query::substring::SubstringQuery;
use crate::query::{Query, QueryResult, QueryType};
use crate::{error::TachyonError, index::builder::IndexBuilder};
use std::path::Path;

pub mod error;
pub mod index;
pub mod query;

// Build
pub fn build_index(drive: &Path) -> Result<Index, TachyonError> {
    let mut builder = IndexBuilder::new(drive);
    builder.build()
}

// Query
pub fn query_index(index: &Index, query_type: QueryType) -> Result<QueryResult, TachyonError> {
    let query: Box<dyn Query> = match query_type {
        QueryType::Substring {
            pattern,
            case_insensitive,
        } => Box::new(SubstringQuery::new(&pattern, case_insensitive)),
        QueryType::Glob {
            pattern,
            case_insensitive,
        } => Box::new(GlobQuery::new(&pattern, case_insensitive)),
    };

    query.query(index)
}
