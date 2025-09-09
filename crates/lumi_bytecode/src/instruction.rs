use crate::core::FunctionObj;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Label(pub usize);

#[derive(Debug)]
pub enum PendingJump {
    Jump(usize),
    JumpIfFalse(usize),
}

/// This module defines the bytecode instructions for the Lumi engine.
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PushConst(usize), // Push a constant value onto the stack
    Pop,              // Pop the top value from the stack
    Add,
    // Arithmetic operations
    Sub,
    Mul,
    Div,
    Mod,
    Inc,
    Dec,
    Eq,
    Neq,
    Lt,
    Gt,
    Leq,
    Geq,                // Comparison operations
    Jump(usize),        // Unconditional jump to a specific instruction index
    JumpIfTrue(usize),  // Conditional jump if the top value is true
    JumpIfFalse(usize), // Conditional jump if the top value is false
    CallFn(String),     // Call a function with a given name
    Return,             // Return from the current function
    LoadVar(usize),     // Load a variable by index
    StoreVar(usize),    // Store the top value into a variable by index
    Print,              // Print the top value on the stack
    Nop,                // No operation (used for padding or alignment)
    Halt,               // Stop execution
}

/// Represents a constant pool for bytecode instructions
#[derive(Debug, Default)]
pub struct ConstantPool {
    pub values: Vec<Constant>,
}

impl ConstantPool {
    pub fn add(&mut self, constant: Constant) -> usize {
        self.values.push(constant);
        self.values.len() - 1 // Return the index of the newly added constant
    }
}

/// Represents a constant value in the bytecode
#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Number(f64),
    String(String),
    Boolean(bool),
    Function(FunctionObj),
    Null,
    Undefined,
}
