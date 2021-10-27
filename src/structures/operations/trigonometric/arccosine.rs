//! Arccosine

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Arccosine(pub Expression);

impl Structure for Arccosine {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::UnaryOperation(String::from("Arccosine"), self.0.clone())
    }
}

impl std::convert::From<Arccosine> for Expression {
    fn from(op: Arccosine) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Arccosine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "arccos({})", self.0)
    }
}

pub fn arccos(expression: Expression) -> Expression {
    Arccosine(expression).into()
}
