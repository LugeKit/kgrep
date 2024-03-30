use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    pub query: String,

    #[arg(short = 'f', long = "file")]
    pub file_name: Option<String>,

    #[arg(short = 'i', long = "ignore_case", default_value = "false")]
    pub ignore_case: bool,

    #[arg(short = 'e', long = "pattern", default_value = "false")]
    pub enable_regex: bool,

    #[arg(short = 'A', long = "after", default_value = "0")]
    pub after_count: u32,

    #[arg(short = 'B', long = "before", default_value = "0")]
    pub before_count: u32,
}
