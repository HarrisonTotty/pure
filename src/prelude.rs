//! Common Definitions and functions

use crate::transformations;

pub use crate::core::*;
pub use crate::structures::irreducibles::*;
pub use crate::structures::operations::arithmetic::*;
pub use crate::structures::operations::trigonometric::*;


/// Performs a simplify transformation on the specified expression.
pub fn simplify(expression: Expression) -> Expression {
    transformations::SIMPLIFY(expression)
}

