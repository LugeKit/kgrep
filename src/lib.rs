use std::{fs, io};
use std::error::Error;
use std::io::{Read, stdin};

use config::Config;

use crate::search::plaintext::PlainText;
use crate::search::Search;

pub mod config;
mod search;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match config.file_name {
        // FIXME: if not piped in, it will infinitely wait here
        None => { io::read_to_string(stdin())? }
        Some(file_name) => { fs::read_to_string(file_name)? }
    };
    let content_lines = contents.lines().collect();
    let searcher = PlainText::new(config.ignore_case);
    let line_indexes = searcher.search(&config.query, &content_lines);
    line_indexes.iter().for_each(|i| println!("{}", content_lines[*i]));
    Ok(())
}

fn search<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast and productive.\nPick three.";
        assert_eq!(vec!["safe, fast and productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "rust";
        let contents = "Rust:\nsafe, fast and productive.\nPick three.";
        assert_eq!(vec!["Rust:"], search_case_insensitive(query, contents));
    }
}