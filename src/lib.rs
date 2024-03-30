use std::{env, fs};
use std::error::Error;

pub struct Config {
    query: String,
    file_name: String,
    ignore_case: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        Ok(Config {
            query: args[1].clone(),
            file_name: args[2].clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;
    for line in search(&config.query, &contents) {
        println!("{line}");
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