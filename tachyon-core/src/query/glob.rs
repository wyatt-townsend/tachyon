use crate::{
    error::TachyonError,
    index::Index,
    query::{Query, QueryResult},
};

use globset::Glob;

pub struct GlobQuery {
    pattern: String,
    case_insensitive: bool,
}

impl GlobQuery {
    pub fn new(pattern: &String, case_insensitive: bool) -> Self {
        let search_pattern = if case_insensitive {
            pattern.to_lowercase()
        } else {
            pattern.to_string()
        };

        Self {
            pattern: search_pattern,
            case_insensitive: case_insensitive,
        }
    }
}

impl Query for GlobQuery {
    fn query(&self, index: &Index) -> Result<QueryResult, TachyonError> {
        let mut result = QueryResult::new();
        let matcher = Glob::new(&self.pattern)
            .map_err(|e| TachyonError::Query(format!("Invalid glob pattern: {}", e)))?
            .compile_matcher();

        for (k, v) in index.entries.iter() {
            let name = if self.case_insensitive {
                v.name.to_lowercase()
            } else {
                v.name.clone()
            };

            if matcher.is_match(&name) {
                result.push(k.clone());
            }
        }

        return Ok(result);
    }
}
