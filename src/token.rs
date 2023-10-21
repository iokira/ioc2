#[derive(Debug, PartialEq)]
enum Token {
    Variable,
    Semicolon,
    IntegerLiteral,
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
