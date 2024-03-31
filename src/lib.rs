use std::{fs, io};
use std::error::Error;
use std::io::{IsTerminal, Read, stdin};

use config::Config;

use crate::search::plaintext::PlainText;
use crate::search::Search;

pub mod config;
mod search;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match config.file_name {
        // FIXME: if not piped in, it will infinitely wait here
        None => {
            if stdin().is_terminal() {
                return Err("file name is empty and no input".into());
            }
            io::read_to_string(stdin())?
        }
        Some(ref file_name) => { fs::read_to_string(file_name)? }
    };
    let content_lines = contents.lines().collect();
    let searcher = PlainText::new(config.ignore_case);
    let line_indexes = searcher.search(&config.query, &content_lines);
    display_result(&config, &content_lines, &line_indexes)
        .iter()
        .for_each(|&(start, end)| {
            content_lines[start..end].iter().for_each(|&line| println!("{line}"));
        });

    Ok(())
}

fn display_result(config: &Config, contents: &Vec<&str>, indexes: &Vec<usize>) -> Vec<(usize, usize)> {
    let ranges: Vec<(usize, usize)> = indexes.iter()
        .map(|i|
            (if *i > config.before_count { *i - config.before_count } else { 0 }, contents.len().min(i + config.after_count + 1))
        )
        .collect();

    return ranges.iter()
        .enumerate()
        .filter_map(|(i, (start, end))| {
            if i > 0 && ranges[i - 1].1 >= *start {
                return None;
            }

            let mut j = i + 1;
            let mut new_end = *end;
            while j < ranges.len() {
                if new_end < ranges[j].0 {
                    break;
                }
                new_end = new_end.max(ranges[j].1);
                j += 1;
            }

            return Some((*start, new_end));
        })
        .collect();
}