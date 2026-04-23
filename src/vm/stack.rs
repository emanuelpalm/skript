pub struct Stack {
    buffer: [f64; 256],
    top: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self { buffer: [f64::NAN; 256], top: 0 }
    }

    pub fn pop(&mut self) -> f64 {
        if self.top > 0 {
            self.top -= 1;
            self.buffer[self.top]
        } else {
            panic!("Stack underflow")
        }
    }

    pub fn push(&mut self, value: f64) {
        if self.top < self.buffer.len() {
            self.buffer[self.top] = value;
            self.top += 1;
        } else {
            panic!("Stack overflow")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_and_pop() {
        let mut stack = Stack::new();
        stack.push(1.0);
        let value = stack.pop();
        assert_eq!(value, 1.0);
    }

    #[test]
    fn push_and_pop_multiple() {
        let mut stack = Stack::new();

        stack.push(1.0);
        stack.push(2.0);

        let value0 = stack.pop();
        assert_eq!(value0, 2.0);

        stack.push(3.0);

        let value1 = stack.pop();
        assert_eq!(value1, 3.0);

        let value2 = stack.pop();
        assert_eq!(value2, 1.0);
    }

    #[test]
    #[should_panic]
    fn pop_empty_stack_panics() {
        let mut stack = Stack::new();
        stack.pop();
    }

    #[test]
    #[should_panic]
    fn push_full_stack_panics() {
        let mut stack = Stack::new();
        for i in 0..=256 {
            stack.push(i as f64);
        }
    }
}