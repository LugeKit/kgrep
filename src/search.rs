use std::error::Error;

use crate::search::kregex::KRegex;
use crate::search::plaintext::PlainText;

mod plaintext;
mod kregex;

pub(crate) trait Search {
    fn search(&self, param: SearchParam) -> Result<Vec<SearchResult>, Box<dyn Error>>;
}

pub(crate) struct SearchParam<'a> {
    ignore_case: bool,
    query: &'a str,
    contents: &'a Vec<&'a str>,
}

impl<'a> SearchParam<'a> {
    pub fn new(query: &'a str, contents: &'a Vec<&'a str>, ignore_case: bool) -> SearchParam<'a> {
        SearchParam { query, contents, ignore_case }
    }
}

#[derive(Debug)]
pub(crate) struct SearchResult {
    pub line_index: usize,
    pub highlights: Vec<(usize, usize)>,
}

impl SearchResult {
    fn new(line_index: usize, highlights: Vec<(usize, usize)>) -> SearchResult {
        SearchResult { line_index, highlights }
    }
}

pub fn new_searcher(enable_regex: bool) -> Box<dyn Search> {
    if enable_regex {
        Box::new(KRegex::new())
    } else {
        Box::new(PlainText::new())
    }
}