use std::error::Error;

use regex::Regex;

use crate::search::Search;

pub struct KRegex {
    re: Regex,
}

impl KRegex {
    pub fn new(pattern: &str) -> Result<KRegex, Box<dyn Error>> {
        Ok(KRegex { re: Regex::new(pattern)? })
    }
}

impl Search for KRegex {
    fn search(&self, line: &str) -> Option<Vec<(usize, usize)>> {
        let highlights: Vec<_> = self.re.find_iter(line)
            .map(|m| {
                (m.start(), m.end())
            })
            .collect();

        if highlights.len() > 0 {
            Some(highlights)
        } else {
            None
        }
    }
}