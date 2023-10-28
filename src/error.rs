pub fn invalid_char_error(source: &str, c: char) -> String {
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
