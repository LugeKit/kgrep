use crate::search::{Search, SearchParam, SearchResult};

pub struct PlainText {}

impl PlainText {
    pub fn new() -> PlainText {
        PlainText {}
    }
}

impl Search for PlainText {
    fn search(&self, param: SearchParam) -> Vec<SearchResult> {
        let actual_query = if param.ignore_case {
            param.query.to_lowercase()
        } else {
            String::from(param.query)
        };

        param.contents
            .into_iter()
            .enumerate()
            .filter_map(|(i, &line)| {
                let matches = if param.ignore_case {
                    indexes(&line.to_lowercase(), &actual_query)
                } else {
                    indexes(line, &actual_query)
                };

                if matches.len() == 0 {
                    None
                } else {
                    Some(SearchResult::new(i, matches))
                }
            })
            .collect()
    }
}

fn indexes(s: &str, pattern: &str) -> Vec<(usize, usize)> {
    s.match_indices(pattern)
        .map(|(i, _)| {
            (i, i + pattern.as_bytes().len())
        })
        .collect()
}