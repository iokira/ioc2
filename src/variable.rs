use crate::token::*;

pub fn variable_analysis(tokens: Vec<Token>) -> Vec<Token> {
    tokens
}

fn extract_variable(tokens: Vec<Token>) -> Vec<Ident> {
    if tokens.is_empty() {
        return vec![];
    } else if let Token::Ident(ident) = tokens[0].clone() {
        [vec![ident], extract_variable(tokens[1..].to_vec())].concat()
    } else {
        extract_variable(tokens[1..].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_analysis_test() {
        let query = vec![
            Token::Variable(0),
            Token::Ident(Ident {
                name: "a".to_string(),
            }),
            Token::Add,
        ];

        assert_eq!(
            vec![Token::Integer(0), Token::Variable(8), Token::Add],
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
}
