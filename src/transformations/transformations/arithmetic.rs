//! Arithmetic Transformations

use crate::core::*;
use crate::transformations::{matches, rules};
use lazy_static::lazy_static;

lazy_static! {
    /// Provides the arithmetic identities.
    pub static ref ARITHMETIC_IDENTITIES: Transformation = Transformation {
        rules: vec![
            // 0 + x = x
            (|e| matches::first("+", 0.into())(e), rules::LAST),
            // x + 0 = x
            (|e| matches::last("+", 0.into())(e), rules::FIRST),
            // x / 1 = x
            (|e| matches::last("/", 1.into())(e), rules::FIRST),
            // x / x = 1
            (|e| matches::all_eq("/")(e), rules::ONE),
            // 1 * x = x
            (|e| matches::first("*", 1.into())(e), rules::LAST),
            // x * 1 = x
            (|e| matches::last("*", 1.into())(e), rules::FIRST),
            // x - 0 = x
            (|e| matches::last("-", 0.into())(e), rules::FIRST),
            // x - x = 0
            (|e| matches::all_eq("-")(e), rules::ZERO)
        ]
    };
    /// Provides arithmetic between integers.
    pub static ref INTEGER_ARITHMETIC: Transformation = Transformation {
        rules: vec![
            (|e| matches::all_flat_kind("+", "z")(e), rules::INTEGER_ARITHMETIC),
            (|e| matches::all_flat_kind("-", "z")(e), rules::INTEGER_ARITHMETIC),
            (|e| matches::all_flat_kind("*", "z")(e), rules::INTEGER_ARITHMETIC),
            (|e| matches::all_flat_kind("/", "z")(e), rules::INTEGER_ARITHMETIC),
        ]
    };
    /// Provides arithmetic simplification.
    pub static ref ARITHMETIC: Transformation = INTEGER_ARITHMETIC.to_owned() + ARITHMETIC_IDENTITIES.to_owned() + Transformation {
        rules: vec![
            // ax + bx = (a + b)x
            (|e| matches::first_last_kind_d2("+", "*", "*", "z", "_", "z", "_")(e), rules::FIRST)
        ]
    };
}
