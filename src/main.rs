use std::{env, fs, process};
use std::error::Error;
use kgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(| err | {
        eprintln!("Problems in parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = kgrep::run(config) {
        eprintln!("Application error: {err}");
        process::exit(1);
    }
}
