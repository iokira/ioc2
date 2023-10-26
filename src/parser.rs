use crate::{token::Token, tree::*};

pub fn parser(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn program(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn stmt(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn expr(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn assign(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn equality(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn relational(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn add(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn mul(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn unary(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn primary(tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        let tokens = vec![Token::Integer(1), Token::Add, Token::Integer(1)];

        assert_eq!(
            Ok(vec![Tree::new_tree(
                NodeKind::Add,
                Tree::Int(1),
                Tree::Int(1)
            )]),
            parser(tokens)
        );
    }
}
