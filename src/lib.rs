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
    line_indexes.into_iter()
        .map(|i|
            (if i > config.before_count { i - config.before_count } else { 0 }, content_lines.len().min(i + config.after_count + 1))
        )
        .for_each(|(start, end)| {
            content_lines[start..end].into_iter()
                .for_each(|line| { println!("{}", line) })
        });
    Ok(())
}