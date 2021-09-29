//! Common Definitions and functions

pub use crate::core::*;
pub use crate::structures::irreducibles::*;
pub use crate::structures::operations::arithmetic::*;

/// Creates a new integer expression from the given value.
pub fn integer(val: i64) -> Expression {
    val.into()
}

/// Creates a new symbol expression from the given string.
pub fn symbol(label: &str) -> Expression {
    label.into()
}
