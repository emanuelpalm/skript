use crate::ast::BinaryOperator;

pub enum Node {
    BinaryOperator(BinaryOperator),
    Value(f64),
}