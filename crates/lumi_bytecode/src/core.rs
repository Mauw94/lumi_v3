use std::{cell::RefCell, rc::Rc};

use crate::{Constant, Instruction};

/// An environment for variable and function scopes
#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    pub functions: Vec<FunctionObj>,
    pub parent: Option<Rc<RefCell<Env>>>,
}

/// A function object
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionObj {
    pub name: Option<String>,
    pub arity: usize,
    pub chunk: Vec<Instruction>,
    pub constants: Vec<Constant>,
}

// NOTE: when creating a new Env, pass the current as the parent
// let env = Env::new(Some(Rc::clone(&self.env)));
impl Env {
    pub fn new(closure: Option<Rc<RefCell<Env>>>) -> Self {
        Env {
            functions: Vec::new(),
            parent: closure,
        }
    }

    pub fn add_function(&mut self, func: FunctionObj) -> usize {
        self.functions.push(func);
        self.functions.len() - 1 // Return the index of the newly added function
    }

    pub fn get_function(&self, name: &str, may_check_parent: bool) -> Option<FunctionObj> {
        for func in &self.functions {
            if let Some(ref func_name) = func.name {
                if func_name == name {
                    return Some(func.clone());
                }
            }
        }

        if !may_check_parent {
            return None;
        }

        // If not found in the current environment, check the parent
        if let Some(ref parent_env) = self.parent {
            return parent_env.borrow().get_function(name, may_check_parent);
        }

        None
    }
}
