use std::collections::HashSet;

use crate::token::*;

pub fn variable_analysis(tokens: Vec<Token>) -> Vec<Token> {
    tokens
}

fn extract_variable(tokens: Vec<Token>) -> Vec<Ident> {
    if tokens.is_empty() {
        vec![]
    } else if let Token::Ident(ident) = tokens[0].clone() {
        [vec![ident], extract_variable(tokens[1..].to_vec())].concat()
    } else {
        extract_variable(tokens[1..].to_vec())
    }
}

fn deduplicate_variable(idents: Vec<Ident>) -> Vec<Ident> {
    HashSet::<_>::from_iter(idents.into_iter())
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
            extract_variable(query)
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
                },
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
}
