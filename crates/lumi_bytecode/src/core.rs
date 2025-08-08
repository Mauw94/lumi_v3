use crate::{Constant, Instruction};

/// A function object
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionObj {
    pub name: Option<String>,
    pub arity: usize,
    pub chunk: Vec<Instruction>,
    pub constants: Vec<Constant>,
}
