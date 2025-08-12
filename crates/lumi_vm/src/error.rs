use thiserror::Error;

use crate::Value;

pub type VmResult<T> = Result<T, VMError>;

#[derive(Clone, Error, Debug, PartialEq)]
pub enum VMError {
    #[error("Callee is not a function: {callee}")]
    CalleeIsNotFunction { callee: String },
}

impl VMError {
    pub fn callee_is_not_a_function(callee: Value) -> Self {
        VMError::CalleeIsNotFunction {
            callee: callee.to_string(),
        }
    }
}
