use crate::ops::Binop;

#[derive(Debug, PartialEq)]
pub enum Expr {
    BinaryOperator {
        binop: Binop,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Identifier(String),
    Value(f64),
}