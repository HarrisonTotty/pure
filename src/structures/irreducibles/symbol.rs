//! Symbol Structures

use crate::core::*;

#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Symbol(pub String);

impl Structure for Symbol {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::Irreducible
        ]
    }

    fn label(&self) -> Option<String> {
        Some(self.0.clone())
    }

    fn value(&self) -> Value {
        Value::Symbol(self.0.clone())
    }
}

impl std::convert::From<Symbol> for Expression {
    fn from(symbol: Symbol) -> Expression {
        Expression(std::rc::Rc::new(symbol))
    }
}

impl std::convert::From<&str> for Expression {
    fn from(string: &str) -> Expression {
        Symbol(String::from(string)).into()
    }
}

impl std::convert::From<String> for Expression {
    fn from(string: String) -> Expression {
        Symbol(string).into()
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
