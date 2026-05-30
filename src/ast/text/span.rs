use std::fmt;
use std::ops::Range;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub struct Span {
    pub start: u32,
    pub stop: u32,
}

impl Span {
    pub fn to_range(self) -> Range<usize> {
        self.start as usize..self.stop as usize
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.stop)
    }
}

impl From<Range<u32>> for Span {
    fn from(range: Range<u32>) -> Self {
        Self { start: range.start, stop: range.end }
    }
}
