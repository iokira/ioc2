use crate::token::{Ident, Int, Token, TokenError};

pub fn lexer(s: &str) -> Result<Vec<Token>, TokenError> {
    let go = |tokenize: fn(&str) -> Result<(Token, usize), TokenError>| {
        if let Ok((token, size)) = tokenize(s) {
            match lexer(&s[size..]) {
                Ok(v) => Ok([vec![token], v].concat()),
                Err(e) => Err(e),
            }
        } else {
            Err(TokenError::TokenizeError)
        }
    };

    if s.is_empty() {
        Ok(vec![])
    } else if expect_whitespace(s) {
        lexer(&s[count_whitespace(s)..])
    } else if expect_int(s) {
        go(tokenize_int)
    } else if expect_ident(s) {
        go(tokenize_ident)
    } else if !expect_operators(s).is_empty() {
        go(tokenize_operator)
    } else if let Some(c) = s.chars().next() {
        Err(TokenError::InvalidChar(c))
    } else {
        Err(TokenError::TokenizeError)
    }
}

fn tokenize_int(s: &str) -> Result<(Token, usize), TokenError> {
    let num = &s[..count_int(s)];
    match num.parse::<Int>() {
        Ok(n) => Ok((Token::Integer(n), num.len())),
        Err(_) => Err(TokenError::TokenizeError),
    }
}

fn tokenize_ident(s: &str) -> Result<(Token, usize), TokenError> {
    let str = &s[..count_ident(s)];
    match str.len() {
        0 => Err(TokenError::TokenizeError),
        _ => Ok((
            Token::Ident(Ident {
                name: str.to_string(),
            }),
            str.len(),
        )),
    }
}

fn tokenize_operator(s: &str) -> Result<(Token, usize), TokenError> {
    match expect_operators(s) {
        ";" => Ok((Token::Semicolon, 1)),
        "==" => Ok((Token::Equality, 2)),
        "=" => Ok((Token::Equal, 1)),
        "!=" => Ok((Token::Noneequality, 2)),
        "<=" => Ok((Token::LessOrEqual, 2)),
        "<" => Ok((Token::Less, 1)),
        ">=" => Ok((Token::GreaterOrEqual, 2)),
        ">" => Ok((Token::Greater, 1)),
        "+" => Ok((Token::Add, 1)),
        "-" => Ok((Token::Sub, 1)),
        "*" => Ok((Token::Mul, 1)),
        "/" => Ok((Token::Div, 1)),
        "(" => Ok((Token::LParen, 1)),
        ")" => Ok((Token::RParen, 1)),
        _ => Err(TokenError::TokenizeError),
    }
}

fn count_int(s: &str) -> usize {
    count(s, |c| c.is_ascii_digit())
}

fn count_ident(s: &str) -> usize {
    count(s, is_ident_char)
}

fn count_whitespace(s: &str) -> usize {
    count(s, |c| c.is_whitespace())
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
    s.len() >= expect.len() && &s[..expect.len()] == expect
}

fn expect_int(s: &str) -> bool {
    count_int(s) > 0
}

fn expect_ident(s: &str) -> bool {
    count_ident(s) > 0
}

fn expect_operators(s: &str) -> &'static str {
    let ops = vec![
        ";", "==", "=", "!=", "<=", "<", ">=", ">", "+", "-", "*", "/", "(", ")",
    ];

    for op in ops {
        if expect_str(s, op) {
            return op;
        }
    }

    ""
}

fn expect_whitespace(s: &str) -> bool {
    count_whitespace(s) > 0
}

fn is_ident_char(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_int_test() {
        let s1 = "123";
        let s2 = "12 abc";
        let s3 = "abc 123";

        assert_eq!(Ok((Token::Integer(123), 3)), tokenize_int(s1));
        assert_eq!(Ok((Token::Integer(12), 2)), tokenize_int(s2));
        assert_eq!(Err(TokenError::TokenizeError), tokenize_int(s3));
    }

    #[test]
    fn tokenize_ident_test() {
        let s1 = "abc 123";
        let s2 = "abc_de 123";
        let s3 = "12345";

        assert_eq!(
            Ok((
                Token::Ident(Ident {
                    name: "abc".to_owned()
                }),
                3
            )),
            tokenize_ident(s1)
        );
        assert_eq!(
            Ok((
                Token::Ident(Ident {
                    name: "abc_de".to_owned()
                }),
                6
            )),
            tokenize_ident(s2)
        );
        assert_eq!(Err(TokenError::TokenizeError), tokenize_ident(s3));
    }

    #[test]
    fn tokenize_operator_test() {
        let s1 = "==abc";
        let s2 = "=abc";
        let s3 = "abc";

        assert_eq!(Ok((Token::Equality, 2)), tokenize_operator(s1));
        assert_eq!(Ok((Token::Equal, 1)), tokenize_operator(s2));
        assert_eq!(Err(TokenError::TokenizeError), tokenize_operator(s3));
    }

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
    fn count_whitespace_test() {
        let s1 = "   abc";
        let s2 = "abc";

        assert_eq!(3, count_whitespace(s1));
        assert_eq!(0, count_whitespace(s2));
    }

    #[test]
    fn expect_str_test() {
        let s1 = "abcde";
        let s2 = "a1 b2";

        assert!(expect_str(s1, "abc"));
        assert!(!expect_str(s2, "a1b2"));
    }

    #[test]
    fn expect_int_test() {
        let s1 = "123";
        let s2 = "abc";

        assert!(expect_int(s1));
        assert!(!expect_int(s2));
    }

    #[test]
    fn expect_ident_test() {
        let s1 = "abc";
        let s2 = "123";

        assert!(expect_ident(s1));
        assert!(!expect_ident(s2));
    }

    #[test]
    fn expect_operators_test() {
        let ops = vec![
            ";", "==", "=", "!=", "<=", "<", ">=", ">", "+", "-", "*", "/", "(", ")",
        ];

        for op in ops {
            assert_eq!(op, expect_operators(op));
        }
    }

    #[test]
    fn expect_whitespace_test() {
        let s1 = " abc";
        let s2 = "abc ";

        assert!(expect_whitespace(s1));
        assert!(!expect_whitespace(s2));
    }

    #[test]
    fn is_ident_char_test() {
        let c1 = 'a';
        let c2 = 'A';
        let c3 = '_';
        let c4 = '1';

        assert!(is_ident_char(c1));
        assert!(is_ident_char(c2));
        assert!(is_ident_char(c3));
        assert!(!is_ident_char(c4));
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

    #[test]
    fn add() {
        let query = "1 + 1";

        assert_eq!(
            Ok(vec![Token::Integer(1), Token::Add, Token::Integer(1)]),
            lexer(query)
        );
    }

    #[test]
    fn sub() {
        let query = " 2 - 1";

        assert_eq!(
            Ok(vec![Token::Integer(2), Token::Sub, Token::Integer(1),]),
            lexer(query)
        );
    }

    #[test]
    fn mul() {
        let query = "3 * 5";

        assert_eq!(
            Ok(vec![Token::Integer(3), Token::Mul, Token::Integer(5),]),
            lexer(query)
        );
    }

    #[test]
    fn div() {
        let query = "6 / 2";

        assert_eq!(
            Ok(vec![Token::Integer(6), Token::Div, Token::Integer(2)]),
            lexer(query)
        );
    }

    #[test]
    fn lexer_test() {
        let query1 = "1 + 10 - 123 * / == abc = d_ef != <= < >= > ();";
        let query2 = "abc$";

        assert_eq!(
            Ok(vec![
                Token::Integer(1),
                Token::Add,
                Token::Integer(10),
                Token::Sub,
                Token::Integer(123),
                Token::Mul,
                Token::Div,
                Token::Equality,
                Token::Ident(Ident {
                    name: "abc".to_owned()
                }),
                Token::Equal,
                Token::Ident(Ident {
                    name: "d_ef".to_owned()
                }),
                Token::Noneequality,
                Token::LessOrEqual,
                Token::Less,
                Token::GreaterOrEqual,
                Token::Greater,
                Token::LParen,
                Token::RParen,
                Token::Semicolon,
            ]),
            lexer(query1)
        );
        assert_eq!(Err(TokenError::InvalidChar('$')), lexer(query2));
    }
}
