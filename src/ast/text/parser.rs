use super::{Class, Error, ErrorKind, Token};

pub struct Parser<'a> {
    tokens: &'a [Token],
    offset: u32,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, offset: 0 }
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
            None => Err(Error::new(
                ErrorKind::UnexpectedEnd,
                self.offset..self.offset,
            )),
        }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.offset as usize)
    }

    pub fn peek_class(&self) -> Option<Class> {
        self.peek().map(|t| t.class())
    }

    pub fn skip1(&mut self) {
        if self.offset < (self.tokens.len() as u32) {
            self.offset += 1;
        }
    }
}
