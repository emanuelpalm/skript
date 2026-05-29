use crate::hir;

pub struct Accumulator {
    instructions: Vec<hir::Instr>,
    register_counter: hir::Register,
}

impl Accumulator {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            register_counter: 0,
        }
    }

    pub fn allocate(&mut self) -> hir::Register {
        let n = self.register_counter;
        self.register_counter += 1;
        n
    }

    pub fn push(&mut self, instr: hir::Instr) {
        self.instructions.push(instr);
    }

    pub fn unwrap(self) -> Vec<hir::Instr> {
        self.instructions
    }
}
