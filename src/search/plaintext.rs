use crate::search::Search;

pub struct PlainText {
    ignore_case: bool,
}

impl PlainText {
    pub fn new(ignore_case: bool) -> PlainText {
        PlainText { ignore_case }
    }
}

impl Search for PlainText {
    fn search(&self, query: &str, contents: &Vec<&str>) -> Vec<usize> {
        contents.iter()
            .enumerate()
            .filter_map(|(i, line)| {
                if self.ignore_case && line.to_lowercase().contains(query) {
                    Some(i)
                } else if !self.ignore_case && line.contains(query) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect()
    }
}