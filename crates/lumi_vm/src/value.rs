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

    pub fn to_string(&self) -> String {
        match self {
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            Value::Boolean(b) => b.to_string(),
            Value::Object(_) => "[object]".to_string(),
            Value::Array(_) => "[array]".to_string(),
            Value::Function(_) => "[function]".to_string(),
            Value::Null => "null".to_string(),
            Value::Undefined => "undefined".to_string(),
        }
    }
    // TODO: add more methods for other types as needed
}
