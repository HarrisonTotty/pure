//! Expression Transformations

use lazy_static::lazy_static;

pub mod helpers;
pub mod matches;
pub mod rules;

use crate::core::*;

lazy_static! {
    /// Provides common arithmetic identities.
    pub static ref ARITHMETIC: Transformation = Transformation {
        rules: vec![
            // 0 + x = x
            (|e| matches::ADDITION(e.clone()) && matches::FIRST_ZERO(e), rules::LAST),
            // x + 0 = x
            (|e| matches::ADDITION(e.clone()) && matches::LAST_ZERO(e), rules::FIRST),
            // x / 1 = x
            (|e| matches::DIVISION(e.clone()) && matches::LAST_ONE(e), rules::FIRST),
            // x / x = 1
            (|e| matches::DIVISION(e.clone()) && matches::ALL_SAME(e), rules::ONE),
            // 1 * x = x
            (|e| matches::MULTIPLICATION(e.clone()) && matches::FIRST_ONE(e), rules::LAST),
            // x * 1 = x
            (|e| matches::MULTIPLICATION(e.clone()) && matches::LAST_ONE(e), rules::FIRST),
            // x - 0 = x
            (|e| matches::SUBTRACTION(e.clone()) && matches::LAST_ZERO(e), rules::FIRST),
            // x - x = 0
            (|e| matches::SUBTRACTION(e.clone()) && matches::ALL_SAME(e), rules::ZERO),
            // 2 + 3 = 5, 2 * 3 = 6, etc.
            (matches::ARITHMETIC_OVER_INTEGERS, rules::INTEGER_ARITHMETIC)
        ]
    };
    pub static ref SIMPLIFY: Transformation = ARITHMETIC.to_owned() + Transformation {
        rules: vec![
            // x * ( 1 / x ) = 1
        ]
    };
}
