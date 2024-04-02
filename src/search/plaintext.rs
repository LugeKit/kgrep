use crate::search::Search;

pub struct PlainText<'a> {
    pattern: &'a str,
}

impl PlainText<'_> {
    pub fn new(pattern: &str) -> PlainText {
        PlainText { pattern }
    }
}

impl<'a> Search for PlainText<'a> {
    fn search(&self, line: &str) -> Option<Vec<(usize, usize)>> {
        let matches: Vec<_> = line.match_indices(self.pattern)
            .map(|(i, _)| {
                (i, i + self.pattern.as_bytes().len())
            })
            .collect();
        if matches.len() > 0 {
            Some(matches)
        } else {
            None
        }
    }
}