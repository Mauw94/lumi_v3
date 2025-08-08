use lumi_bytecode::Instruction;

use crate::Value;

/// Frame management for the Lumi virtual machine.
#[derive(Debug, Clone)]
pub struct Frame {
    pub return_ip: usize,
    pub base_pointer: usize,
    pub arg_count: usize,
    pub return_instructions: Vec<Instruction>,
    pub locals: Vec<Value>,
}

impl Frame {
    pub fn new() -> Self {
        Frame {
            return_ip: 0,
            base_pointer: 0,
            arg_count: 0,
            return_instructions: Vec::new(),
            locals: Vec::new(),
        }
    }
}
