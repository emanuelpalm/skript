#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    Asterisk,
    Dash,
    Number(f64),
    ParenthesisLeft,
    ParenthesisRight,
    Plus,
    Slash,
}
