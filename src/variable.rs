use std::collections::HashSet;

use crate::token::*;

pub fn variable_analysis(tokens: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    convert_tokens(tokens)
}

fn extract_ident(tokens: &Vec<Token>) -> Vec<Ident> {
    if tokens.is_empty() {
        vec![]
    } else if let Token::Ident(ident) = tokens[0].clone() {
        [vec![ident], extract_ident(&tokens[1..].to_vec())].concat()
    } else {
        extract_ident(&tokens[1..].to_vec())
    }
}

fn deduplicate_variable(idents: Vec<Ident>) -> Vec<Ident> {
    let mut idents = HashSet::<_>::from_iter(idents)
        .into_iter()
        .collect::<Vec<Ident>>();
    idents.sort();
    idents
}

fn calc_offset(ident: Ident, idents: &[Ident]) -> Option<usize> {
    idents
        .to_owned()
        .clone()
        .into_iter()
        .position(|i| i == ident)
        .map(|n| (n + 1) * 8)
}

fn ident2var(token: Token, idents: &[Ident]) -> Result<Token, &'static str> {
    match token {
        Token::Ident(i) => match calc_offset(i, idents) {
            Some(n) => Ok(Token::Variable { offset: n }),
            None => Err("unexpected ident"),
        },
        _ => Ok(token),
    }
}

fn convert_tokens(tokens: Vec<Token>) -> Result<Vec<Token>, &'static str> {
    let idents = deduplicate_variable(extract_ident(&tokens));
    fn go(tokens: Vec<Token>, idents: Vec<Ident>) -> Result<Vec<Token>, &'static str> {
        if tokens.is_empty() {
            return Ok(vec![]);
        }
        match ident2var(tokens[0].clone(), &idents) {
            Ok(t) => match go(tokens[1..].to_vec(), idents) {
                Ok(ts) => Ok([vec![t], ts].concat()),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }
    go(tokens, idents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_analysis_test() {
        let query1 = vec![
            Token::Integer(0),
            Token::Ident(Ident {
                name: "a".to_string(),
            }),
            Token::Add,
        ];

        assert_eq!(
            Ok(vec![
                Token::Integer(0),
                Token::Variable { offset: 8 },
                Token::Add
            ]),
            variable_analysis(query1)
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
            extract_ident(&query)
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
            vec![
                Ident {
                    name: "a".to_string()
                },
                Ident {
                    name: "b".to_string()
                },
                Ident {
                    name: "c".to_string()
                }
            ],
            deduplicate_variable(query)
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

    #[test]
    fn convert_tokens_test() {
        let query = vec![
            Token::Integer(0),
            Token::Ident(Ident {
                name: "a".to_string(),
            }),
            Token::Ident(Ident {
                name: "b".to_string(),
            }),
            Token::Add,
            Token::Ident(Ident {
                name: "b".to_string(),
            }),
            Token::Ident(Ident {
                name: "c".to_string(),
            }),
            Token::Ident(Ident {
                name: "c".to_string(),
            }),
            Token::Add,
        ];

        assert_eq!(
            Ok(vec![
                Token::Integer(0),
                Token::Variable { offset: 8 },
                Token::Variable { offset: 16 },
                Token::Add,
                Token::Variable { offset: 16 },
                Token::Variable { offset: 24 },
                Token::Variable { offset: 24 },
                Token::Add,
            ]),
            convert_tokens(query)
        );
    }
}
