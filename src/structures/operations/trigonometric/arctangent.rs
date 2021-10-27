//! Arctangent

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Arctangent(pub Expression);

impl Structure for Arctangent {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Arctangent"), self.0.clone())
    }
}

impl std::convert::From<Arctangent> for Expression {
    fn from(op: Arctangent) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Arctangent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arctan({})", self.0)
    }
}

pub fn arctan(expression: Expression) -> Expression {
    Arctangent(expression).into()
}
