use crate::rvm::error::Error;

const STACK_CAPACITY: usize = 256;

pub struct Stack {
    buffer: [f64; STACK_CAPACITY],
}

impl Stack {
    pub fn new() -> Self {
        Self {
            buffer: [f64::NAN; _],
        }
    }

    pub fn get(&self, index: usize) -> Result<f64, Error> {
        if index >= self.buffer.len() {
            return Err(Error::StackMissed);
        }
        Ok(self.buffer[index])
    }

    pub fn set(&mut self, index: usize, value: f64) -> Result<(), Error> {
        if index >= self.buffer.len() {
            return Err(Error::StackMissed);
        }
        self.buffer[index] = value;
        Ok(())
    }
}
