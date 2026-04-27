use crate::ast::BinaryOperator;

#[derive(Debug, PartialEq)]
pub enum Node {
    BinaryOperator(BinaryOperator),
    Value(f64),
}