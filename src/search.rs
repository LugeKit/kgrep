pub mod plaintext;

pub trait Search {
    fn search(&self, query: &str, contents: &Vec<&str>) -> Vec<usize>;
}