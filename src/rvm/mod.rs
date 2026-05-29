pub mod opcode;

mod code;
mod error;
mod instr;
mod program;
mod stack;

pub use code::*;
pub use error::*;
pub use instr::*;

use opcode::*;
use program::Program;
use stack::Stack;

pub struct VirtualMachine<'a> {
    program: Program<'a>,
    running: bool,
    stack: Stack,
    retval: Option<f64>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(code: &'a [Instr]) -> Self {
        Self {
            program: Program::new(code),
            running: false,
            stack: Stack::new(),
            retval: None,
        }
    }

    pub fn run(&mut self) -> Result<f64, Error> {
        self.running = true;
        while self.running {
            self.step()?;
        }
        self.retval.ok_or(Error::NoReturnValue)
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let instr = self.program.read()?;
        self.program.step();
        match instr.opcode() {
            OP_HALT => {
                self.running = false;
            }
            OP_LOAD_I => {
                let ra = instr.a();
                let bx = instr.bx_i32();
                self.stack.set(ra as usize, bx as f64)?;
            }
            OP_RET => {
                let ra = instr.a();
                let retval = self.stack.get(ra as usize)?;
                self.retval = Some(retval);
                self.running = false;
            }
            OP_ADD => {
                let ra = instr.a();
                let rb = instr.b();
                let rc = instr.c();
                let b = self.stack.get(rb as usize)?;
                let c = self.stack.get(rc as usize)?;
                self.stack.set(ra as usize, b + c)?;
            }
            OP_SUB => {
                let ra = instr.a();
                let b = self.stack.get(instr.b() as usize)?;
                let c = self.stack.get(instr.c() as usize)?;
                self.stack.set(ra as usize, b - c)?;
            }
            OP_MUL => {
                let ra = instr.a();
                let b = self.stack.get(instr.b() as usize)?;
                let c = self.stack.get(instr.c() as usize)?;
                self.stack.set(ra as usize, b * c)?;
            }
            OP_DIV => {
                let ra = instr.a();
                let b = self.stack.get(instr.b() as usize)?;
                let c = self.stack.get(instr.c() as usize)?;
                self.stack.set(ra as usize, b / c)?;
            }
            _ => {
                return Err(Error::InvalidOpcode {
                    opcode: instr.opcode(),
                    pc: self.program.pc() - 1,
                });
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_simple_arithmetic_program_correctly() {
        // (((5 + 4) - 3) * 2) / 1
        let code = [
            Instr::from_a_bx(OP_LOAD_I, 0, 1),
            Instr::from_a_bx(OP_LOAD_I, 1, 2),
            Instr::from_a_bx(OP_LOAD_I, 2, 3),
            Instr::from_a_bx(OP_LOAD_I, 3, 4),
            Instr::from_a_bx(OP_LOAD_I, 4, 5),
            Instr::from_a_b_c(OP_ADD, 5, 3, 4),
            Instr::from_a_b_c(OP_SUB, 5, 5, 2),
            Instr::from_a_b_c(OP_MUL, 5, 5, 1),
            Instr::from_a_b_c(OP_DIV, 5, 5, 0),
            Instr::from_a_b_c(OP_RET, 5, 0, 0),
        ];
        let mut vm = VirtualMachine::new(&code);
        let res = vm.run();
        assert_eq!(res, Ok(12.0));
    }

    #[test]
    fn produces_error_when_encountering_invalid_opcode() {
        let code = [Instr::new(0xFFFF_FFFF)];
        let mut vm = VirtualMachine::new(&code);
        assert_eq!(
            vm.run(),
            Err(Error::InvalidOpcode {
                opcode: 0x3F,
                pc: 0
            })
        );
    }
}
