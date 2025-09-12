use std::{cell::RefCell, rc::Rc};

use lumi_bytecode::{Bytecode, Constant, Env, Instruction};

use crate::{
    error::{VMError, VmResult},
    frame::Frame,
    stack::Stack,
    value::Value,
};

/// The virtual machine (VM) for the Lumi engine.
pub struct Vm {
    // TODO: add later
    // pub registers: Registers
    // pub heap: Heap
    pub env: Rc<RefCell<Env>>, // TODO: add instruction when entering a new scope and leaving a scope.
    pub stack: Stack,
    pub globals: Vec<Value>,
    pub locals: Vec<Value>,
}

impl Vm {
    pub fn new() -> Self {
        Vm {
            env: Rc::new(RefCell::new(Env::new(None))),
            stack: Stack::new(),
            globals: vec![Value::Undefined; 256],
            locals: vec![Value::Undefined; 16],
        }
    }

    pub fn execute(&mut self, bytecode: &Bytecode) -> VmResult<()> {
        let mut ip = 0; // instruction pointer
        let mut instructions = bytecode.instructions.clone();

        while ip < instructions.len() {
            match &instructions[ip] {
                Instruction::PushConst(idx) => {
                    let constant = bytecode
                        .constants
                        .get(*idx)
                        .cloned()
                        .unwrap_or(Constant::Undefined);
                    match constant {
                        Constant::Function(ref f) => {
                            self.env.borrow_mut().add_function(f.clone());
                        }
                        _ => self.stack.push(Stack::convert_constant_to_value(constant)),
                    }
                    ip += 1;
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
                    ip += 1;
                }
                Instruction::Sub => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a - b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                    ip += 1;
                }
                Instruction::Mul => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a * b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                    ip += 1;
                }
                Instruction::Div => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Number(a / b));
                    } else {
                        self.stack.push(Value::Number(f64::NAN));
                    }
                    ip += 1;
                }
                Instruction::Eq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a == b));
                    ip += 1;
                }
                Instruction::Neq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(Value::Boolean(a != b));
                    ip += 1;
                }
                Instruction::Lt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a < b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                    ip += 1;
                }
                Instruction::Gt => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a > b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                    ip += 1;
                }
                Instruction::Leq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a <= b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                    ip += 1;
                }
                Instruction::Geq => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    if let (Value::Number(a), Value::Number(b)) = (a, b) {
                        self.stack.push(Value::Boolean(a >= b));
                    } else {
                        self.stack.push(Value::Boolean(false));
                    }
                    ip += 1;
                }
                Instruction::JumpIfTrue(target) => {
                    let cond = self.stack.pop().unwrap();
                    if cond.as_bool().unwrap_or(false) {
                        ip = *target;
                        continue;
                    }
                    ip += 1;
                }
                Instruction::JumpIfFalse(target) => {
                    let cond = self.stack.pop().unwrap();
                    if !cond.as_bool().unwrap_or(false) {
                        ip = *target;
                        continue;
                    }
                    ip += 1;
                }
                Instruction::Jump(target) => {
                    ip = *target;
                    continue;
                }
                Instruction::Pop => {
                    self.stack.pop();
                    ip += 1;
                }
                Instruction::StoreVar(index) => {
                    let idx = *index;
                    if let Some(frame) = self.stack.frames.last_mut() {
                        if idx < frame.locals.len() {
                            frame.locals[idx] = self.stack.values.pop().unwrap();
                        }
                    } else {
                        self.locals[idx] = self.stack.values.pop().unwrap();
                    }
                    ip += 1;
                }
                Instruction::LoadVar(index) => {
                    let idx = *index;
                    if let Some(frame) = self.stack.frames.last() {
                        let val = frame.locals.get(idx).cloned().unwrap_or(Value::Undefined);
                        self.stack.push(val);
                    } else {
                        self.locals
                            .get(idx)
                            .cloned()
                            .map(|v| self.stack.push(v))
                            .unwrap_or_else(|| self.stack.push(Value::Undefined));
                    }
                    ip += 1;
                }
                Instruction::Print => {
                    let value = self.stack.peek().unwrap();
                    println!("{:?}", value.to_string());
                    ip += 1;
                }
                Instruction::CallFn(fn_name) => {
                    let function = self.env.borrow_mut().get_function(fn_name, false);
                    if function.is_none() {
                        return Err(VMError::function_not_found(fn_name));
                    }
                    let function = function.unwrap();
                    let argc = function.arity;

                    // Extract arguments from stack (in order)
                    let mut args = Vec::with_capacity(argc);
                    for _ in 0..argc {
                        args.push(self.stack.pop().unwrap());
                    }
                    args.reverse(); // ensure original order

                    let mut locals = vec![Value::Undefined; 16];
                    for (i, arg) in args.into_iter().enumerate() {
                        locals[i] = arg;
                    }

                    let return_ip = ip + 1;

                    self.stack.push_frame(Frame {
                        return_ip,
                        arg_count: argc,
                        base_pointer: 0, // NOTE: atm we don't use base_pointer since our functions don't live on the stack.
                        return_instructions: instructions.clone(),
                        locals: locals,
                    });

                    // Set the instructions to the functions instructions chunk and start from 0 again.
                    // The frame has a pointer and copy of the previous instruction set.
                    instructions = function.chunk.clone();
                    ip = 0;
                }
                Instruction::Return => {
                    let frame = self.stack.frames.pop().unwrap();

                    let ret = if self.stack.values.len() > frame.base_pointer + 1 {
                        self.stack.pop().unwrap()
                    } else {
                        Value::Undefined
                    };

                    // Leave the callee (function) on the stack.
                    self.stack.values.truncate(frame.base_pointer + 1);

                    // Push the return value
                    if ret != Value::Undefined {
                        self.stack.values.push(ret);
                    }

                    instructions = frame.return_instructions;
                    ip = frame.return_ip;
                }
                _ => unimplemented!(), // Placeholder for other instructions
            }
        }
        Ok(())
    }
}
