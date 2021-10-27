//! Cosecant

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Cosecant(pub Expression);

impl Structure for Cosecant {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Cosecant"), self.0.clone())
    }
}

impl std::convert::From<Cosecant> for Expression {
    fn from(op: Cosecant) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Cosecant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "csc({})", self.0)
    }
}

pub fn csc(expression: Expression) -> Expression {
    Cosecant(expression).into()
}
