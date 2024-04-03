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
