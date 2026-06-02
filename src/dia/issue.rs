use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Issue {
    MissingClosingParenthesis,
    MissingEqualAfterLetIdentifier,
    MissingIdentifierAfterLet,
    MissingSemicolonAfterStatement,
    InvalidToken,
    UnexpectedEnd,
    UnexpectedToken,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Issue::MissingClosingParenthesis => "missing closing parenthesis `)`",
            Issue::MissingEqualAfterLetIdentifier => "missing equal `=` after `let` identifier",
            Issue::MissingIdentifierAfterLet => "missing identifier after `let`",
            Issue::MissingSemicolonAfterStatement => "missing semicolon `;` after statement",
            Issue::InvalidToken => "invalid token",
            Issue::UnexpectedEnd => "unexpected end",
            Issue::UnexpectedToken => "unexpected token",
        })
    }
}