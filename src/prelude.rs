//! Common Definitions and functions

use crate::transformations;

pub use crate::core::*;
pub use crate::structures::irreducibles::*;
pub use crate::structures::operations::arithmetic::*;

/// Creates a new integer expression from the given value.
pub fn integer(val: i64) -> Expression {
    val.into()
}

/// Performs a simplify transformation on the specified expression.
pub fn simplify(expression: Expression) -> Expression {
    transformations::SIMPLIFY(expression)
}

/// Creates a new symbol expression from the given string.
pub fn symbol(label: &str) -> Expression {
    label.into()
}
