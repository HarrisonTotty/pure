//! Integer Structures

use crate::core::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Integer(pub i64);

impl Structure for Integer {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Irreducible,
            Attribute::Number
        ]
    }

    fn value(&self) -> Value {
        Value::Integer(self.0)
    }
}

impl std::convert::From<i64> for Expression {
    fn from(number: i64) -> Expression {
        Integer(number).into()
    }
}

impl std::convert::From<Integer> for Expression {
    fn from(integer: Integer) -> Expression {
        Expression(std::rc::Rc::new(integer))
    }
}

impl std::fmt::Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Creates a new integer expression from the given value.
pub fn integer(val: i64) -> Expression {
    val.into()
}
