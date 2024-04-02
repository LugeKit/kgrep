use std::error::Error;

use regex::Regex;

use crate::search::{Search, SearchParam, SearchResult};

pub struct KRegex {}

impl KRegex {
    pub fn new() -> KRegex {
        KRegex {}
    }
}

impl Search for KRegex {
    fn search(&self, param: SearchParam) -> Result<Vec<SearchResult>, Box<dyn Error>> {
        let re = Regex::new(param.query)?;

        param.contents.iter()
            .enumerate()
            .filter_map(|(i, &line)| {
                let highlights: Vec<_> = if param.ignore_case {
                    re.find_iter(&line.to_lowercase())
                        .map(|m| {
                            (m.start(), m.end())
                        })
                        .collect()
                } else {
                    re.find_iter(line)
                        .map(|m| {
                            (m.start(), m.end())
                        })
                        .collect()
                };

                if highlights.len() > 0 {
                    Some(Ok(SearchResult::new(i, highlights)))
                } else {
                    None
                }
            })
            .collect()
    }
}