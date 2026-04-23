mod opcode;
mod program;
mod stack;

use program::Program;
use stack::Stack;

pub struct VirtualMachine<'a> {
    program: Program<'a>,
    stack: Stack,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(code: &'a [u8]) -> Self {
        Self {
            program: Program::new(code),
            stack: Stack::new(),
        }
    }

    pub fn step(&mut self) {
        let op = self.program.read();
        match op {
            opcode::ADD => {
                let rhs = self.stack.pop();
                let lhs = self.stack.pop();
                let res = lhs + rhs;
                self.stack.push(res);
            },
            opcode::SUB => {
                let rhs = self.stack.pop();
                let lhs = self.stack.pop();
                let res = lhs - rhs;
                self.stack.push(res);
            },
            opcode::MUL => {
                let rhs = self.stack.pop();
                let lhs = self.stack.pop();
                let res = lhs * rhs;
                self.stack.push(res);
            },
            opcode::DIV => {
                let rhs = self.stack.pop();
                let lhs = self.stack.pop();
                let res = lhs / rhs;
                self.stack.push(res);
            }
            _ => panic!("Unsupported opcode {}", op),
        }
    }
}