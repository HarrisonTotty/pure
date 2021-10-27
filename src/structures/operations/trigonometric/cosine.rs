//! Cosine

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Cosine(pub Expression);

impl Structure for Cosine {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Cosine"), self.0.clone())
    }
}

impl std::convert::From<Cosine> for Expression {
    fn from(op: Cosine) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Cosine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cos({})", self.0)
    }
}

pub fn cos(expression: Expression) -> Expression {
    Cosine(expression).into()
}
