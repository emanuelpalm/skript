pub struct Program<'a> {
    code: &'a [u8],
    pc: usize,
}

impl<'a> Program<'a>  {
    pub fn new(code: &'a [u8]) -> Self {
        Self { code, pc: 0 }
    }

    pub fn read(&self) -> u8 {
        self.code[self.pc]
    }

    pub fn step(&mut self) {
        if self.pc < self.code.len() {
            self.pc += 1;
        }
    }
}