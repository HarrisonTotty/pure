//! Arccotangent

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Arccotangent(pub Expression);

impl Structure for Arccotangent {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Arccotangent"), self.0.clone())
    }
}

impl std::convert::From<Arccotangent> for Expression {
    fn from(op: Arccotangent) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Arccotangent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arccot({})", self.0)
    }
}

pub fn arccot(expression: Expression) -> Expression {
    Arccotangent(expression).into()
}
