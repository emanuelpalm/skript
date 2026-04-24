mod opcode;
mod program;
mod stack;
mod error;

use program::Program;
use stack::Stack;
use crate::vm::error::Error;

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
        let op = self.program.read();
        match op {
            opcode::HALT => {
                self.running = false;
            },
            opcode::ADD => {
                let rhs = self.stack.pop()?;
                let lhs = self.stack.pop()?;
                let res = lhs + rhs;
                self.stack.push(res)?;
                self.program.step();
            },
            opcode::SUB => {
                let rhs = self.stack.pop()?;
                let lhs = self.stack.pop()?;
                let res = lhs - rhs;
                self.stack.push(res)?;
                self.program.step();
            },
            opcode::MUL => {
                let rhs = self.stack.pop()?;
                let lhs = self.stack.pop()?;
                let res = lhs * rhs;
                self.stack.push(res)?;
                self.program.step();
            },
            opcode::DIV => {
                let rhs = self.stack.pop()?;
                let lhs = self.stack.pop()?;
                let res = lhs / rhs;
                self.stack.push(res)?;
                self.program.step();
            },
            opcode::PUSH_I8 => {
                self.program.step();
                let value = self.program.read() as i8 as f64;
                self.program.step();
                self.stack.push(value)?;
            }
            _ => panic!("Unsupported opcode {}", op),
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
            opcode::PUSH_I8, 5,
            opcode::PUSH_I8, 4,
            opcode::ADD,

            opcode::PUSH_I8, 3,
            opcode::SUB,

            opcode::PUSH_I8, 2,
            opcode::MUL,

            opcode::PUSH_I8, 1,
            opcode::DIV,
        ]);

        let res = vm.run();
        assert_eq!(res, Ok(12.0));
    }
}
