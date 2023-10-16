use std::{env, process};

use ioc2::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = ioc2::run(input) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
