use crate::{token::Token, tree::*};

pub fn parser(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    program(tokens)
}

fn program(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    if tokens.is_empty() {
        Ok(vec![])
    } else {
        let (tree, tokens) = stmt(tokens)?;
        let trees = program(tokens)?;
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
            Token::CloseBrace => (Tree::None, tokens[1..].to_vec()),
            Token::OpenBrace => {
                fn go(tokens: Vec<Token>) -> Result<(Vec<Tree>, Vec<Token>), TreeError> {
                    match tokens[0] {
                        Token::CloseBrace => Ok((vec![], tokens[1..].to_vec())),
                        _ => {
                            let (go_stmt, tokens) = stmt(tokens[1..].to_vec())?;
                            let (go_trees, tokens) = go(tokens)?;
                            Ok(([vec![go_stmt], go_trees].concat(), tokens))
                        }
                    }
                }
                let (stmts, tokens) = go(tokens)?;
                (Tree::new_block(stmts), tokens)
            }
            _ => {
                let (expr_tree, tokens) = expr(tokens)?;
                if tokens.is_empty() {
                    return Err(semicolon_error());
                } else {
                    match tokens[0] {
                        Token::Semicolon => (expr_tree, tokens[1..].to_vec()),
                        _ => return Err(semicolon_error()),
                    }
                }
            }
        };
        Ok((tree, tokens))
    }
}

fn expr(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    assign(tokens)
}

fn assign(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (tree, tokens) = equality(tokens)?;
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        match tokens[0] {
            Token::Equal => {
                let (assign, tokens) = assign(tokens[1..].to_vec())?;
                Ok((Tree::new_tree(NodeKind::Assign, tree, assign), tokens))
            }
            _ => Ok((tree, tokens)),
        }
    }
}

fn equality(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    fn go(tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
        let (tree, tokens) = match tokens[0] {
            Token::Equality => {
                let (relational, tokens) = relational(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::Equality, tree, relational), tokens)
            }
            Token::Noneequality => {
                let (relational, tokens) = relational(tokens[1..].to_vec())?;
                (
                    Tree::new_tree(NodeKind::Nonequality, tree, relational),
                    tokens,
                )
            }
            _ => return Ok((tree, tokens)),
        };
        go(tree, tokens)
    }
    let (tree, tokens) = relational(tokens)?;
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        go(tree, tokens)
    }
}

fn relational(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    fn go(tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
        let (tree, tokens) = match tokens[0] {
            Token::LessOrEqual => {
                let (add, tokens) = add(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::LessOrEqual, tree, add), tokens)
            }
            Token::Less => {
                let (add, tokens) = add(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::Less, tree, add), tokens)
            }
            Token::GreaterOrEqual => {
                let (add, tokens) = add(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::LessOrEqual, add, tree), tokens)
            }
            Token::Greater => {
                let (add, tokens) = add(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::Less, add, tree), tokens)
            }
            _ => return Ok((tree, tokens)),
        };
        go(tree, tokens)
    }
    let (tree, tokens) = add(tokens)?;
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        go(tree, tokens)
    }
}

fn add(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    fn go(tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
        let (tree, tokens) = match tokens[0] {
            Token::Add => {
                let (mul, tokens) = mul(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::Add, tree, mul), tokens)
            }
            Token::Sub => {
                let (mul, tokens) = mul(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::Sub, tree, mul), tokens)
            }
            _ => return Ok((tree, tokens)),
        };
        go(tree, tokens)
    }
    let (tree, tokens) = mul(tokens)?;
    if tokens.is_empty() {
        Ok((tree, tokens))
    } else {
        go(tree, tokens)
    }
}

fn mul(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    fn go(tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
        let (tree, tokens) = match tokens[0] {
            Token::Mul => {
                let (unary, tokens) = unary(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::Mul, tree, unary), tokens)
            }
            Token::Div => {
                let (unary, tokens) = unary(tokens[1..].to_vec())?;
                (Tree::new_tree(NodeKind::Div, tree, unary), tokens)
            }
            _ => return Ok((tree, tokens)),
        };
        go(tree, tokens)
    }
    let (tree, tokens) = unary(tokens)?;
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
            Token::Sub => {
                let (primary, tokens) = primary(tokens[1..].to_vec())?;
                Ok((
                    Tree::new_tree(NodeKind::Sub, Tree::new_int(0), primary),
                    tokens,
                ))
            }
            _ => primary(tokens),
        }
    }
}

fn primary(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    if tokens.is_empty() {
        Err("expect number or block but disappear".to_owned())
    } else {
        match tokens[0] {
            Token::LParen => {
                let (expr, tokens) = expr(tokens[1..].to_vec())?;
                match tokens[0] {
                    Token::RParen => Ok((expr, tokens[1..].to_vec())),
                    _ => Err("expect ')' but disappear".to_owned()),
                }
            }
            Token::Integer(n) => Ok((Tree::new_int(n), tokens[1..].to_vec())),
            Token::Variable { offset } => Ok((Tree::new_val(offset), tokens[1..].to_vec())),
            _ => Err("expect number or block but disappear".to_owned()),
        }
    }
}

fn parse_return(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (expr_tree, tokens) = expr(tokens)?;
    if tokens.is_empty() {
        Err(semicolon_error())
    } else {
        match tokens[0] {
            Token::Semicolon => Ok((Tree::new_return(expr_tree), tokens[1..].to_vec())),
            _ => Err(semicolon_error()),
        }
    }
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
    if tokens.is_empty() {
        Err(lparen_error())
    } else {
        match tokens[0] {
            Token::LParen => parse_init_tree(tokens[1..].to_vec()),
            _ => Err(lparen_error()),
        }
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
    if tokens.is_empty() {
        Ok((Tree::new_if(expr_tree, stmt_tree), tokens))
    } else {
        match tokens[0] {
            Token::Else => parse_if_else(expr_tree, stmt_tree, tokens[1..].to_vec()),
            _ => Ok((Tree::new_if(expr_tree, stmt_tree), tokens)),
        }
    }
}

fn parse_while_tree(expr_tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    let (stmt_tree, tokens) = stmt(tokens)?;
    Ok((Tree::new_while(expr_tree, stmt_tree), tokens))
}

fn parse_for_tree(
    init_tree: Tree,
    cond_tree: Tree,
    loop_tree: Tree,
    tokens: Vec<Token>,
) -> Result<(Tree, Vec<Token>), TreeError> {
    let (stmt_tree, tokens) = stmt(tokens)?;
    Ok((
        Tree::new_for(init_tree, cond_tree, loop_tree, stmt_tree),
        tokens,
    ))
}

fn parse_loop_tree(
    init_tree: Tree,
    cond_tree: Tree,
    tokens: Vec<Token>,
) -> Result<(Tree, Vec<Token>), TreeError> {
    if tokens.is_empty() {
        Err("expected loop expression or ')' but disappear".to_owned())
    } else {
        match tokens[0] {
            Token::RParen => parse_for_tree(init_tree, cond_tree, Tree::None, tokens[1..].to_vec()),
            _ => {
                let (loop_tree, tokens) = expr(tokens)?;
                match tokens[0] {
                    Token::RParen => {
                        parse_for_tree(init_tree, cond_tree, loop_tree, tokens[1..].to_vec())
                    }
                    _ => Err(rparen_error()),
                }
            }
        }
    }
}

fn parse_cond_tree(init_tree: Tree, tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    if tokens.is_empty() {
        Err("expected semicolon or cond-expression but disappear".to_owned())
    } else {
        match tokens[0] {
            Token::Semicolon => parse_loop_tree(init_tree, Tree::None, tokens[1..].to_vec()),
            _ => {
                let (cond_tree, tokens) = expr(tokens)?;
                match tokens[0] {
                    Token::Semicolon => parse_loop_tree(init_tree, cond_tree, tokens[1..].to_vec()),
                    _ => Err(semicolon_error()),
                }
            }
        }
    }
}

fn parse_init_tree(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    if tokens.is_empty() {
        Err("expected semicolon or init-expression but disappear".to_owned())
    } else {
        match tokens[0] {
            Token::Semicolon => parse_cond_tree(Tree::None, tokens[1..].to_vec()),
            _ => {
                let (init_tree, tokens) = expr(tokens)?;
                match tokens[0] {
                    Token::Semicolon => parse_cond_tree(init_tree, tokens[1..].to_vec()),
                    _ => Err(semicolon_error()),
                }
            }
        }
    }
}

fn parse_paren_expr(tokens: Vec<Token>) -> Result<(Tree, Vec<Token>), TreeError> {
    if tokens.is_empty() {
        Err(lparen_error())
    } else {
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

fn closebrace_error() -> TreeError {
    "expected '}' but disappear".to_owned()
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

    #[test]
    fn if_else_test() {
        let (query, _ident_count) =
            variable_analysis(lexer("if(0)return0;else return1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_if_else(
                Tree::new_int(0),
                Tree::new_return(Tree::new_int(0)),
                Tree::new_return(Tree::new_int(1))
            )]),
            parser(query)
        );
    }

    #[test]
    fn while_test() {
        let (query, _ident_count) =
            variable_analysis(lexer("while (2 > 1) 1 + 1;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_while(
                Tree::new_tree(NodeKind::Less, Tree::new_int(1), Tree::new_int(2)),
                Tree::new_tree(NodeKind::Add, Tree::new_int(1), Tree::new_int(1))
            )]),
            parser(query)
        );
    }

    #[test]
    fn for_test() {
        let (query, _ident_count) = variable_analysis(lexer("for(;;)0;").unwrap()).unwrap();

        assert_eq!(
            Ok(vec![Tree::new_for(
                Tree::None,
                Tree::None,
                Tree::None,
                Tree::new_int(0)
            )]),
            parser(query)
        );
    }
}
