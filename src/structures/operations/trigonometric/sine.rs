//! Sine

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Sine(pub Expression);

impl Structure for Sine {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Sine"), self.0.clone())
    }
}

impl std::convert::From<Sine> for Expression {
    fn from(op: Sine) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Sine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sin({})", self.0)
    }
}

pub fn sin(expression: Expression) -> Expression {
    Sine(expression).into()
}
