use crate::{token::Token, tree::*};

pub fn parser(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    program(tokens)
}

fn program(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    if tokens.is_empty() {
        Ok(vec![])
    } else {
        let (tree, tokens) = match stmt(tokens) {
            Ok((tree, tokens)) => (tree, tokens),
            Err(e) => return Err(e),
        };
        let trees = match program(tokens) {
            Ok(trees) => trees,
            Err(e) => return Err(e),
        };
        Ok([vec![tree], trees].concat())
    }
}

fn stmt(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (tree, tokens) = match expr(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Err(TreeError::ParseError(
            "expected semicolon but disappear".to_string(),
        ))
    } else {
        match tokens[0] {
            Token::Semicolon => Ok((tree, tokens[1..].to_vec())),
            _ => Err(TreeError::ParseError(
                "expected semicolon but disappear".to_string(),
            )),
        }
    }
}

fn expr(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    match assign(tokens) {
        Ok((tree, tokens)) => Ok((tree, tokens)),
        Err(e) => Err(e),
    }
}

fn assign(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (tree, tokens) = match equality(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        match tokens[0] {
            Token::Equal => match assign(tokens[1..].to_vec()) {
                Ok((assign, tokens)) => {
                    Ok((Tree::new_tree(NodeKind::Assign, tree, assign), tokens))
                }
                Err(e) => Err(e),
            },
            _ => Ok((tree, tokens)),
        }
    }
}

fn equality(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (tree, tokens) = match relational(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        match tokens[0] {
            Token::Equality => match relational(tokens[1..].to_vec()) {
                Ok((relational, tokens)) => {
                    Ok((Tree::new_tree(NodeKind::Equality, tree, relational), tokens))
                }
                Err(e) => Err(e),
            },
            Token::Noneequality => match relational(tokens[1..].to_vec()) {
                Ok((relational, tokens)) => Ok((
                    Tree::new_tree(NodeKind::Nonequality, tree, relational),
                    tokens,
                )),
                Err(e) => Err(e),
            },
            _ => Ok((tree, tokens)),
        }
    }
}

fn relational(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (tree, tokens) = match add(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        match tokens[0] {
            Token::LessOrEqual => match add(tokens[1..].to_vec()) {
                Ok((add, tokens)) => Ok((Tree::new_tree(NodeKind::LessOrEqual, tree, add), tokens)),
                Err(e) => Err(e),
            },
            Token::Less => match add(tokens[1..].to_vec()) {
                Ok((add, tokens)) => Ok((Tree::new_tree(NodeKind::Less, tree, add), tokens)),
                Err(e) => Err(e),
            },
            Token::GreaterOrEqual => match add(tokens[1..].to_vec()) {
                Ok((add, tokens)) => Ok((Tree::new_tree(NodeKind::LessOrEqual, add, tree), tokens)),
                Err(e) => Err(e),
            },
            Token::Greater => match add(tokens[1..].to_vec()) {
                Ok((add, tokens)) => Ok((Tree::new_tree(NodeKind::Less, add, tree), tokens)),
                Err(e) => Err(e),
            },
            _ => Ok((tree, tokens)),
        }
    }
}

fn add(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (tree, tokens) = match mul(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        match tokens[0] {
            Token::Add => match mul(tokens[1..].to_vec()) {
                Ok((mul, tokens)) => Ok((Tree::new_tree(NodeKind::Add, tree, mul), tokens)),
                Err(e) => Err(e),
            },
            Token::Sub => match mul(tokens[1..].to_vec()) {
                Ok((mul, tokens)) => Ok((Tree::new_tree(NodeKind::Sub, tree, mul), tokens)),
                Err(e) => Err(e),
            },
            _ => Ok((tree, tokens)),
        }
    }
}

fn mul(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (tree, tokens) = match unary(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        match tokens[0] {
            Token::Mul => match unary(tokens[1..].to_vec()) {
                Ok((unary, tokens)) => Ok((Tree::new_tree(NodeKind::Mul, tree, unary), tokens)),
                Err(e) => Err(e),
            },
            Token::Div => match unary(tokens[1..].to_vec()) {
                Ok((unary, tokens)) => Ok((Tree::new_tree(NodeKind::Div, tree, unary), tokens)),
                Err(e) => Err(e),
            },
            _ => Ok((tree, tokens)),
        }
    }
}

fn unary(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    if tokens.is_empty() {
        primary(tokens)
    } else {
        match tokens[0] {
            Token::Add => primary(tokens[1..].to_vec()),
            Token::Sub => match primary(tokens[1..].to_vec()) {
                Ok((primary, tokens)) => Ok((
                    Tree::new_tree(NodeKind::Sub, Tree::new_int(0), primary),
                    tokens,
                )),
                Err(e) => Err(e),
            },
            _ => primary(tokens),
        }
    }
}

fn primary(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    if tokens.is_empty() {
        Err(TreeError::ParseError(
            "expect number or block but disappear".to_string(),
        ))
    } else {
        match tokens[0] {
            Token::LParen => match expr(tokens[1..].to_vec()) {
                Ok((expr, tokens)) => match tokens[0] {
                    Token::RParen => Ok((expr, tokens)),
                    _ => Err(TreeError::ParseError(
                        "expect ')' but disappear".to_string(),
                    )),
                },
                Err(e) => Err(e),
            },
            Token::Integer(n) => Ok((Tree::new_int(n), tokens[1..].to_vec())),
            Token::Variable { offset } => Ok((Tree::new_val(offset), tokens[1..].to_vec())),
            _ => Err(TreeError::ParseError(
                "expect number or block but disappear".to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{lexer::lexer, variable::variable_analysis};

    use super::*;

    #[test]
    fn parser_test() {
        unimplemented!()
    }

    #[test]
    fn one_int_test() {
        let query = variable_analysis(lexer("500;").unwrap()).unwrap();

        assert_eq!(Ok(vec![Tree::Int(500)]), parser(query))
    }

    #[test]
    fn add_test() {
        let query = variable_analysis(lexer("1 + 2;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Add,
                Tree::Int(1),
                Tree::Int(2)
            )]),
            parser(query)
        );
    }

    #[test]
    fn sub_test() {
        let query = variable_analysis(lexer("2 - 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Sub,
                Tree::Int(2),
                Tree::Int(1)
            )]),
            parser(query)
        );
    }

    #[test]
    fn mul_test() {
        let query = variable_analysis(lexer("4 * 5;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Mul,
                Tree::Int(4),
                Tree::Int(5)
            )]),
            parser(query)
        );
    }

    #[test]
    fn div_test() {
        let query = variable_analysis(lexer("8 / 4;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Div,
                Tree::Int(8),
                Tree::Int(4)
            )]),
            parser(query)
        );
    }

    #[test]
    fn unary_test() {
        let query1 = variable_analysis(lexer("+1;").unwrap()).unwrap();
        let query2 = variable_analysis(lexer("-1;").unwrap()).unwrap();

        assert_eq!(Ok(vec![Tree::new_int(1)]), parser(query1));
        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Sub,
                Tree::new_int(0),
                Tree::new_int(1)
            )]),
            parser(query2)
        );
    }
}
