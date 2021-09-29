//! Mathematical Structures

use crate::core::*;

/// Represents a mathematical structure.
pub trait Structure: std::fmt::Debug + std::fmt::Display {
    /// The collection of attributes associated with this expression.
    fn attributes(&self) -> Attributes {
        Attributes::new()
    }

    /// The collection of sub-elements making up this structure.
    fn elements(&self) -> Expressions {
        match self.value() {
            Value::BinaryOperation(_, x, y) => vec![x, y],
            Value::NaryOperation(_, v) => v,
            Value::TernaryOperation(_, x, y, z) => vec![x, y, z],
            Value::UnaryOperation(_, x) => vec![x],
            _ => Vec::new()
        }
    }

    /// The flattened collection of sub-elements making up this structure.
    fn flat_elements(&self) -> Expressions {
        let mut result = Expressions::new();
        for element in self.elements() {
            if element.kind() == self.kind() {
                result.extend(element.flat_elements());
            } else {
                result.push(element);
            }
        }
        result
    }

    /// The kind associated with the structure, excluding subtypes.
    fn kind(&self) -> String {
        match self.value() {
            Value::BinaryOperation(k, _, _)       => k.clone(),
            Value::Boolean(_)                     => "Boolean".to_string(),
            Value::Float(_)                       => "Float".to_string(),
            Value::Integer(_)                     => "Integer".to_string(),
            Value::Kind(k)                        => k.clone(),
            Value::NaryOperation(k, _)            => k.clone(),
            Value::String(_)                      => "String".to_string(),
            Value::Symbol(_)                      => "Symbol".to_string(),
            Value::TernaryOperation(k, _, _, _)   => k.clone(),
            Value::UnaryOperation(k, _)           => k.clone(),
            _ => "None".to_string()
        }
    }

    /// Whether this structure is structurally identical to another.
    fn is_identical_to(&self, other: Expression) -> bool {
        self.value() == other.value()
    }

    /// A label associated with this structure.
    fn label(&self) -> Option<String> {
        None
    }

    /// Represents the external (rust) value associated with the structure.
    fn value(&self) -> Value {
        Value::None
    }
}
