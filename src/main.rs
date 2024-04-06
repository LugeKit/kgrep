use std::process;

use clap::Parser;

use kgrep::config::Config;

fn main() {
    let mut config = Config::parse();

    if let Err(e) = config.validate() {
        eprintln!("Parameter error: {e}");
        process::exit(1);
    }

    if let Err(e) = kgrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

    process::exit(0);
}
