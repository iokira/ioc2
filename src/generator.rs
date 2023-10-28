use crate::tree::Tree;

pub enum GenerateError {
    GenerateError,
}

pub fn generator(trees: Vec<Tree>) -> Result<String, GenerateError> {
    Ok("".to_string())
}
