use super::accumulator::*;
use crate::{hir, rvm};
use crate::ops::Binop;
use crate::rvm::opcode::*;

pub fn lower(block: &hir::Block) -> rvm::Code {
    let mut acc = Accumulator::new();
    lower_block(block, &mut acc);
    rvm::Code::new(acc.unwrap())
}

fn lower_block(block: &hir::Block, acc: &mut Accumulator) {
    for instr in block.instructions() {
        match instr {
            hir::Instr::Load { target, value } => {
                acc.push(rvm::Instr::from_a_bx_i32(OP_LOAD_I, *target, *value as i32));
            }

            hir::Instr::BinaryOperator { left, right, binop: operator, target } => {
                let opcode = match operator {
                    Binop::Add => OP_ADD,
                    Binop::Sub => OP_SUB,
                    Binop::Mul => OP_MUL,
                    Binop::Div => OP_DIV,
                };
                acc.push(rvm::Instr::from_a_b_c(opcode, *target, *left, *right));
            }

            hir::Instr::Return { source } => {
                acc.push(rvm::Instr::from_a(OP_RET, *source));
            }
        }
    }
}