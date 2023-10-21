use crate::token::Token;

pub fn lexer(input: &str) -> Vec<Token> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_integer() {
        let query1 = "0";
        let query2 = "1";
        let query3 = "32";

        assert_eq!(vec![Token::IntegerLiteral(0)], lexer(query1));
        assert_eq!(vec![Token::IntegerLiteral(1)], lexer(query2));
        assert_eq!(vec![Token::IntegerLiteral(32)], lexer(query3));
    }
}
