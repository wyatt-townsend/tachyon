use crate::{
    error::TachyonError,
    index::Index,
    query::{Query, QueryResult},
};

pub struct SubstringQuery {
    pattern: String,
    case_insensitive: bool,
}

impl SubstringQuery {
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

impl Query for SubstringQuery {
    fn query(&self, index: &Index) -> Result<QueryResult, TachyonError> {
        let mut result = QueryResult::new();

        for (k, v) in index.entries.iter() {
            let name = if self.case_insensitive {
                v.name.to_lowercase()
            } else {
                v.name.clone()
            };

            if name.contains(&self.pattern) {
                result.push(k.clone());
            }
        }

        return Ok(result);
    }
}
