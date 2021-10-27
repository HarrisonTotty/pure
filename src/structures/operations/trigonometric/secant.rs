//! Secant

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Secant(pub Expression);

impl Structure for Secant {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Secant"), self.0.clone())
    }
}

impl std::convert::From<Secant> for Expression {
    fn from(op: Secant) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Secant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "sec({})", self.0)
    }
}

pub fn sec(expression: Expression) -> Expression {
    Secant(expression).into()
}
