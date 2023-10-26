use crate::{token::Token, tree::*};

pub fn parser(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn program(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn stmt(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn expr(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn assign(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn equality(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn relational(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn add(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn mul(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn unary(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
    unimplemented!()
}

fn primary(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
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
