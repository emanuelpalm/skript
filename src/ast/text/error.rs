use std::error;
use std::fmt::{Display, Formatter};
use crate::ast::text::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedParenthesisRight,
    UnexpectedCharacter(char),
    UnexpectedEnd,
    UnexpectedToken(Token),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}
