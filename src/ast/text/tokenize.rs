use std::str::from_utf8_unchecked;
use crate::ast::text::{Error, Scanner};
use crate::ast::text::token::Token;

pub fn tokenize(source: &[u8]) -> Result<Vec<Token>, Error> {
    let mut scanner = Scanner::new(source);

    let mut tokens = Vec::new();
    loop {
        scanner.skip_while(u8::is_ascii_whitespace);
        scanner.move_mark_to_offset();
        let token = match scanner.next() {
            Some(b'(') => Token::ParenthesisLeft,
            Some(b')') => Token::ParenthesisRight,
            Some(b'*') => Token::Asterisk,
            Some(b'+') => Token::Plus,
            Some(b'-') => Token::Dash,
            Some(b'/') => Token::Slash,
            Some(b'0'..=b'9') => {
                scanner.skip_while(u8::is_ascii_digit);
                if scanner.skip_ch(b'.') {
                    scanner.skip_while(u8::is_ascii_digit);
                }
                if scanner.skip_if(|ch| *ch == b'E' || *ch == b'e') {
                    scanner.skip_if(|ch| *ch == b'+' || *ch == b'-');
                    scanner.skip_while(u8::is_ascii_digit);
                }
                let slice = scanner.get_marked_slice();
                let str = unsafe { from_utf8_unchecked(slice) };
                let value = str.parse().unwrap();
                Token::Number(value)
            },
            Some(ch) => return Err(Error::UnexpectedCharacter(ch as char)),
            None => break,
        };
        tokens.push(token);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_correct_tokens_from_source() {
        let source = "12 + (3 / 4)";
        let result = tokenize(source.as_bytes());
        assert_eq!(result, Ok(vec![
            Token::Number(12.0),
            Token::Plus,
            Token::ParenthesisLeft,
            Token::Number(3.0),
            Token::Slash,
            Token::Number(4.0),
            Token::ParenthesisRight
        ]));
    }
}