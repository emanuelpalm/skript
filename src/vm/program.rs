use crate::vm::instr::Instr;
use crate::vm::{Code, Error};

pub struct Program<'a> {
    code: &'a [Instr],
    pc: usize,
}

impl<'a> Program<'a> {
    pub fn new(code: &'a [Instr]) -> Self {
        Self { code, pc: 0 }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn read(&self) -> Result<Instr, Error> {
        if self.pc >= self.code.len() {
            return Err(Error::EndOfProgram);
        }
        Ok(self.code[self.pc])
    }

    pub fn step(&mut self) {
        if self.pc < self.code.len() {
            self.pc += 1;
        }
    }

    pub fn step_n(&mut self, n: usize) {
        if self.pc + n <= self.code.len() {
            self.pc += n;
        } else {
            self.pc = self.code.len();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pc_stops_at_end_of_program() {
        let code = [Instr::new(1), Instr::new(2), Instr::new(3), Instr::new(255)];
        let mut program = Program::new(&code);
        for _ in 0..256 {
            program.step();
        }
        assert_eq!(program.pc, 3);
    }

    #[test]
    fn stepping_changes_read_output() {
        let code = [Instr::new(1), Instr::new(2), Instr::new(3), Instr::new(255)];
        let mut program = Program::new(&code);
        assert_eq!(program.read(), Ok(Instr::new(0x01)));
        assert_eq!(program.read(), Ok(Instr::new(0x01)));

        program.step();
        assert_eq!(program.read(), Ok(Instr::new(0x02)));

        program.step();
        assert_eq!(program.read(), Ok(Instr::new(0x03)));

        program.step();
        assert_eq!(program.read(), Ok(Instr::new(0xFF)));

        program.step();
        assert_eq!(program.read(), Err(Error::EndOfProgram));
        assert_eq!(program.read(), Err(Error::EndOfProgram));
    }
}
