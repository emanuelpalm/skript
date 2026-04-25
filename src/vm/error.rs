#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    EndOfProgram,
    InvalidOpcode {
        opcode: u8,
        pc: usize,
    },
    StackOverflow,
    StackUnderflow,
}
