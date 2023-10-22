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

        assert_eq!(vec![Token::Integer(0)], lexer(query1));
        assert_eq!(vec![Token::Integer(1)], lexer(query2));
        assert_eq!(vec![Token::Integer(32)], lexer(query3));
    }

    // #[test]
    // fn add() {
    //     let query = "1 + 1";
    //
    //     assert_eq!(
    //         vec![
    //             Token::IntegerLiteral(1),
    //             Token::Add,
    //             Token::IntegerLiteral(1)
    //         ],
    //         lexer(query)
    //     );
    // }

    // #[test]
    // fn sub() {
    //     let query = " 2 - 1";
    //
    //     assert_eq!(
    //         vec![
    //             Token::IntegerLiteral(2),
    //             Token::Sub,
    //             Token::IntegerLiteral(1),
    //         ],
    //         lexer(query)
    //     );
    // }

    // #[test]
    // fn mul() {
    //     let query = "3 * 5";
    //
    //     assert_eq!(
    //         vec![
    //             Token::IntegerLiteral(3),
    //             Token::Mul,
    //             Token::IntegerLiteral(5),
    //         ],
    //         lexer(query)
    //     );
    // }

    // #[test]
    // fn div() {
    //     let query = "6 / 2";
    //
    //     assert_eq!(
    //         vec![
    //             Token::IntegerLiteral(6),
    //             Token::Div,
    //             Token::IntegerLiteral(2)
    //         ],
    //         lexer(query)
    //     );
    // }
}
