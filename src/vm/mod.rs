pub mod opcode;

mod code;
mod error;
mod program;
mod stack;

pub use code::*;
pub use error::*;

use program::Program;
use stack::Stack;

pub struct VirtualMachine<'a> {
    program: Program<'a>,
    running: bool,
    stack: Stack,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self {
            program: Program::new(code),
            running: false,
            stack: Stack::new(),
        }
    }

    pub fn run(&mut self) -> Result<f64, Error> {
        self.running = true;
        while self.running {
            self.step()?;
        }
        self.stack.pop()
    }

    pub fn step(&mut self) -> Result<(), Error> {
        let op = self.program.read_u8()?;
        self.program.step();
        match op {
            opcode::HALT => {
                self.running = false;
            }
            opcode::ADD => {
                let lhs = self.stack.pop()?;
                let rhs = self.stack.pop()?;
                let res = lhs + rhs;
                self.stack.push(res)?;
            }
            opcode::SUB => {
                let lhs = self.stack.pop()?;
                let rhs = self.stack.pop()?;
                let res = lhs - rhs;
                self.stack.push(res)?;
            }
            opcode::MUL => {
                let lhs = self.stack.pop()?;
                let rhs = self.stack.pop()?;
                let res = lhs * rhs;
                self.stack.push(res)?;
            }
            opcode::DIV => {
                let lhs = self.stack.pop()?;
                let rhs = self.stack.pop()?;
                let res = lhs / rhs;
                self.stack.push(res)?;
            }
            opcode::PUSH_I8 => {
                let value = self.program.read_i8()?;
                self.program.step();
                self.stack.push(value as f64)?;
            }
            opcode::PUSH_F64 => {
                let value = self.program.read_f64()?;
                self.program.step_n(8);
                self.stack.push(value)?;
            }
            _ => {
                return Err(Error::InvalidOpcode {
                    opcode: op,
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
        let mut vm = VirtualMachine::new(&[
            opcode::PUSH_I8,
            1,
            opcode::PUSH_I8,
            2,
            opcode::PUSH_I8,
            3,
            opcode::PUSH_I8,
            4,
            opcode::PUSH_I8,
            5,
            opcode::ADD,
            opcode::SUB,
            opcode::MUL,
            opcode::DIV,
            opcode::HALT,
        ]);
        let res = vm.run();
        assert_eq!(res, Ok(12.0));
    }

    #[test]
    fn x() {
        // 100 + 4 * 5
        let mut vm = VirtualMachine::new(&[
            opcode::PUSH_I8,
            5,
            opcode::PUSH_I8,
            4,
            opcode::MUL,
            opcode::PUSH_I8,
            100,
            opcode::ADD,
            opcode::HALT,
        ]);
        let res = vm.run();
        assert_eq!(res, Ok(120.0));
    }

    #[test]
    fn computes_push_f64_correctly() {
        let v = 1.2e34f64.to_ne_bytes();
        let code = [
            opcode::PUSH_F64,
            v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7],
            opcode::HALT
        ];
        let mut vm = VirtualMachine::new(&code);
        assert_eq!(vm.run(), Ok(1.2e34));
    }

    #[test]
    fn produces_error_when_encountering_invalid_opcode() {
        let mut vm = VirtualMachine::new(&[0xFF]);
        assert_eq!(
            vm.run(),
            Err(Error::InvalidOpcode {
                opcode: 0xFF,
                pc: 0
            })
        );
    }
}
