use crate::{frame::Frame, value::Value};

/// Stack is a structure that holds the values and frames for the virtual machine.
#[derive(Debug, Clone)]
pub struct Stack {
    pub values: Vec<Value>,
    pub frames: Vec<Frame>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            values: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn push(&mut self, value: Value) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> Option<Value> {
        self.values.pop()
    }

    pub fn peek(&self) -> Option<&Value> {
        self.values.last()
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frames.pop()
    }
}
