use std::collections::VecDeque;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, IsTerminal, Read, stdin};

use colored::{ColoredString, Colorize};

use config::Config;

use crate::search::Search;

pub mod config;
mod search;
mod model;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if let None = config.filename {
        if stdin().is_terminal() {
            return Err("file name is empty and no input".into());
        }
    }

    let searcher = search::new_searcher(config.enable_regex, &config.query)?;
    match config.filename {
        None => {
            process(&config, stdin(), searcher);
        }
        Some(ref filename) if filename.ne("*") => {
            process(&config, File::open(filename)?, searcher);
        }
        _ => {}
    }

    Ok(())
}

fn process<'a, T: Read>(config: &Config, reader: T, searcher: Box<dyn Search + 'a>) {
    let mut process_helper = process_fn(config, searcher);

    BufReader::new(reader)
        .lines()
        .for_each(|s| {
            match s {
                Ok(line) => {
                    process_helper(line);
                }
                Err(_) => { return; }
            }
        });
}

fn process_fn<'a>(config: &'a Config, searcher: Box<dyn Search + 'a>) -> Box<dyn FnMut(String) + 'a> {
    match config.revert_match {
        true => {
            Box::new(move |line| {
                let results = if config.ignore_case {
                    searcher.search(&line.to_lowercase())
                } else {
                    searcher.search(&line)
                };
                match results {
                    None => { println!("{}", line); }
                    Some(_) => {}
                }
            })
        }
        false => {
            let mut before_lines = VecDeque::new();
            let mut after_count = 0;
            Box::new(move |line| {
                let results = if config.ignore_case {
                    searcher.search(&line.to_lowercase())
                } else {
                    searcher.search(&line)
                };

                match results {
                    None => {
                        if after_count > 0 {
                            after_count -= 1;
                            println!("{}", line);
                            return;
                        }

                        before_lines.push_back(line);
                        if before_lines.len() > config.before_count {
                            before_lines.pop_front();
                        }
                    }
                    Some(highlights) => {
                        before_lines.iter().for_each(|line| println!("{}", line));
                        before_lines.clear();
                        display_highlights(&line, &highlights);
                        after_count = config.after_count;
                    }
                }
            })
        }
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
