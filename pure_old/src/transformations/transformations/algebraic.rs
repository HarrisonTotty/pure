//! Algebraic Transformations

use crate::core::*;
//use crate::transformations::{matches, rules};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SIMPLIFY: Transformation = super::arithmetic::ARITHMETIC.to_owned() + Transformation {
        rules: vec![
            // x * ( 1 / x ) = 1
        ]
    };
}

/// Performs a simplify transformation on the specified expression.
pub fn simplify(expression: Expression) -> Expression {
    SIMPLIFY(expression)
}
