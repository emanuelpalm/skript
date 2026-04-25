pub struct Scanner<'a> {
    source: &'a [u8],
    mark: usize,
    offset: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a [u8]) -> Self {
        Self { source, mark: 0, offset: 0 }
    }

    pub fn get_marked_slice(&self) -> &'a [u8] {
        &self.source[self.mark..self.offset]
    }

    pub fn move_mark_to_offset(&mut self) {
        self.mark = self.offset
    }

    pub fn next(&mut self) -> Option<u8> {
        let ch = self.source.get(self.offset)?;
        self.offset += 1;
        Some(*ch)
    }

    pub fn peek(&self) -> Option<u8> {
        self.source.get(self.offset).cloned()
    }

    pub fn skip1(&mut self) {
        self.offset += 1;
    }

    pub fn skip_ch(&mut self, ch: u8) -> bool {
        match self.peek() {
            Some(ch0) => ch == ch0,
            None => false,
        }
    }

    pub fn skip_if(&mut self, predicate: fn(&u8) -> bool) -> bool {
        match self.peek() {
            Some(ch) if predicate(&ch) => {
                self.skip1();
                true
            },
            _ => false,
        }
    }

    pub fn skip_while(&mut self, predicate: fn(&u8) -> bool) -> usize {
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
