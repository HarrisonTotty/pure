//! Mathematical Structures

use crate::core::*;

/// Represents a mathematical structure.
pub trait Structure: std::fmt::Debug + std::fmt::Display {
    /// The collection of sub-elements making up this structure.
    fn elements(&self) -> Option<Expressions> {
        match self.value() {
            Value::BinaryOperation(_, x, y) => Some(vec![x, y]),
            Value::NaryOperation(_, v) => Some(v),
            Value::TernaryOperation(_, x, y, z) => Some(vec![x, y, z]),
            Value::UnaryOperation(_, x) => Some(vec![x]),
            _ => None
        }
    }

    /// The kinds associated with the elements of this structure.
    fn element_kinds(&self) -> Option<Vec<String>> {
        if let Some(elements) = self.elements() {
            Some(elements.iter().map(|e| e.kind()).collect())
        } else {
            None
        }
    }

    /// The values associated with the elements of this structure.
    fn element_values(&self) -> Option<Vec<Value>> {
        if let Some(elements) = self.elements() {
            Some(elements.iter().map(|e| e.value()).collect())
        } else {
            None
        }
    }

    /// Returns the first element of the structure.
    fn first(&self) -> Option<Expression> {
        if let Some(elements) = self.elements() {
            Some(elements.first().unwrap().to_owned())
        } else {
            None
        }
    }

    /// The flattened collection of sub-elements making up this structure.
    fn flat_elements(&self) -> Option<Expressions> {
        if let Some(elements) = self.elements() {
            let mut result = Expressions::new();
            for element in elements {
                if element.kind() == self.kind() {
                    if let Some(element_flat_elements) = element.flat_elements() {
                        result.extend(element_flat_elements);
                    } else {
                        result.push(element);
                    }
                } else {
                    result.push(element);
                }
            }
            Some(result)
        } else {
            None
        }
    }

    /// The kinds associated with the flattened collection of sub-elements.
    fn flat_element_kinds(&self) -> Option<Vec<String>> {
        if let Some(flat_elements) = self.flat_elements() {
            Some(flat_elements.iter().map(|e| e.kind()).collect())
        } else {
            None
        }
    }

    /// The values associated with the flattened collection of elements.
    fn flat_element_values(&self) -> Option<Vec<Value>> {
        if let Some(flat_elements) = self.flat_elements() {
            Some(flat_elements.iter().map(|e| e.value()).collect())
        } else {
            None
        }
    }

    /// Creates a new expression containing this structure from the specified
    /// collection of sub-elements.
    fn from_elements(&self, elements: Expressions) -> Expression;

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
    fn kind_tuple(&self) -> Option<(String, String)> {
        if let Some((x, y)) = self.tuple() {
            Some((x.kind(), y.kind()))
        } else {
            None
        }
    }

    /// Whether this structure is structurally identical to another.
    fn is_identical_to(&self, other: &Expression) -> bool {
        self.value() == other.value()
    }

    /// Returns the last element of the structure.
    fn last(&self) -> Option<Expression> {
        if let Some(elements) = self.elements() {
            Some(elements.last().unwrap().to_owned())
        } else {
            None
        }
    }

    /// Attempts to construct a tuple from the first and last elements of the
    /// structure.
    fn tuple(&self) -> Option<(Expression, Expression)> {
        if let Some(elements) = self.elements() {
            Some((elements.first().unwrap().to_owned(), elements.last().unwrap().to_owned()))
        } else {
            None
        }
    }

    /// Represents the external (rust) value associated with the structure.
    fn value(&self) -> Value {
        Value::None
    }

    /// Attempts to construct a tuple of the values associated with the first
    /// and last elements of the structure.
    fn value_tuple(&self) -> Option<(Value, Value)> {
        if let Some((x, y)) = self.tuple() {
            Some((x.value(), y.value()))
        } else {
            None
        }
    }
}

