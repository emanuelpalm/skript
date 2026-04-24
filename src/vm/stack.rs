use crate::vm::error::Error;

const STACK_CAPACITY: usize = 256;

pub struct Stack {
    buffer: [f64; STACK_CAPACITY],
    top: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self { buffer: [f64::NAN; _], top: 0 }
    }

    pub fn pop(&mut self) -> Result<f64, Error> {
        if self.top > 0 {
            self.top -= 1;
            Ok(self.buffer[self.top])
        } else {
            Err(Error::StackUnderflow)
        }
    }

    pub fn push(&mut self, value: f64) -> Result<(), Error> {
        if self.top < self.buffer.len() {
            self.buffer[self.top] = value;
            self.top += 1;
            Ok(())
        } else {
            Err(Error::StackOverflow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1.0).unwrap();
        let value = stack.pop();
        assert_eq!(value, Ok(1.0));
    }

    #[test]
    fn push_and_pop_multiple() {
        let mut stack = Stack::new();

        stack.push(1.0).unwrap();
        stack.push(2.0).unwrap();

        let value0 = stack.pop();
        assert_eq!(value0, Ok(2.0));

        stack.push(3.0).unwrap();

        let value1 = stack.pop();
        assert_eq!(value1, Ok(3.0));

        let value2 = stack.pop();
        assert_eq!(value2, Ok(1.0));
    }

    #[test]
    fn popping_from_empty_stack_produces_none() {
        let mut stack = Stack::new();
        assert_eq!(stack.pop(), Err(Error::StackUnderflow));
    }

    #[test]
    fn pushing_to_full_stack_produces_none() {
        let mut stack = Stack::new();
        for i in 0..STACK_CAPACITY {
            stack.push(i as f64)
                .expect("expected available capacity");
        }
        assert_eq!(stack.push(256.0), Err(Error::StackOverflow));
    }
}