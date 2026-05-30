use super::Span;
use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

impl Error {
    pub fn new<S: Into<Span>>(kind: ErrorKind, span: S) -> Self {
        Error { kind, span: span.into() }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ErrorKind {
    ExpectedClosingParenthesis,
    ExpectedEqualAfterLetIdentifier,
    ExpectedIdentifierAfterLet,
    ExpectedSemicolonAfterStatement,
    InvalidToken,
    UnexpectedEnd,
    UnexpectedToken,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ErrorKind::ExpectedClosingParenthesis => "expected closing parenthesis `)`",
            ErrorKind::ExpectedEqualAfterLetIdentifier => "expected equal `=` after `let` identifier",
            ErrorKind::ExpectedIdentifierAfterLet => "expected identifier after `let`",
            ErrorKind::ExpectedSemicolonAfterStatement => "expected semicolon `;` after statement",
            ErrorKind::InvalidToken => "invalid token",
            ErrorKind::UnexpectedEnd => "unexpected end",
            ErrorKind::UnexpectedToken => "unexpected token",
        })
    }
}