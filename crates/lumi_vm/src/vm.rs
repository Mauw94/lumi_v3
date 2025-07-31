use lumi_bytecode::{Bytecode, Constant, Instruction};

use crate::{frame::Frame, heap::Heap, stack::Stack, value::Value};

/// The virtual machine (VM) for the Lumi engine.
pub struct Vm {
    pub stack: Stack,
    pub frame: Frame,
    // pub registers: Registers // TODO: add later
    pub heap: Heap,
    pub globals: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            stack: Stack::new(),
            frame: Frame::new(),
            heap: Heap::new(),
            globals: Vec::new(),
        }
    }

    // TODO: add Result<Value, VMError> add VMError
    // TODO: VmResult<T>
    pub fn execute(&mut self, bytecode: &Bytecode) {
        let mut ip = 0; // instruction pointer
        let mut locals = vec![Value::Undefined; 256]; // local variables, size can be adjusted

        while ip < bytecode.instructions.len() {
            match &bytecode.instructions[ip] {
                // Handle each instruction type here
                Instruction::PushConst(idx) => {
                    let value = bytecode
                        .constants
                        .get(*idx)
                        .cloned()
                        .unwrap_or(Constant::Undefined);
                    // TODO: move to a method in Stack
                    // Convert Constant to Value and push onto the stack
                    match value {
                        Constant::Number(num) => self.stack.push(Value::Number(num)),
                        Constant::String(s) => self.stack.push(Value::String(s)),
                        Constant::Boolean(b) => self.stack.push(Value::Boolean(b)),
                        Constant::Null => self.stack.push(Value::Null),
                        Constant::Undefined => self.stack.push(Value::Undefined),
                    }
                    // self.stack.push(value);
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a.clone(), b.clone()) {
                        (Value::Number(a_num), Value::Number(b_num)) => {
                            self.stack.push(Value::Number(a_num + b_num));
                        }
                        _ => panic!("Invalid types for addition: {:?} and {:?}", a, b),
                    }
                }
                _ => unimplemented!(), // Placeholder for other instructions
            }

            ip += 1; // Move to the next instruction
        }
    }
}
