use crate::ast::Node;
use crate::ops::Binop;

#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    binop: Binop,
    left: Box<Node>,
    right: Box<Node>,
}

impl BinaryOperator {
    pub fn new(binop: Binop, left: Box<Node>, right: Box<Node>) -> BinaryOperator {
        Self { binop, left, right }
    }

    pub fn binop(&self) -> Binop {
        self.binop
    }

    pub fn left(&self) -> &Node {
        &self.left
    }

    pub fn right(&self) -> &Node {
        &self.right
    }
}
