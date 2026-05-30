use super::{Class, Span};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Token {
    class: Class,
    span: Span,
}

impl Token {
    pub fn new<S: Into<Span>>(class: Class, span: S) -> Self {
        Self {
            class,
            span: span.into(),
        }
    }

    pub fn class(self) -> Class {
        self.class
    }

    pub fn resolve_as_f64(self, source: &[u8]) -> f64 {
        let slice = &source[self.span.to_range()];
        let str = str::from_utf8(slice).unwrap();
        f64::from_str(str).unwrap()
    }

    pub fn resolve_as_string(self, source: &[u8]) -> String {
        let slice = &source[self.span.to_range()];
        let str = str::from_utf8(slice).unwrap();
        str.into()
    }

    pub fn span(self) -> Span {
        self.span
    }
}
