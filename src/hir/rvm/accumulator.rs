use crate::rvm;

pub struct Accumulator {
    instructions: Vec<rvm::Instr>,
}

impl Accumulator {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    pub fn push(&mut self, instr: rvm::Instr) {
        self.instructions.push(instr);
    }

    pub fn unwrap(self) -> Box<[rvm::Instr]> {
        self.instructions.into_boxed_slice()
    }
}