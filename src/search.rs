pub mod plaintext;

pub(crate) trait Search {
    fn search(&self, param: SearchParam) -> Vec<SearchResult>;
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

pub(crate) struct SearchResult {
    pub line_index: usize,
    pub highlights: Vec<(usize, usize)>,
}

impl SearchResult {
    fn new(line_index: usize, highlights: Vec<(usize, usize)>) -> SearchResult {
        SearchResult { line_index, highlights }
    }
}