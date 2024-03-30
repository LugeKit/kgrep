use std::process;

use clap::Parser;

use kgrep::config::Config;

fn main() {
    let config = Config::parse();
    if let Err(e) = kgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    process::exit(0);
}
