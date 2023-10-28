pub type Int = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Integer(Int),
    Ident(Ident),
    Variable { offset: usize },
    Return,
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

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::Ident(Ident { name: value })
    }
}

impl<'a> From<&'a str> for Token {
    fn from(value: &'a str) -> Self {
        Token::Ident(Ident {
            name: value.to_string(),
        })
    }
}

impl From<Int> for Token {
    fn from(value: Int) -> Self {
        Token::Integer(value)
    }
}
