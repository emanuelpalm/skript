use crate::vm::opcode;

pub struct Program<'a> {
    code: &'a [u8],
    pc: usize,
}

impl<'a> Program<'a>  {
    pub fn new(code: &'a [u8]) -> Self {
        Self { code, pc: 0 }
    }

    pub fn read(&self) -> u8 {
        if self.pc >= self.code.len() {
            return opcode::HALT;
        }
        self.code[self.pc]
    }

    pub fn step(&mut self) {
        if self.pc < self.code.len() {
            self.pc += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pc_stops_at_end_of_program() {
        let mut program = Program::new(&[0x01, 0x02, 0x03, 0xFF]);
        for _ in 0..256 {
            program.step();
        }
        assert_eq!(program.pc, 4);
    }

    #[test]
    fn stepping_changes_read_output() {
        let mut program = Program::new(&[0x01, 0x02, 0x03, 0xFF]);
        assert_eq!(program.read(), 0x01);
        assert_eq!(program.read(), 0x01);

        program.step();
        assert_eq!(program.read(), 0x02);

        program.step();
        assert_eq!(program.read(), 0x03);

        program.step();
        assert_eq!(program.read(), 0xFF);

        program.step();
        assert_eq!(program.read(), opcode::HALT);
        assert_eq!(program.read(), opcode::HALT);
    }
}