use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    pub query: String,

    pub file_name: Option<String>,

    #[arg(short = 'i', long = "ignore_case", default_value = "false")]
    pub ignore_case: bool,

    // TODO: implement this
    #[arg(short = 'r', long = "revert_match", default_value = "false")]
    pub revert_match: bool,

    // TODO: implement this
    #[arg(short = 'w', long = "word", default_value = "false")]
    pub word_match: bool,

    #[arg(short = 'E', long = "pattern", default_value = "false")]
    pub enable_regex: bool,

    #[arg(short = 'A', long = "after", default_value = "0")]
    pub after_count: usize,

    #[arg(short = 'B', long = "before", default_value = "0")]
    pub before_count: usize,
}
