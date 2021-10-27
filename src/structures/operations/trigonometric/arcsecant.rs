//! Arcsecant

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Arcsecant(pub Expression);

impl Structure for Arcsecant {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Arcsecant"), self.0.clone())
    }
}

impl std::convert::From<Arcsecant> for Expression {
    fn from(op: Arcsecant) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Arcsecant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arcsec({})", self.0)
    }
}

pub fn arcsec(expression: Expression) -> Expression {
    Arcsecant(expression).into()
}
