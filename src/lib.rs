use crate::lexer::lexer;
use parser::parser;
use std::{
    fs::{self, File},
    io::Write,
};
use token::TokenError::{InvalidChar, TokenizeError};
use variable::variable_analysis;

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

    let _trees = parser(tokens);

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

fn invalid_char_error(source: &str, c: char) -> String {
    let source_split: Vec<String> = source.split('\n').map(|s| s.to_string()).collect();
    let irregular_line = match source_split.clone().into_iter().find(|s| s.contains(c)) {
        Some(s) => s,
        None => String::from(""),
    };
    let irregular_line_num = source_split
        .into_iter()
        .position(|s| s.contains(c))
        .unwrap_or(0);
    let pos = irregular_line.find(c).unwrap_or(0);
    format!(
        "--> {}:{}\n{}\n{}^ invalid char",
        irregular_line_num,
        pos,
        irregular_line,
        " ".repeat(pos)
    )
}

#[cfg(test)]
mod tests {
    use super::invalid_char_error;

    #[test]
    fn invalid_char_error_test() {
        let s = "int main() {\n\tint a = 2;\n\tint b = 3;\n\treturn a * b:\n}";
        let c = ':';

        assert_eq!(
            "--> 3:13\n\treturn a * b:\n             ^ invalid char",
            invalid_char_error(s, c)
        );
    }
}
