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
                }
                Instruction::Add => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    match (a.clone(), b.clone()) {
                        (Value::Number(a), Value::Number(b)) => {
                            self.stack.push(Value::Number(a + b));
                        }
                        _ => {
                            let a_str = a.to_string();
                            let b_str = b.to_string();
                            self.stack.push(Value::String(format!("{a_str}{b_str}")));
                        }
                    }
                }
                Instruction::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a - b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a * b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a / b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                }
                Instruction::Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a == b));
                }
                Instruction::Neq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a != b));
                }
                Instruction::Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a < b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a > b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Leq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a <= b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Geq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a >= b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                }
                Instruction::Pop => {
                    self.stack.pop();
                }
                Instruction::StoreVar(index) => {
                    let value = self.stack.pop().unwrap();
                    if *index < locals.len() {
                        locals[*index] = value;
                    }
                }
                Instruction::LoadVar(index) => {
                    let value = locals.get(*index).cloned().unwrap_or(Value::Undefined);
                    self.stack.push(value);
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
