use crate::txt::Span;
use super::{Class, Error, ErrorKind, Token};

pub struct TokenStream<'a> {
    tokens: &'a [Token],
    offset: u32,
}

impl<'a> TokenStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, offset: 0 }
    }

    pub fn end(&self) -> Span {
        (self.offset..self.offset).into()
    }

    pub fn expect_class_or(&mut self, class: Class, err: ErrorKind) -> Result<Token, Error> {
        let token = self.next()?;
        if token.class() == class {
            Ok(token)
        } else {
            Err(Error::new(err, token.span()))
        }
    }

    pub fn next(&mut self) -> Result<Token, Error> {
        match self.tokens.get(self.offset as usize) {
            Some(token) => {
                self.offset += 1;
                Ok(*token)
            }
            None => Err(Error::new(ErrorKind::UnexpectedEnd, self.end())),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.offset as usize)
    }

    pub fn peek_class(&self) -> Option<Class> {
        self.peek().map(|t| t.class())
    }

    pub fn restore(&mut self, snapshot: u32) {
        self.offset = snapshot;
    }

    pub fn skip1(&mut self) {
        if self.offset < (self.tokens.len() as u32) {
            self.offset += 1;
        }
    }

    pub fn skip_class(&mut self, class: Class) -> bool {
        if self.peek_class() == Some(class) {
            self.skip1();
            return true;
        }
        false
    }

    pub fn snapshot(&self) -> u32 {
        self.offset
    }
}
