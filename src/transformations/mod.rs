//! Expression Transformations

use lazy_static::lazy_static;

pub mod helpers;
pub mod matches;
pub mod rules;

use crate::core::*;

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
            (|e| matches::all("/", e.first())(e), rules::ONE),
            // 1 * x = x
            (|e| matches::first("*", 1.into())(e), rules::LAST),
            // x * 1 = x
            (|e| matches::last("*", 1.into())(e), rules::FIRST),
            // x - 0 = x
            (|e| matches::last("-", 0.into())(e), rules::FIRST),
            // x - x = 0
            (|e| matches::all("-", e.first())(e), rules::ZERO)
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
    /// Provides arithmetic.
    pub static ref ARITHMETIC: Transformation = INTEGER_ARITHMETIC.to_owned() + ARITHMETIC_IDENTITIES.to_owned();
    /// Provides common trigonometric identities.
    pub static ref TRIGONOMETRIC_IDENTITIES: Transformation = Transformation {
        rules: vec![
            // cos(0) = 1
            (|e| matches::first("cos", 0.into())(e), rules::ONE),
            // sec(0) = 1
            (|e| matches::first("sec", 0.into())(e), rules::ONE),
            // sin(0) = 0
            (|e| matches::first("sin", 0.into())(e), rules::ZERO),
            // tan(0) = 0
            (|e| matches::first("tan", 0.into())(e), rules::ZERO),
        ]
    };
    pub static ref SIMPLIFY: Transformation = ARITHMETIC.to_owned() + Transformation {
        rules: vec![
            // x * ( 1 / x ) = 1
        ]
    };
}
