use std::error::Error;

use crate::search::kregex::KRegex;
use crate::search::plaintext::PlainText;

mod plaintext;
mod kregex;

pub(crate) trait Search {
    fn search(&self, line: &str) -> Option<Vec<(usize, usize)>>;
}

pub fn new_searcher<'a>(enable_regex: bool, query: &'a str) -> Result<Box<dyn Search + 'a>, Box<dyn Error>> {
    if enable_regex {
        Ok(Box::new(KRegex::new(query)?))
    } else {
        Ok(Box::new(PlainText::new(query)))
    }
}

pub fn execute_search<'a>(lines: &Vec<&str>, param: &SearchParam, searcher: Box<dyn Search + 'a>) -> Vec<SearchResult> {
    lines.iter()
        .enumerate()
        .filter_map(|(i, &s)| {
            if param.ignore_case {
                filter_out_not_matched(i, s.to_lowercase().as_str(), &searcher)
            } else {
                filter_out_not_matched(i, s, &searcher)
            }
        })
        .collect()
}

fn filter_out_not_matched<'a>(i: usize, line: &str, searcher: &Box<dyn Search + 'a>) -> Option<SearchResult> {
    match searcher.search(line) {
        None => { None }
        Some(highlights) => { Some(SearchResult::new(i, highlights)) }
    }
}

pub(crate) struct SearchParam {
    ignore_case: bool,
    word_match: bool,
}

impl SearchParam {
    pub(crate) fn new(ignore_case: bool, word_match: bool) -> SearchParam {
        SearchParam { ignore_case, word_match }
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
