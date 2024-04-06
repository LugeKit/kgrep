use std::error::Error;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    pub query: String,

    pub filename: Option<String>,

    #[arg(short = 'i', long = "ignore_case", default_value = "false")]
    pub ignore_case: bool,

    #[arg(short = 'r', long = "revert_match", default_value = "false")]
    pub revert_match: bool,

    // TODO: implement this
    #[arg(short = 'w', long = "word", default_value = "false")]
    pub word_match: bool,

    #[arg(short = 'E', long = "pattern", default_value = "false")]
    pub enable_regex: bool,

    #[arg(short = 'A', long = "after_context", default_value = "0")]
    pub after_context: usize,

    #[arg(short = 'B', long = "before_context", default_value = "0")]
    pub before_context: usize,

    #[arg(short = 'C', long = "context", default_value = "0")]
    pub context_count: usize,
}

impl Config {
    pub fn validate(&mut self) -> Result<(), Box<dyn Error>> {
        if self.context_count > 0 {
            self.before_context = self.context_count;
            self.after_context = self.context_count;
        }

        Ok(())
    }
}