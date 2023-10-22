pub type Int = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(Ident),
    Integer(Int),
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
