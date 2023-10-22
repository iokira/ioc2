pub type Num = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(Ident),
    Integer(Num),
    Semicolon,
    Equal,
    Equality,
    Noneequality,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ident {
    name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorToken {
    invailed_char: char,
    position: usize,
}
