use crate::hir::Instr;

pub struct Block {
    instructions: Vec<Instr>,
}

impl Block {
    pub fn new(instructions: Vec<Instr>) -> Self {
        Self { instructions }
    }

    pub fn instructions(&self) -> &[Instr] {
        &self.instructions
    }
}