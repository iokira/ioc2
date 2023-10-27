use std::collections::HashSet;

use crate::token::*;

pub fn variable_analysis(tokens: Vec<Token>) -> Vec<Token> {
    tokens
}

fn extract_ident(tokens: Vec<Token>) -> Vec<Ident> {
    if tokens.is_empty() {
        vec![]
    } else if let Token::Ident(ident) = tokens[0].clone() {
        [vec![ident], extract_ident(tokens[1..].to_vec())].concat()
    } else {
        extract_ident(tokens[1..].to_vec())
    }
}

fn deduplicate_variable(idents: Vec<Ident>) -> Vec<Ident> {
    HashSet::<_>::from_iter(idents)
        .into_iter()
        .collect::<Vec<Ident>>()
}

fn calc_offset(ident: Ident, idents: &Vec<Ident>) -> Option<usize> {
    idents
        .clone()
        .into_iter()
        .position(|i| i == ident)
        .map(|n| (n + 1) * 8)
}

fn ident2var(token: Token, idents: &Vec<Ident>) -> Result<Token, &'static str> {
    match token {
        Token::Ident(i) => match calc_offset(i, idents) {
            Some(n) => Ok(Token::Variable { offset: n }),
            None => Err("unexpected ident"),
        },
        _ => Ok(token),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_analysis_test() {
        let query = vec![
            Token::Integer(0),
            Token::Ident(Ident {
                name: "a".to_string(),
            }),
            Token::Add,
        ];

        assert_eq!(
            vec![Token::Integer(0), Token::Variable { offset: 8 }, Token::Add],
            variable_analysis(query)
        );
    }

    #[test]
    fn extract_variable_test() {
        let query = vec![
            Token::Integer(0),
            Token::Ident(Ident {
                name: "a".to_string(),
            }),
            Token::Add,
        ];

        assert_eq!(
            vec![Ident {
                name: "a".to_string()
            }],
            extract_ident(query)
        );
    }

    #[test]
    fn deduplicate_variable_test() {
        let query = vec![
            Ident {
                name: "a".to_string(),
            },
            Ident {
                name: "b".to_string(),
            },
            Ident {
                name: "b".to_string(),
            },
            Ident {
                name: "c".to_string(),
            },
            Ident {
                name: "c".to_string(),
            },
        ];

        assert_eq!(
            [
                Ident {
                    name: "a".to_string()
                },
                Ident {
                    name: "b".to_string()
                },
                Ident {
                    name: "c".to_string()
                }
            ]
            .sort(),
            deduplicate_variable(query).sort()
        );
    }

    #[test]
    fn calc_offset_test() {
        let query = vec![
            Ident {
                name: "a".to_string(),
            },
            Ident {
                name: "b".to_string(),
            },
            Ident {
                name: "c".to_string(),
            },
        ];

        assert_eq!(
            Some(8),
            calc_offset(
                Ident {
                    name: "a".to_string()
                },
                &query
            )
        );
        assert_eq!(
            Some(16),
            calc_offset(
                Ident {
                    name: "b".to_string()
                },
                &query
            )
        );
        assert_eq!(
            Some(24),
            calc_offset(
                Ident {
                    name: "c".to_string()
                },
                &query
            )
        );
        assert_eq!(
            None,
            calc_offset(
                Ident {
                    name: "d".to_string()
                },
                &query
            )
        );
    }

    #[test]
    fn ident2var_test() {
        let ident0 = Ident {
            name: "a".to_string(),
        };
        let ident1 = Ident {
            name: "b".to_string(),
        };
        let ident2 = Ident {
            name: "c".to_string(),
        };
        let ident3 = Ident {
            name: "d".to_string(),
        };
        let idents = vec![ident0.clone(), ident1.clone(), ident2.clone()];

        assert_eq!(
            Ok(Token::Variable { offset: 8 }),
            ident2var(Token::Ident(ident0), &idents)
        );
        assert_eq!(
            Ok(Token::Variable { offset: 16 }),
            ident2var(Token::Ident(ident1), &idents)
        );
        assert_eq!(
            Ok(Token::Variable { offset: 24 }),
            ident2var(Token::Ident(ident2), &idents)
        );
        assert_eq!(
            Err("unexpected ident"),
            ident2var(Token::Ident(ident3), &idents)
        );
    }
}
