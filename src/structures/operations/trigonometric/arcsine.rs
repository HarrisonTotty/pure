//! Arcsine

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Arcsine(pub Expression);

impl Structure for Arcsine {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Arcsine"), self.0.clone())
    }
}

impl std::convert::From<Arcsine> for Expression {
    fn from(op: Arcsine) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Arcsine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arcsin({})", self.0)
    }
}

pub fn arcsin(expression: Expression) -> Expression {
    Arcsine(expression).into()
}
