use std::{fs, io};
use std::collections::HashMap;
use std::error::Error;
use std::io::{IsTerminal, stdin};

use colored::{ColoredString, Colorize};

use config::Config;
use search::{SearchParam, SearchResult};

use crate::search::new_searcher;

pub mod config;
mod search;
mod model;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = match config.file_name {
        None => {
            if stdin().is_terminal() {
                return Err("file name is empty and no input".into());
            }
            io::read_to_string(stdin())?
        }
        Some(ref file_name) => { fs::read_to_string(file_name)? }
    };
    let content_lines: Vec<_> = contents.lines().collect();

    let searcher = new_searcher(config.enable_regex);
    let search_result = searcher.search(SearchParam::new(&config.query, &content_lines, config.ignore_case))?;

    display_results(&config, &content_lines, &search_result);

    Ok(())
}

fn display_results(config: &Config, content_lines: &Vec<&str>, results: &Vec<SearchResult>) {
    if results.len() == 0 {
        return;
    }

    let mut highlights_map = HashMap::new();
    for result in results {
        highlights_map.insert(result.line_index, &result.highlights);
    }

    let build_range = |result: &SearchResult| -> (usize, usize) {
        let start_index = if result.line_index > config.before_count {
            result.line_index - config.before_count
        } else {
            0
        };

        let end_index = if result.line_index + config.after_count < content_lines.len() {
            result.line_index + config.after_count + 1
        } else {
            content_lines.len()
        };

        (start_index, end_index)
    };

    let mut j = 0;
    let mut range = build_range(&results[j]);
    'outer: for i in 0..content_lines.len() {
        while i >= range.1 {
            j += 1;
            if j >= results.len() {
                break 'outer;
            }

            range = build_range(&results[j]);
        }

        if i < range.0 {
            continue;
        }

        if let Some(&highlights) = highlights_map.get(&i) {
            display_highlights(content_lines[i], highlights);
        } else {
            println!("{}", content_lines[i]);
        }
        continue;
    }
}

fn display_highlights(s: &str, highlights: &Vec<(usize, usize)>) {
    let mut line: Vec<ColoredString> = vec![];
    let mut i = 0;
    for &(start, end) in highlights {
        let _s = &s[i..start];
        line.push(_s.normal());

        let _s = &s[start..end];
        line.push(_s.on_white().bold());

        i = end;
    }

    if i < s.len() {
        line.push(s[i..].normal())
    }

    for part in line {
        print!("{}", part);
    }
    print!("\n");
}
