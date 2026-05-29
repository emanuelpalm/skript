use std::{fmt, ops};
use crate::rvm::instr::Instr;

#[derive(Eq, PartialEq)]
pub struct Code(Box<[Instr]>);

impl Code {
    pub fn new<T: Into<Box<[Instr]>>>(code: T) -> Self {
        Code(code.into())
    }
}

impl ops::Deref for Code {
    type Target = [Instr];

    fn deref(&self) -> &[Instr] {
        self.0.deref()
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, instr) in self.0.iter().enumerate() {
            writeln!(f, "{:4} {:?}", i, instr)?;
        }
        Ok(())
    }
}
