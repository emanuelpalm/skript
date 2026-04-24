#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvalidOpcode {
        opcode: u8,
        pc: usize,
    },
    StackOverflow,
    StackUnderflow,
}
