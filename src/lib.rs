use error::invalid_char_error;
use generator::generator;
use lexer::lexer;
use parser::parser;
use std::{
    fs::{self, File},
    io::Write,
};
use token::TokenError::{InvalidChar, TokenizeError};
use variable::variable_analysis;

mod error;
mod generator;
mod lexer;
mod parser;
mod token;
mod tree;
mod variable;

pub struct Config {
    source_file_path: String,
    assembly_file_path: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let input_file_path = args[1].clone();
        let output_file_path = args[2].clone();

        Ok(Config {
            source_file_path: input_file_path,
            assembly_file_path: output_file_path,
        })
    }
}

pub fn run(input: Config) -> Result<(), String> {
    let contents = match fs::read_to_string(input.source_file_path) {
        Ok(it) => it,
        Err(err) => return Err(err.to_string()),
    };

    let tokens = match lexer(&contents) {
        Ok(tokens) => tokens,
        Err(e) => {
            return match e {
                TokenizeError => Err("tokenize error".to_string()),
                InvalidChar(c) => Err(format!(
                    "tokenize error\n{}",
                    invalid_char_error(&contents, c)
                )),
            }
        }
    };

    let tokens = match variable_analysis(tokens) {
        Ok(tokens) => tokens,
        Err(e) => return Err(e.to_string()),
    };

    let trees = match parser(tokens) {
        Ok(trees) => trees,
        Err(e) => return Err("parse error".to_string()),
    };

    let asm = match generator(trees) {
        Ok(asm) => asm,
        Err(_) => return Err("compile error".to_string()),
    };

    let mut output_file = match File::create(input.assembly_file_path) {
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
