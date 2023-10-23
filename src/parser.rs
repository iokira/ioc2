use crate::{token::Token, tree::*};

pub fn parser(_tokens: Vec<Token>) -> Result<Vec<Tree>, TreeError> {
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
