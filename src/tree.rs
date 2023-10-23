use crate::token::Int;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
    Value,
    Assign,
    Equality,
    Nonequality,
    LessOrEqual,
    Less,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tree {
    Int(Int),
    Val(usize),
    Node(NodeKind, Box<Tree>, Box<Tree>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TreeError {
    ParseError,
}

impl Tree {
    pub fn new_tree(kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
        Tree::Node(kind, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_num(num: Int) -> Tree {
        Tree::Int(num)
    }
}
