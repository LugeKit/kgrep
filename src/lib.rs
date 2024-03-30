use std::error::Error;
use std::fs;

use config::Config;

pub mod config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

    if !config.ignore_case {
        for line in search(&config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search_case_insensitive(&config.query, &contents) {
            println!("{line}");
        }
    }

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