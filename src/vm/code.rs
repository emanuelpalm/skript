use crate::vm::opcode;
use std::{fmt, ops};

#[derive(Eq, PartialEq)]
pub struct Code(Box<[u8]>);

impl Code {
    pub fn new<T: Into<Box<[u8]>>>(code: T) -> Self {
        Code(code.into())
    }
}

impl ops::Deref for Code {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl fmt::Debug for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code: &[u8] = &self.0;
        let mut i = 0;
        while i < code.len() {
            let s = match code[i] {
                opcode::HALT => "HALT",
                opcode::ADD => "ADD",
                opcode::SUB => "SUB",
                opcode::MUL => "MUL",
                opcode::DIV => "DIV",
                opcode::PUSH_I8 => {
                    let value = code
                        .get(i + 1)
                        .cloned()
                        .map(|x| x as i8)
                        .ok_or(fmt::Error)?;
                    writeln!(f, "PUSH_I8 {}", value)?;
                    i += 2;
                    continue;
                }
                opcode::PUSH_F64 => {
                    let chunk: [u8; 8] = match &code[i..i + 8].try_into() {
                        Ok(chunk) => *chunk,
                        Err(_) => return Err(fmt::Error),
                    };
                    writeln!(f, "PUSH_F64 {}", f64::from_ne_bytes(chunk))?;
                    i += 9;
                    continue;
                }
                opcode => {
                    writeln!(f, "<{:02X}>", opcode)?;
                    i += 1;
                    continue;
                }
            };
            writeln!(f, "{}", s)?;
            i += 1;
        }
        Ok(())
    }
}
