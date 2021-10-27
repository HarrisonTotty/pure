//! Tangent

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Tangent(pub Expression);

impl Structure for Tangent {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Tangent"), self.0.clone())
    }
}

impl std::convert::From<Tangent> for Expression {
    fn from(op: Tangent) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Tangent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tan({})", self.0)
    }
}

pub fn tan(expression: Expression) -> Expression {
    Tangent(expression).into()
}
