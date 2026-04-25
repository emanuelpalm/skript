use crate::ast::text::Token;

#[derive(Debug, PartialEq)]
pub enum Error {
    ExpectedParenthesisRight,
    UnexpectedCharacter(char),
    UnexpectedEnd,
    UnexpectedToken(Token),
}
