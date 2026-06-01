use std::collections::HashMap;
use crate::hir;

pub struct Accumulator {
    instructions: Vec<hir::Instr>,
    register_counter: hir::Register,
    symbol_table: HashMap<String, hir::Register>,
}

impl Accumulator {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            register_counter: 0,
            symbol_table: HashMap::new(),
        }
    }

    pub fn allocate(&mut self) -> hir::Register {
        let n = self.register_counter;
        self.register_counter += 1;
        n
    }

    pub fn bind(&mut self, register: hir::Register, name: &str) {
        self.symbol_table.insert(name.into(), register);
    }

    pub fn lookup(&mut self, name: &str) -> Option<hir::Register> {
        self.symbol_table.get(name).cloned()
    }

    pub fn push(&mut self, instr: hir::Instr) {
        self.instructions.push(instr);
    }

    pub fn unwrap(self) -> Vec<hir::Instr> {
        self.instructions
    }
}
