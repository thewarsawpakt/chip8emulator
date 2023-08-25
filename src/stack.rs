use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Stack<T>
where
    T: Display,
{
    capacity: usize,
    buffer: Vec<T>,
}

impl<T: Display + Debug> Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.buffer)
    }
}

pub struct StackOverflowError;

impl<T: Display> Stack<T> {
    pub fn push(&mut self, item: T) -> Result<usize, StackOverflowError> {
        if self.buffer.len() == self.capacity {
            Err(StackOverflowError)
        } else {
            self.buffer.push(item);
            Ok(self.buffer.len())
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        self.buffer.pop()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Stack {
            capacity: capacity,
            buffer: Vec::new(),
        }
    }
}
