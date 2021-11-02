//! Common Enumerables (and associated types)

use crate::core::{Expression, Expressions};

/// A collection of attributes.
pub type Attributes = Vec<Attribute>;

/// Represents an attribute that can be associated with a structure.
#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    ArithmeticOperation,
    AssociativeOperation,
    CommutativeOperation,
    Constant,
    DistributiveOperation,
    Function,
    IdempotentOperation,
    Irreducible,
    LogicalOperation,
    Matrix,
    Number,
    Operation,
    Other(String),
    Polynomial,
    Symbol,
    Tensor,
    Vector
}

/// Represents the enumerable (rust) value of a structure.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    BinaryOperation(String, Expression, Expression),
    Boolean(bool),
    Float(f64),
    Integer(i64),
    Kind(String),
    NaryOperation(String, Expressions),
    None,
    String(String),
    Symbol(String),
    TernaryOperation(String, Expression, Expression, Expression),
    UnaryOperation(String, Expression)
}

impl std::ops::Add<Value> for Value {
    type Output = Value;
    fn add(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
            (Value::Integer(x), Value::Integer(y)) => Value::Integer(x + y),
            _ => Value::None
        }
    }
}

impl std::ops::Div<Value> for Value {
    type Output = Value;
    fn div(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
            (Value::Integer(x), Value::Integer(y)) => Value::Integer(x / y),
            _ => Value::None
        }
    }
}

impl std::ops::Mul<Value> for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
            (Value::Integer(x), Value::Integer(y)) => Value::Integer(x * y),
            _ => Value::None
        }
    }
}

impl std::ops::Sub<Value> for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Self::Output {
        match (self, other) {
            (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
            (Value::Integer(x), Value::Integer(y)) => Value::Integer(x - y),
            _ => Value::None
        }
    }
}
