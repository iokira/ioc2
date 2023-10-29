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
    if tokens.is_empty() {
        Err(semicolon_error())
    } else {
        let (tree, tokens) = match tokens[0] {
            Token::If => parse_if(tokens[1..].to_vec())?,
            Token::While => parse_while(tokens[1..].to_vec())?,
            Token::For => parse_for(tokens[1..].to_vec())?,
            Token::Return => parse_return(tokens[1..].to_vec())?,
            _ => expr(tokens)?,
        };
        if tokens.is_empty() {
            Err(semicolon_error())
        } else {
            match tokens[0] {
                Token::Semicolon => Ok((tree, tokens[1..].to_vec())),
                _ => Err(semicolon_error()),
            }
        }
    }
}

fn expr(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    assign(tokens)
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
    fn go(tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
        let (tree, tokens) = match tokens[0] {
            Token::Equality => match relational(tokens[1..].to_vec()) {
                Ok((relational, tokens)) => {
                    (Tree::new_tree(NodeKind::Equality, tree, relational), tokens)
                }
                Err(e) => return Err(e),
            },
            Token::Noneequality => match relational(tokens[1..].to_vec()) {
                Ok((relational, tokens)) => (
                    Tree::new_tree(NodeKind::Nonequality, tree, relational),
                    tokens,
                ),
                Err(e) => return Err(e),
            },
            _ => return Ok((tree, tokens)),
        };
        go(tree, tokens)
    }
    let (tree, tokens) = match relational(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        go(tree, tokens)
    }
}

fn relational(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    fn go(tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
        let (tree, tokens) = match tokens[0] {
            Token::LessOrEqual => match add(tokens[1..].to_vec()) {
                Ok((add, tokens)) => (Tree::new_tree(NodeKind::LessOrEqual, tree, add), tokens),
                Err(e) => return Err(e),
            },
            Token::Less => match add(tokens[1..].to_vec()) {
                Ok((add, tokens)) => (Tree::new_tree(NodeKind::Less, tree, add), tokens),
                Err(e) => return Err(e),
            },
            Token::GreaterOrEqual => match add(tokens[1..].to_vec()) {
                Ok((add, tokens)) => (Tree::new_tree(NodeKind::LessOrEqual, add, tree), tokens),
                Err(e) => return Err(e),
            },
            Token::Greater => match add(tokens[1..].to_vec()) {
                Ok((add, tokens)) => (Tree::new_tree(NodeKind::Less, add, tree), tokens),
                Err(e) => return Err(e),
            },
            _ => return Ok((tree, tokens)),
        };
        go(tree, tokens)
    }
    let (tree, tokens) = match add(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        go(tree, tokens)
    }
}

fn add(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    fn go(tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
        let (tree, tokens) = match tokens[0] {
            Token::Add => match mul(tokens[1..].to_vec()) {
                Ok((mul, tokens)) => (Tree::new_tree(NodeKind::Add, tree, mul), tokens),
                Err(e) => return Err(e),
            },
            Token::Sub => match mul(tokens[1..].to_vec()) {
                Ok((mul, tokens)) => (Tree::new_tree(NodeKind::Sub, tree, mul), tokens),
                Err(e) => return Err(e),
            },
            _ => return Ok((tree, tokens)),
        };
        go(tree, tokens)
    }
    let (tree, tokens) = match mul(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        go(tree, tokens)
    }
}

fn mul(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    fn go(tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
        let (tree, tokens) = match tokens[0] {
            Token::Mul => match unary(tokens[1..].to_vec()) {
                Ok((unary, tokens)) => (Tree::new_tree(NodeKind::Mul, tree, unary), tokens),
                Err(e) => return Err(e),
            },
            Token::Div => match unary(tokens[1..].to_vec()) {
                Ok((unary, tokens)) => (Tree::new_tree(NodeKind::Div, tree, unary), tokens),
                Err(e) => return Err(e),
            },
            _ => return Ok((tree, tokens)),
        };
        go(tree, tokens)
    }
    let (tree, tokens) = match unary(tokens) {
        Ok((tree, tokens)) => (tree, tokens),
        Err(e) => return Err(e),
    };
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        go(tree, tokens)
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
        Err("expect number or block but disappear".to_owned())
    } else {
        match tokens[0] {
            Token::LParen => match expr(tokens[1..].to_vec()) {
                Ok((expr, tokens)) => match tokens[0] {
                    Token::RParen => Ok((expr, tokens[1..].to_vec())),
                    _ => Err("expect ')' but disappear".to_owned()),
                },
                Err(e) => Err(e),
            },
            Token::Integer(n) => Ok((Tree::new_int(n), tokens[1..].to_vec())),
            Token::Variable { offset } => Ok((Tree::new_val(offset), tokens[1..].to_vec())),
            _ => Err("expect number or block but disappear".to_owned()),
        }
    }
}

fn parse_return(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (expr_tree, tokens) = expr(tokens)?;
    Ok((Tree::new_return(expr_tree), tokens))
}

fn parse_if(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (expr_tree, tokens) = parse_paren_expr(tokens)?;
    parse_if_tree(expr_tree, tokens)
}

fn parse_while(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (expr_tree, tokens) = parse_paren_expr(tokens)?;
    parse_while_tree(expr_tree, tokens)
}

fn parse_for(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    match tokens[0] {
        Token::LParen => parse_init_tree(tokens[1..].to_vec()),
        _ => return Err(lparen_error()),
    }
}

fn parse_if_else(
    expr_tree: Tree,
    stmt_tree: Tree,
    tokens: Vec<Token>,
) -> Result<(Tree, Vec<Token>), TreeError> {
    let (else_stmt, tokens) = stmt(tokens)?;
    Ok((Tree::new_if_else(expr_tree, stmt_tree, else_stmt), tokens))
}

fn parse_if_tree(expr_tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (stmt_tree, tokens) = stmt(tokens)?;
    match tokens[0] {
        Token::Else => parse_if_else(expr_tree, stmt_tree, tokens[1..].to_vec()),
        _ => Ok((Tree::new_if(expr_tree, stmt_tree), tokens)),
    }
}

fn parse_while_tree(expr_tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (stmt_tree, tokens) = stmt(tokens)?;
    Ok((Tree::new_while(expr_tree, stmt_tree), tokens))
}

fn parse_for_tree(
    init_tree: Tree,
    inc_tree: Tree,
    cond_tree: Tree,
    tokens: Vec<Token>,
) -> Result<(Tree, Vec<Token>), TreeError> {
    let (stmt_tree, tokens) = stmt(tokens)?;
    Ok((
        Tree::new_for(init_tree, inc_tree, cond_tree, stmt_tree),
        tokens,
    ))
}

fn parse_cond_tree(
    init_tree: Tree,
    inc_tree: Tree,
    tokens: Vec<Token>,
) -> Result<(Tree, Vec<Token>), TreeError> {
    match tokens[0] {
        Token::RParen => parse_for_tree(init_tree, inc_tree, Tree::None, tokens[1..].to_vec()),
        _ => {
            let (cond_tree, tokens) = expr(tokens)?;
            match tokens[0] {
                Token::RParen => {
                    parse_for_tree(init_tree, inc_tree, cond_tree, tokens[1..].to_vec())
                }
                _ => Err(rparen_error()),
            }
        }
    }
}

fn parse_inc_tree(init_tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    match tokens[0] {
        Token::Semicolon => parse_cond_tree(init_tree, Tree::None, tokens[1..].to_vec()),
        _ => {
            let (inc_tree, tokens) = expr(tokens)?;
            match tokens[0] {
                Token::Semicolon => parse_cond_tree(init_tree, inc_tree, tokens[1..].to_vec()),
                _ => Err(semicolon_error()),
            }
        }
    }
}

fn parse_init_tree(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    match tokens[0] {
        Token::Semicolon => parse_inc_tree(Tree::None, tokens[1..].to_vec()),
        _ => {
            let (init_tree, tokens) = expr(tokens)?;
            match tokens[0] {
                Token::Semicolon => parse_inc_tree(init_tree, tokens[1..].to_vec()),
                _ => Err(semicolon_error()),
            }
        }
    }
}

fn parse_paren_expr(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    match tokens[0] {
        Token::LParen => {
            let (expr_tree, tokens) = expr(tokens[1..].to_vec())?;
            match tokens[0] {
                Token::RParen => Ok((expr_tree, tokens[1..].to_vec())),
                _ => Err(rparen_error()),
            }
        }
        _ => Err(lparen_error()),
    }
}

fn semicolon_error() -> TreeError {
    "expected semicolon but disappear".to_owned()
}

fn lparen_error() -> TreeError {
    "expected '(' but disappear".to_owned()
}

fn rparen_error() -> TreeError {
    "expected ')' but disappear".to_owned()
}

#[cfg(test)]
mod tests {
    use crate::{lexer::lexer, variable::variable_analysis};

    use super::*;

    #[test]
    fn parser_test() {
        let (query, _ident_count) = variable_analysis(
            lexer(
                "
column = 5;
row = 40;
column * row;
",
            )
            .unwrap(),
        )
        .unwrap();

        assert_eq!(
            Ok(vec![
                Tree::new_tree(NodeKind::Assign, Tree::new_val(8), Tree::new_int(5)),
                Tree::new_tree(NodeKind::Assign, Tree::new_val(16), Tree::new_int(40)),
                Tree::new_tree(NodeKind::Mul, Tree::new_val(8), Tree::new_val(16))
            ]),
            parser(query)
        );
    }

    #[test]
    fn one_int_test() {
        let (query, _ident_count) = variable_analysis(lexer("500;").unwrap()).unwrap();

        assert_eq!(Ok(vec![Tree::new_int(500)]), parser(query));
    }

    #[test]
    fn add_test() {
        let (query, _ident_count) = variable_analysis(lexer("1 + 2;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Add,
                Tree::new_int(1),
                Tree::new_int(2)
            )]),
            parser(query)
        );
    }

    #[test]
    fn sub_test() {
        let (query, _ident_count) = variable_analysis(lexer("2 - 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Sub,
                Tree::new_int(2),
                Tree::new_int(1)
            )]),
            parser(query)
        );
    }

    #[test]
    fn mul_test() {
        let (query, _ident_count) = variable_analysis(lexer("4 * 5;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Mul,
                Tree::new_int(4),
                Tree::new_int(5)
            )]),
            parser(query)
        );
    }

    #[test]
    fn div_test() {
        let (query, _ident_count) = variable_analysis(lexer("8 / 4;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Div,
                Tree::new_int(8),
                Tree::new_int(4)
            )]),
            parser(query)
        );
    }

    #[test]
    fn unary_test() {
        let (query1, _ident_count) = variable_analysis(lexer("+1;").unwrap()).unwrap();
        let (query2, _ident_count) = variable_analysis(lexer("-1;").unwrap()).unwrap();

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

    #[test]
    fn equality_test() {
        let (query, _ident_count) = variable_analysis(lexer("1 == 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Equality,
                Tree::new_int(1),
                Tree::new_int(1)
            )]),
            parser(query)
        );
    }

    #[test]
    fn noneequality_test() {
        let (query, _ident_count) = variable_analysis(lexer("1 != 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Nonequality,
                Tree::new_int(1),
                Tree::new_int(1)
            )]),
            parser(query)
        );
    }

    #[test]
    fn less_or_equal_test() {
        let (query, _ident_count) = variable_analysis(lexer("2 <= 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::LessOrEqual,
                Tree::new_int(2),
                Tree::new_int(1)
            )]),
            parser(query)
        );
    }

    #[test]
    fn less_test() {
        let (query, _ident_count) = variable_analysis(lexer("2 < 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Less,
                Tree::new_int(2),
                Tree::new_int(1)
            )]),
            parser(query)
        );
    }

    #[test]
    fn greater_or_equal_test() {
        let (query, _ident_count) = variable_analysis(lexer("2 >= 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::LessOrEqual,
                Tree::new_int(1),
                Tree::new_int(2)
            )]),
            parser(query)
        );
    }

    #[test]
    fn greater_test() {
        let (query, _ident_count) = variable_analysis(lexer("2 > 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Less,
                Tree::new_int(1),
                Tree::new_int(2)
            )]),
            parser(query)
        );
    }

    #[test]
    fn paren_test() {
        let (query, _ident_count) = variable_analysis(lexer("2 * (1 + 2);").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Mul,
                Tree::new_int(2),
                Tree::new_tree(NodeKind::Add, Tree::new_int(1), Tree::new_int(2))
            )]),
            parser(query)
        );
    }

    #[test]
    fn assign_test() {
        let (query, _ident_count) = variable_analysis(lexer("a = 123;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Assign,
                Tree::new_val(8),
                Tree::new_int(123)
            )]),
            parser(query)
        );
    }

    #[test]
    fn statement_test() {
        let (query, _ident_count) = variable_analysis(lexer("1;2;").unwrap()).unwrap();

        assert_eq!(Ok(vec![Tree::new_int(1), Tree::new_int(2)]), parser(query));
    }

    #[test]
    fn return_test() {
        let (query, _ident_count) = variable_analysis(lexer("return 0;").unwrap()).unwrap();

        assert_eq!(Ok(vec![Tree::new_return(Tree::new_int(0))]), parser(query));
    }

    #[test]
    fn if_test() {
        let (query, _ident_count) = variable_analysis(lexer("if(0)return0;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_if(
                Tree::new_int(0),
                Tree::new_return(Tree::new_int(0))
            )]),
            parser(query)
        );
    }
}
