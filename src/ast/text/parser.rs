use crate::ast::text::Token;

pub struct Parser<'a> {
    tokens: &'a [Token],
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, offset: 0 }
    }

    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.offset)?;
        self.offset += 1;
        Some(token)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.offset)
    }

    pub fn skip1(&mut self) {
        if self.offset < self.tokens.len() {
            self.offset += 1;
        }
    }
}