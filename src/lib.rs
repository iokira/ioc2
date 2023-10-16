use std::{
    fs::File,
    io::{Read, Write},
};

pub struct Config {
    source: String,
    assembly: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let input_file_path = args[1].clone();
        let output_file_path = args[2].clone();

        Ok(Config {
            source: input_file_path,
            assembly: output_file_path,
        })
    }
}

pub fn run(input: Config) -> Result<(), String> {
    let mut input_file = match File::open(input.source) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };

    let mut output_file = match File::create(input.assembly) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };

    let mut contents = String::new();
    match input_file.read_to_string(&mut contents) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };

    match write!(output_file, "{}", contents) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };
    match output_file.flush() {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };

    Ok(())
}
