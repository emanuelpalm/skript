use super::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Box<Expr>),
    Let {
        identifier: String,
        expr: Box<Expr>,
    },
    Return(Box<Expr>),
}