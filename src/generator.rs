use crate::tree::Tree;

pub type GenerateError = String;

pub fn generator(_trees: Vec<Tree>, _ident_count: usize) -> Result<String, GenerateError> {
    let asm = String::new();
    Ok(asm)
}
