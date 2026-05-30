use crate::ast::Expr;
use crate::ops::Binop;

#[derive(Debug, PartialEq)]
pub struct BinaryOperator {
    binop: Binop,
    left: Box<Expr>,
    right: Box<Expr>,
}

impl BinaryOperator {
    pub fn new(binop: Binop, left: Box<Expr>, right: Box<Expr>) -> BinaryOperator {
        Self { binop, left, right }
    }

    pub fn binop(&self) -> Binop {
        self.binop
    }

    pub fn left(&self) -> &Expr {
        &self.left
    }

    pub fn right(&self) -> &Expr {
        &self.right
    }
}
