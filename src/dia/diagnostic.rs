use crate::txt::Span;
use super::Issue;

#[derive(Debug)]
pub struct Diagnostic {
    issue: Issue,
    span: Span,
}

impl Diagnostic {
    pub fn new<S: Into<Span>>(issue: Issue, span: S) -> Self {
        Diagnostic { issue, span: span.into() }
    }

    pub fn issue(&self) -> &Issue {
        &self.issue
    }

    pub fn span(&self) -> Span {
        self.span
    }
}