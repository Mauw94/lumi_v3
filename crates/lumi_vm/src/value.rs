use crate::heap::HandleId;

/// Represents a value in the Lumi VM.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Object(HandleId),
    Array(HandleId),
    Function(HandleId),
    Null,
    Undefined,
}

impl Value {
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }

    pub fn as_number(&self) -> Option<f64> {
        if let Value::Number(n) = self {
            Some(*n)
        } else {
            None
        }
    }
    // TODO: add more methods for other types as needed
}
