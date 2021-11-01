//! Mathematical Expressions

use crate::core::Structure;

/// Represents a collection of mathematical expressions.
pub type Expressions = Vec<Expression>;

/// Represents a mathematical expression, which is defined to be a pointer to
/// some underlying mathematical structure.
#[derive(Clone, Debug)]
pub struct Expression(pub std::rc::Rc<dyn Structure>);

impl std::convert::From<Expression> for Option<(Expression, Expression)> {
    fn from(ex: Expression) -> Option<(Expression, Expression)> {
        ex.tuple()
    }
}

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

impl std::iter::IntoIterator for Expression {
    type Item = Expression;
    type IntoIter = std::vec::IntoIter<Expression>;
    fn into_iter(self) -> Self::IntoIter {
        if let Some(el) = self.0.elements() {
            el.into_iter()
        } else {
            Vec::new().into_iter()
        }
    }
}

impl std::cmp::PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        self.is_identical_to(other.to_owned())
    }
}

impl std::cmp::Eq for Expression {}
