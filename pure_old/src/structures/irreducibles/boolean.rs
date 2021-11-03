//! Boolean Structures

use crate::core::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Boolean(pub bool);

impl Structure for Boolean {
    fn from_elements(&self, _elements: Expressions) -> Expression {
        self.clone().into()
    }

    fn value(&self) -> Value {
        Value::Boolean(self.0)
    }
}

impl std::convert::From<bool> for Expression {
    fn from(boolean: bool) -> Expression {
        Boolean(boolean).into()
    }
}

impl std::convert::From<Boolean> for Expression {
    fn from(boolean: Boolean) -> Expression {
        Expression(std::rc::Rc::new(boolean))
    }
}

impl std::fmt::Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
