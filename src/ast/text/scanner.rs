use crate::ast::Span;

pub struct Scanner<'a> {
    source: &'a [u8],
    mark: u32,
    offset: u32,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self { source, mark: 0, offset: 0 }
    }

    pub fn get_marked_as_slice(&self) -> &'a [u8] {
        &self.source[self.mark as usize..self.offset as usize]
    }
    
    pub fn get_marked_as_span(&self) -> Span {
        Span { start: self.mark, stop: self.offset }
    }

    pub fn move_mark_to_offset(&mut self) {
        self.mark = self.offset
    }

    pub fn next(&mut self) -> Option<u8> {
        let ch = self.source.get(self.offset as usize)?;
        self.offset += 1;
        Some(*ch)
    }

    pub fn peek(&self) -> Option<u8> {
        self.source.get(self.offset as usize).cloned()
    }

    pub fn skip1(&mut self) {
        self.offset += 1;
    }

    pub fn skip_match(&mut self, predicate: fn(&u8) -> bool) -> bool {
        match self.peek() {
            Some(ch) if predicate(&ch) => {
                self.skip1();
                true
            },
            _ => false,
        }
    }

    pub fn skip_u8(&mut self, v: u8) -> bool {
        match self.peek() {
            Some(u) => v == u,
            None => false,
        }
    }
    
    pub fn skip_while(&mut self, predicate: fn(&u8) -> bool) -> u32 {
        let start = self.offset;
        loop {
            match self.peek() {
                Some(ch) if predicate(&ch) => {
                    self.skip1();
                }
                _ => return self.offset - start,
            };
        }
    }
}
