use crate::token::{Ident, Int, Token, TokenError};

pub fn lexer(_: &str) -> Result<Vec<Token>, TokenError> {
    unimplemented!()
}

fn tokenize_int(s: &str) -> Result<(Token, usize), TokenError> {
    let num = &s[..count_int(&s)];
    match num.parse::<Int>() {
        Ok(n) => Ok((Token::Integer(n), num.len())),
        Err(_) => Err(TokenError),
    }
}

fn tokenize_ident(s: &str) -> Result<(Token, usize), TokenError> {
    let str = &s[..count_ident(&s)];
    match str.len() {
        0 => Err(TokenError),
        _ => Ok((
            Token::Ident(Ident {
                name: str.to_string(),
            }),
            str.len(),
        )),
    }
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

fn expect_str(s: &str, expect: &str) -> bool {
    &s[..expect.len()] == expect
}

fn expect_int(s: &str) -> bool {
    count_int(s) > 0
}

fn expect_ident(s: &str) -> bool {
    count_ident(s) > 0
}

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
    fn tokenize_int_test() {
        let s1 = "123";
        let s2 = "12 abc";
        let s3 = "abc 123";

        assert_eq!(Ok((Token::Integer(123), 3)), tokenize_int(s1));
        assert_eq!(Ok((Token::Integer(12), 2)), tokenize_int(s2));
        assert_eq!(Err(TokenError), tokenize_int(s3));
    }

    #[test]
    fn tokenize_ident_test() {
        let s1 = "abc 123";
        let s2 = "abc_de 123";
        let s3 = "12345";

        assert_eq!(
            Ok((
                Token::Ident(Ident {
                    name: "abc".to_string()
                }),
                3
            )),
            tokenize_ident(s1)
        );
        assert_eq!(
            Ok((
                Token::Ident(Ident {
                    name: "abc_de".to_string()
                }),
                6
            )),
            tokenize_ident(s2)
        );
        assert_eq!(Err(TokenError), tokenize_ident(s3));
    }

    #[test]
    fn expect_str_test() {
        let s1 = "abcde";
        let s2 = "a1 b2";

        assert_eq!(true, expect_str(s1, "abc"));
        assert_eq!(false, expect_str(s2, "a1b2"));
    }

    #[test]
    fn expect_int_test() {
        let s1 = "123";
        let s2 = "abc";

        assert_eq!(true, expect_int(s1));
        assert_eq!(false, expect_int(s2));
    }

    #[test]
    fn expect_ident_test() {
        let s1 = "abc";
        let s2 = "123";

        assert_eq!(true, expect_ident(s1));
        assert_eq!(false, expect_ident(s2));
    }

    // #[test]
    // fn one_integer() {
    //     let query1 = "0";
    //     let query2 = "1";
    //     let query3 = "32";
    //
    //     assert_eq!(Ok(vec![Token::Integer(0)]), lexer(query1));
    //     assert_eq!(Ok(vec![Token::Integer(1)]), lexer(query2));
    //     assert_eq!(Ok(vec![Token::Integer(32)]), lexer(query3));
    // }

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
