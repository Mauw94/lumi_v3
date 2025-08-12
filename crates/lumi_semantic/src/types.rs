use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Represents a Lumi type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    /// Undefined type
    Undefined,

    /// Null type
    Null,

    /// Boolean type
    Boolean,

    /// Number type (including NaN and Infinity)
    Number,

    /// String type
    String,

    /// Symbol type
    Symbol,

    /// Object type
    Object,

    /// Array type
    Array(Box<Type>),

    /// Function type with parameter types and return type
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },

    /// Union type (e.g., string | number)
    Union(Vec<Type>),

    /// Any type (can be anything)
    Any,

    /// Type is unkown
    Unknown,
}

impl Type {
    /// Check if this type is compatible with another type
    pub fn is_compatible_with(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Any, _) | (_, Type::Any) => true,
            (Type::Union(types), other) => types.iter().any(|t| t.is_compatible_with(other)),
            (Type::Array(inner1), Type::Array(inner2)) => inner1.is_compatible_with(inner2),
            (
                Type::Function {
                    params: p1,
                    return_type: r1,
                },
                Type::Function {
                    params: p2,
                    return_type: r2,
                },
            ) => {
                p1.len() == p2.len()
                    && p1
                        .iter()
                        .zip(p2.iter())
                        .all(|(a, b)| a.is_compatible_with(b))
                    && r1.is_compatible_with(r2)
            }
            (Type::Boolean, Type::Boolean) => true,
            _ => self == other,
        }
    }

    /// Get the most specific common type between two types
    pub fn common_type(&self, other: &Type) -> Type {
        if self.is_compatible_with(other) {
            self.clone()
        } else if other.is_compatible_with(self) {
            other.clone()
        } else {
            Type::Union(vec![self.clone(), other.clone()])
        }
    }

    /// Check if this type is a primitive type
    pub fn is_primitive(&self) -> bool {
        matches!(
            self,
            Type::Undefined
                | Type::Null
                | Type::Boolean
                | Type::Number
                | Type::String
                | Type::Symbol
        )
    }

    /// Check if this type is an object type
    pub fn is_object(&self) -> bool {
        matches!(self, Type::Object | Type::Array(_) | Type::Function { .. })
    }

    pub fn to_string(&self) -> String {
        match self {
            Type::Undefined => "undefined".to_string(),
            Type::Null => "null".to_string(),
            Type::Boolean => "boolean".to_string(),
            Type::Number => "number".to_string(),
            Type::String => "string".to_string(),
            Type::Symbol => "symbol".to_string(),
            Type::Object => "object".to_string(),
            Type::Array(inner) => format!("Array<{}>", inner.to_string()),
            Type::Function {
                params,
                return_type,
            } => {
                let params_str: Vec<String> = params.iter().map(|p| p.to_string()).collect();
                format!(
                    "Function<({}) -> {}>",
                    params_str.join(", "),
                    return_type.to_string()
                )
            }
            Type::Union(types) => {
                let types_str: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                types_str.join(" | ")
            }
            Type::Any => "any".to_string(),
            Type::Unknown => "unkown".to_string(),
        }
    }
}

impl Default for Type {
    fn default() -> Self {
        Type::Any
    }
}

/// Type environment for tracking variable types
#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    types: HashMap<String, Type>,
}

impl TypeEnvironment {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
        }
    }

    /// Declare a variable with a specific type
    pub fn declare(&mut self, name: &str, type_info: Type) {
        self.types.insert(name.to_string(), type_info);
    }

    /// Get the type of a variable
    pub fn get_type(&self, name: &str) -> Option<&Type> {
        self.types.get(name)
    }

    /// Check if a variable is declared
    pub fn is_declared(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }

    /// Update the type of an existing variable
    pub fn update_type(&mut self, name: &str, type_info: Type) -> bool {
        if self.types.contains_key(name) {
            self.types.insert(name.to_string(), type_info);
            true
        } else {
            false
        }
    }
}
