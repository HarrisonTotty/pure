//! Cotangent

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Cotangent(pub Expression);

impl Structure for Cotangent {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Cotangent"), self.0.clone())
    }
}

impl std::convert::From<Cotangent> for Expression {
    fn from(op: Cotangent) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Cotangent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cot({})", self.0)
    }
}

pub fn cot(expression: Expression) -> Expression {
    Cotangent(expression).into()
}
