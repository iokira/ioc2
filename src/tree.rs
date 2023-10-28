use crate::token::Int;

#[derive(Debug, PartialEq, Clone)]
pub enum NodeKind {
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
    Val { offset: usize },
    Node(NodeKind, Box<Tree>, Box<Tree>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum TreeError {
    ParseError(String),
}

impl Tree {
    pub fn new_tree(kind: NodeKind, lhs: Tree, rhs: Tree) -> Tree {
        Tree::Node(kind, Box::new(lhs), Box::new(rhs))
    }

    pub fn new_int(num: Int) -> Tree {
        Tree::Int(num)
    }

    pub fn new_val(offset: usize) -> Tree {
        Tree::Val { offset }
    }
}
