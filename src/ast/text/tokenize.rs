use crate::txt::Scanner;
use super::{Class, Token};

pub fn tokenize(source: &[u8]) -> Vec<Token> {
    let mut scanner = Scanner::new(source);

    let mut tokens = Vec::new();
    loop {
        scanner.skip_while(u8::is_ascii_whitespace);
        scanner.move_mark_to_offset();
        let class = match scanner.next() {
            Some(b'(') => Class::ParenthesisLeft,
            Some(b')') => Class::ParenthesisRight,
            Some(b'*') => Class::Asterisk,
            Some(b'+') => Class::Plus,
            Some(b',') => Class::Comma,
            Some(b'-') => if scanner.skip_u8(b'>') {
                Class::Arrow
            } else {
                Class::Dash
            },
            Some(b'/') => Class::Slash,
            Some(b'0'..=b'9') => {
                scanner.skip_while(u8::is_ascii_digit);
                if scanner.skip_u8(b'.') {
                    scanner.skip_while(u8::is_ascii_digit);
                }
                if scanner.skip_match(|ch| *ch == b'E' || *ch == b'e') {
                    scanner.skip_match(|ch| *ch == b'+' || *ch == b'-');
                    scanner.skip_while(u8::is_ascii_digit);
                }
                Class::Number
            },
            Some(b';') => Class::Semicolon,
            Some(b'=') => Class::Equal,
            Some(b'A'..=b'Z' | b'_' | b'a'..=b'z') => {
                scanner.skip_while(u8::is_ascii_alphanumeric);
                match scanner.get_marked_as_slice() {
                    b"let" => Class::Let,
                    b"return" => Class::Return,
                    _ => Class::Identifier,
                }
            }
            Some(b'{') => Class::BraceLeft,
            Some(b'}') => Class::BraceRight,
            Some(_) => {
                scanner.skip_while(u8::is_ascii_graphic);
                Class::Unknown
            },
            None => break,
        };
        let token = Token::new(class, scanner.get_marked_as_span());
        tokens.push(token);
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_tokens_from_source() {
        let source = "let digits = 12 + (3 / 4); () -> {};";
        let result = tokenize(source.as_bytes());
        assert_eq!(result, vec![
            Token::new(Class::Let, 0..3),
            Token::new(Class::Identifier, 4..10),
            Token::new(Class::Equal, 11..12),
            Token::new(Class::Number, 13..15),
            Token::new(Class::Plus, 16..17),
            Token::new(Class::ParenthesisLeft, 18..19),
            Token::new(Class::Number, 19..20),
            Token::new(Class::Slash, 21..22),
            Token::new(Class::Number, 23..24),
            Token::new(Class::ParenthesisRight, 24..25),
            Token::new(Class::Semicolon, 25..26),
            Token::new(Class::ParenthesisLeft, 27..28),
            Token::new(Class::ParenthesisRight, 28..29),
            Token::new(Class::Arrow, 30..32),
            Token::new(Class::BraceLeft, 33..34),
            Token::new(Class::BraceRight, 34..35),
            Token::new(Class::Semicolon, 35..36),
        ]);
    }
}