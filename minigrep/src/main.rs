use minigrep::{run, Config};
use std::error::Error;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("App error: {}", e);

        process::exit(1);
    }
}
