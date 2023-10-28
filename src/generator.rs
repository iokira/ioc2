use crate::tree::Tree;

pub type GenerateError = String;

pub fn generator(trees: Vec<Tree>, ident_count: usize) -> Result<String, GenerateError> {
    let mut asm = String::new();
    Ok(asm)
}
