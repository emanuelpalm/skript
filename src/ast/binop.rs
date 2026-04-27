use crate::ast::Node;

#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    code: BinaryOperatorCode,
    left: Box<Node>,
    right: Box<Node>,
}

impl BinaryOperator {
    pub fn new(code: BinaryOperatorCode, left: Box<Node>, right: Box<Node>) -> BinaryOperator {
        Self { code, left, right }
    }

    pub fn code(&self) -> BinaryOperatorCode {
        self.code
    }

    pub fn left(&self) -> &Node {
        &self.left
    }

    pub fn right(&self) -> &Node {
        &self.right
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BinaryOperatorCode {
    Add,
    Sub,
    Mul,
    Div,
}
