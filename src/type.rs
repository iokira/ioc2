#[derive(Debug, PartialEq)]
enum Token {
    Variable(Variable),
    IntegerLiteral,
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
struct Variable {
    name: String,
}
