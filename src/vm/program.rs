use crate::vm::Error;

pub struct Program<'a> {
    code: &'a [u8],
    pc: usize,
}

impl<'a> Program<'a>  {
    pub fn new(code: &'a [u8]) -> Self {
        Self { code, pc: 0 }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn read_u8(&self) -> Result<u8, Error> {
        if self.pc >= self.code.len() {
            return Err(Error::EndOfProgram);
        }
        Ok(self.code[self.pc])
    }

    pub fn read_i8(&self) -> Result<i8, Error> {
        self.read_u8().map(|x| x as i8)
    }

    pub fn read_f64(&self) -> Result<f64, Error> {
        let chunk: [u8; 8] = match &self.code[self.pc..self.pc + 8].try_into() {
            Ok(chunk) => *chunk,
            Err(_) => return Err(Error::EndOfProgram),
        };
        Ok(f64::from_ne_bytes(chunk))
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
        let mut program = Program::new(&[0x01, 0x02, 0x03, 0xFF]);
        for _ in 0..256 {
            program.step();
        }
        assert_eq!(program.pc, 4);
    }

    #[test]
    fn stepping_changes_read_output() {
        let mut program = Program::new(&[0x01, 0x02, 0x03, 0xFF]);
        assert_eq!(program.read_u8(), Ok(0x01));
        assert_eq!(program.read_u8(), Ok(0x01));

        program.step();
        assert_eq!(program.read_u8(), Ok(0x02));

        program.step();
        assert_eq!(program.read_u8(), Ok(0x03));

        program.step();
        assert_eq!(program.read_u8(), Ok(0xFF));

        program.step();
        assert_eq!(program.read_u8(), Err(Error::EndOfProgram));
        assert_eq!(program.read_u8(), Err(Error::EndOfProgram));
    }
}