use crate::{hir, rvm};
use crate::hir::{BinaryOperatorCode, Instr};
use crate::rvm::opcode::{OP_ADD, OP_DIV, OP_LOAD_I, OP_MUL, OP_RET, OP_SUB};

struct Accumulator {
    instructions: Vec<rvm::Instr>,
}

impl Accumulator {
    fn new() -> Self {
        Self {
            instructions: Vec::new(),
        }
    }

    fn push(&mut self, instr: rvm::Instr) {
        self.instructions.push(instr);
    }

    fn unwrap(self) -> Box<[rvm::Instr]> {
        self.instructions.into_boxed_slice()
    }
}

pub fn lower(block: &hir::Block) -> rvm::Code {
    let mut acc = Accumulator::new();
    lower_block(block, &mut acc);
    rvm::Code::new(acc.unwrap())
}

fn lower_block(block: &hir::Block, acc: &mut Accumulator) {
    for instr in block.instructions() {
        match instr {
            Instr::Load { target, value } => {
                acc.push(rvm::Instr::from_a_bx_i32(OP_LOAD_I, target.0, *value as i32));
            }

            Instr::BinaryOperator { left, right, operator, target } => {
                let op = match operator {
                    BinaryOperatorCode::Add => OP_ADD,
                    BinaryOperatorCode::Sub => OP_SUB,
                    BinaryOperatorCode::Mul => OP_MUL,
                    BinaryOperatorCode::Div => OP_DIV,
                };

                acc.push(rvm::Instr::from_a_b_c(op, target.0, left.0, right.0));
            }

            Instr::Return { source } => {
                acc.push(rvm::Instr::from_a_b_c(OP_RET, source.0, 0, 0));
            }
        }
    }
}