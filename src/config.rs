use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    pub query: String,

    #[arg(short = 'f', long = "file")]
    pub file_name: String,

    #[arg(short = 'i', long = "ignore_case", default_value = "false")]
    pub ignore_case: bool,

    #[arg(short = 'e', long = "pattern", default_value = "false")]
    pub enable_regex: bool,
}
