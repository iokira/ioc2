pub type Int = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(Int),
    Ident(Ident),
    Variable { name: String, offset: usize },
    Return,
    If,
    Else,
    While,
    For,
    Semicolon,
    Equality,
    Equal,
    Noneequality,
    LessOrEqual,
    Less,
    GreaterOrEqual,
    Greater,
    Add,
    Sub,
    Mul,
    Div,
    LParen,
    RParen,
    OpenBrace,
    CloseBrace,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, PartialOrd, Ord)]
pub struct Ident {
    pub name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenError {
    TokenizeError,
    InvalidChar(char),
}
