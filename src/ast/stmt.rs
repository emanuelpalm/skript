use super::Expr;

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expr(Box<Expr>),
    Function {
        parameters: Vec<String>,
        body: Vec<Stmt>,
    },
    Let {
        identifier: String,
        expr: Box<Expr>,
    },
    Return(Box<Expr>),
}