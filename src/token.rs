pub type Num = usize;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Variable(Variable),
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
pub struct Variable {
    name: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ErrorToken {
    invailed_char: char,
    position: usize,
}
