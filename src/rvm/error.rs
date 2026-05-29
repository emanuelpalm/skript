use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    EndOfProgram,
    InvalidOpcode {
        opcode: u8,
        pc: usize,
    },
    NoReturnValue,
    StackMissed,
    StackOverflow,
    StackUnderflow,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}