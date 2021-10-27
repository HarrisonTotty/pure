//! Arccosecant

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Arccosecant(pub Expression);

impl Structure for Arccosecant {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Arccosecant"), self.0.clone())
    }
}

impl std::convert::From<Arccosecant> for Expression {
    fn from(op: Arccosecant) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Arccosecant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arccsc({})", self.0)
    }
}

pub fn arccsc(expression: Expression) -> Expression {
    Arccosecant(expression).into()
}
