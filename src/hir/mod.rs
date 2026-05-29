pub mod rvm;

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

pub enum Instr {
    Load {
        target: Register,
        value: f64,
    },

    BinaryOperator {
        operator: BinaryOperatorCode,
        left: Register,
        right: Register,
        target: Register,
    },

    Return {
        source: Register,
    },
}

pub enum BinaryOperatorCode {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Register(pub u8);

impl From<Register> for u8 {
    fn from(reg: Register) -> Self {
        reg.0
    }
}
