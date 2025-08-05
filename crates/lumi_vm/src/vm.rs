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
                        // TODO: proper error handling
                        _ => panic!("Invalid types for addition: {:?} and {:?}", a, b),
                    }
                }
                Instruction::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a.clone(), b.clone()) {
                        (Value::Number(a_num), Value::Number(b_num)) => {
                            self.stack.push(Value::Number(a_num - b_num));
                        }
                        // TODO: proper error handling
                        _ => panic!("Invalid types for subtraction: {:?} and {:?}", a, b),
                    }
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a.clone(), b.clone()) {
                        (Value::Number(a_num), Value::Number(b_num)) => {
                            self.stack.push(Value::Number(a_num * b_num));
                        }
                        // TODO: proper error handling
                        _ => panic!("Invalid types for multiplication: {:?} and {:?}", a, b),
                    }
                }
                Instruction::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a.clone(), b.clone()) {
                        (Value::Number(a_num), Value::Number(b_num)) => {
                            if b_num == 0.0 {
                                // TODO: proper error handling
                                panic!("Division by zero");
                            }
                            self.stack.push(Value::Number(a_num / b_num));
                        }
                        // TODO: proper error handling
                        _ => panic!("Invalid types for division: {:?} and {:?}", a, b),
                    }
                }
                Instruction::StoreVar(index) => {
                    let value = self.stack.pop().unwrap();
                    if *index < locals.len() {
                        locals[*index] = value;
                    } else {
                        // TODO: proper error handling
                        panic!("Variable index out of bounds: {}", index);
                    }
                }
                Instruction::LoadVar(index) => {
                    if *index < locals.len() {
                        let value = locals[*index].clone();
                        self.stack.push(value);
                    } else {
                        // TODO: proper error handling
                        panic!("Variable index out of bounds: {}", index);
                    }
                }
                Instruction::Print => {
                    let value = self.stack.peek().unwrap();
                    println!("{:?}", value);
                }
                _ => unimplemented!(), // Placeholder for other instructions
            }

            ip += 1; // Move to the next instruction
        }
    }
}
