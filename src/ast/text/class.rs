#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Class {
    Asterisk,
    Dash,
    Equal,
    Identifier,
    Let,
    Number,
    ParenthesisLeft,
    ParenthesisRight,
    Plus,
    Return,
    Semicolon,
    Slash,
}