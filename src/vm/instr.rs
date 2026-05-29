use std::fmt;
use crate::vm::opcode::*;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Instr(u32);

impl Instr {
    pub fn new(word: u32) -> Self {
        Instr(word)
    }

    pub fn from_a_bx(opcode: u8, a: u8, bx: u32) -> Self {
        Instr(((opcode as u32) << 26) | ((a as u32) << 18) | (bx & 0x0003_FFFF))
    }

    pub fn from_a_b_c(opcode: u8, a: u8, b: u8, c: u8) -> Self {
        Instr(((opcode as u32) << 26) | ((a as u32) << 18) | ((b as u32) << 12) | ((c as u32) << 2))
    }

    pub fn opcode(&self) -> u8 {
        ((self.0 >> 26) & 0x3F) as u8
    }

    pub fn a(&self) -> u8 {
        ((self.0 >> 18) & 0xFF) as u8
    }

    pub fn ax(&self) -> u32 {
        self.0 & 0x03FF_FFFF
    }

    pub fn b(&self) -> u8 {
        ((self.0 >> 10) & 0xFF) as u8
    }

    pub fn bx(&self) -> u32 {
        self.0 & 0x0003_FFFF
    }

    pub fn bx_i32(&self) -> i32 {
        ((self.bx() as i32) << 14) >> 14
    }

    pub fn c(&self) -> u8 {
        ((self.0 >> 2) & 0xFF) as u8
    }

    pub fn cx(&self) -> u32 {
        self.0 & 0x0000_03FF
    }
}

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = self.opcode();
        match op {
            OP_HALT => write!(f, "halt  exit({})", self.a()),
            OP_LOAD_I => write!(f, "load_i R[{}] = {}", self.a(), self.bx_i32()),
            OP_ADD => write!(f, "add    R[{}] := R[{}] + R[{}]", self.a(), self.b(), self.c()),
            OP_SUB => write!(f, "sub    R[{}] := R[{}] - R[{}]", self.a(), self.b(), self.c()),
            OP_MUL => write!(f, "mul    R[{}] := R[{}] * R[{}]", self.a(), self.b(), self.c()),
            OP_DIV => write!(f, "div    R[{}] := R[{}] / R[{}]", self.a(), self.b(), self.c()),
            _ => write!(f, "{{{}}} {}", op, self.ax()),
        }
    }
}
