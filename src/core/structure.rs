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

    /// The kinds associated with the elements of this structure.
    fn element_kinds(&self) -> Vec<String> {
        self.elements().iter().map(|e| e.kind()).collect()
    }

    /// The values associated with the elements of this structure.
    fn element_values(&self) -> Vec<Value> {
        self.elements().iter().map(|e| e.value()).collect()
    }

    /// Returns the first element of the structure.
    fn first(&self) -> Expression {
        self.elements().first().unwrap().to_owned()
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

    /// The kinds associated with the flattened collection of sub-elements.
    fn flat_element_kinds(&self) -> Vec<String> {
        self.flat_elements().iter().map(|e| e.kind()).collect()
    }

    /// The values associated with the flattened collection of elements.
    fn flat_element_values(&self) -> Vec<Value> {
        self.flat_elements().iter().map(|e| e.value()).collect()
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

    /// Attempts to construct a tuple of the kinds associated with the first and
    /// last elements of the structure.
    fn kind_tuple(&self) -> (String, String) {
        let (x, y) = self.tuple();
        (x.kind(), y.kind())
    }

    /// Whether this structure is structurally identical to another.
    fn is_identical_to(&self, other: Expression) -> bool {
        self.value() == other.value()
    }

    /// A label associated with this structure.
    fn label(&self) -> Option<String> {
        None
    }

    /// Returns the last element of the structure.
    fn last(&self) -> Expression {
        self.elements().last().unwrap().to_owned()
    }

    /// Attempts to construct a tuple from the first and last elements of the
    /// structure.
    fn tuple(&self) -> (Expression, Expression) {
        let elements = self.elements();
        (elements.first().unwrap().to_owned(), elements.last().unwrap().to_owned())
    }

    /// Represents the external (rust) value associated with the structure.
    fn value(&self) -> Value {
        Value::None
    }

    /// Attempts to construct a tuple of the values associated with the first
    /// and last elements of the structure.
    fn value_tuple(&self) -> (Value, Value) {
        let (x, y) = self.tuple();
        (x.value(), y.value())
    }
}
