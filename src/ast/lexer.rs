pub struct Lexer<'a> {
    input: &'a [u8],
    mark: usize,
    offset: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { input, mark: 0, offset: 0 }
    }

    pub fn mark(&mut self) {
        self.mark = self.offset
    }

    pub fn next(&mut self) -> Option<u8> {
        let ch = self.input.get(self.offset)?;
        self.offset += 1;
        Some(*ch)
    }

    pub fn peek(&self) -> Option<u8> {
        self.input.get(self.offset).cloned()
    }

    pub fn skip1(&mut self) {
        self.offset += 1;
    }

    pub fn skip_ch(&mut self) -> Option<u8> {
        todo!()
    }
}