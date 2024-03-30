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
        let mut result = vec![];
        let lowercase_query = query.to_lowercase();
        let mut actual_query = query;
        if self.ignore_case {
            actual_query = lowercase_query.as_str();
        }

        for i in (0..contents.len()) {
            let line = contents[i];
            if !self.ignore_case && line.contains(actual_query) {
                result.push(i);
                continue;
            }

            if self.ignore_case && line.to_lowercase().contains(actual_query) {
                result.push(i);
            }
        }
        result
    }
}