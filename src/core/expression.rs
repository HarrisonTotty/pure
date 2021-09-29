//! Mathematical Expressions

use crate::core::Structure;

pub type Expressions = Vec<Expression>;

/// Represents a mathematical expression.
#[derive(Clone, Debug)]
pub struct Expression(pub std::rc::Rc<dyn Structure>);

impl std::ops::Deref for Expression {
    type Target = std::rc::Rc<dyn Structure>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl std::cmp::PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.is_identical_to(other.to_owned())
    }
}

impl std::cmp::Eq for Expression {}
