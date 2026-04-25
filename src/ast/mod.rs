mod binop;

pub use binop::*;

pub enum Node {
    BinaryOperator(BinaryOperator),
    Value(f64),
}
