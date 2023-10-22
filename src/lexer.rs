use crate::token::{Int, Token, TokenError};

pub fn lexer(_: &str) -> Result<Vec<Token>, TokenError> {
    unimplemented!()
}

fn count_int(s: &str) -> usize {
    count(s, |c: char| c.is_digit(10))
}

fn count_ident(s: &str) -> usize {
    count(s, |c: char| is_ident_char(c))
}

fn count(s: &str, pred: fn(char) -> bool) -> usize {
    fn go(n: usize, s: &str, pred: fn(char) -> bool) -> usize {
        match s.chars().next() {
            Some(c) if pred(c) => go(n + 1, &s[1..], pred),
            _ => n,
        }
    }
    go(0, s, pred)
}

// fn expect(s: &str, expect: &str) -> bool {
//     &s[..expect.len()] == expect
// }

fn is_ident_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_int_test() {
        let s1 = "123abc";
        let s2 = "12345";
        let s3 = "abcde";

        assert_eq!(3, count_int(s1));
        assert_eq!(5, count_int(s2));
        assert_eq!(0, count_int(s3));
    }

    #[test]
    fn count_ident_test() {
        let s1 = "abc 123";
        let s2 = "abc_de 123";
        let s3 = "12345";

        assert_eq!(3, count_ident(s1));
        assert_eq!(6, count_ident(s2));
        assert_eq!(0, count_ident(s3));
    }

    #[test]
    fn one_integer() {
        let query1 = "0";
        let query2 = "1";
        let query3 = "32";

        assert_eq!(Ok(vec![Token::Integer(0)]), lexer(query1));
        assert_eq!(Ok(vec![Token::Integer(1)]), lexer(query2));
        assert_eq!(Ok(vec![Token::Integer(32)]), lexer(query3));
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
