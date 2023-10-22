pub type Num = usize;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Variable {
    name: String,
}
