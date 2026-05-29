use crate::hir;
use crate::ops::Binop;

pub enum Instr {
    Load {
        target: hir::Register,
        value: f64,
    },

    BinaryOperator {
        kind: Binop,
        left: hir::Register,
        right: hir::Register,
        target: hir::Register,
    },

    Return {
        source: hir::Register,
    },
}