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
    None,
    Int(Int),
    Val { offset: usize },
    Return(Box<Tree>),
    If(Box<Tree>, Box<Tree>),
    IfElse(Box<Tree>, Box<Tree>, Box<Tree>),
    While(Box<Tree>, Box<Tree>),
    For(Box<Tree>, Box<Tree>, Box<Tree>, Box<Tree>),
    Node(NodeKind, Box<Tree>, Box<Tree>),
}

pub type TreeError = String;

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

    pub fn new_return(tree: Tree) -> Tree {
        Tree::Return(Box::new(tree))
    }

    pub fn new_if(expr: Tree, stmt: Tree) -> Tree {
        Tree::If(Box::new(expr), Box::new(stmt))
    }

    pub fn new_if_else(expr: Tree, stmt: Tree, else_stmt: Tree) -> Tree {
        Tree::IfElse(Box::new(expr), Box::new(stmt), Box::new(else_stmt))
    }

    pub fn new_while(expr: Tree, stmt: Tree) -> Tree {
        Tree::While(Box::new(expr), Box::new(stmt))
    }

    pub fn new_for(init_expr: Tree, cond_expr: Tree, loop_expr: Tree, stmt: Tree) -> Tree {
        Tree::For(
            Box::new(init_expr),
            Box::new(cond_expr),
            Box::new(loop_expr),
            Box::new(stmt),
        )
    }
}
