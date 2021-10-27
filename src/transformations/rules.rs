//! Transformation Rules

use crate::prelude::*;

/// Performs addition, division, multiplication, and subtraction between
/// integers.
pub const INTEGER_ARITHMETIC: Rule = |e| {
    let kind = e.kind();
    let mut result: Option<i64> = None;
    for ex in e.flat_elements() {
        if let Value::Integer(v) = ex.value() {
            if let Some(curr) = result {
                result = Some(match kind.as_str() {
                    "Addition" => curr + v,
                    "Division" => curr / v,
                    "Multiplication" => curr * v,
                    "Subtraction" => curr - v,
                    _ => panic!("invalid operation")
                })
            } else {
                result = Some(v)
            }
        }
    }
    Expression::from(result.unwrap())
};

/// If the input expression is a division or fraction, returns the input as is.
/// Otherwise, returns the input expression over `1`.
pub const INTO_FRACTION: Rule = |e| {
    match e.kind().as_str() {
        "Division" => e,
        _ => Division(e, integer(1)).into()
    }
};

/// Returns the first element of the expression.
pub const FIRST: Rule = |e| e.elements().first().unwrap().to_owned();

/// Returns the integer `1`.
pub const ONE: Rule = |_| Expression::from(1);

/// Returns the last element of the expression.
pub const LAST: Rule = |e| e.elements().last().unwrap().to_owned();

/// Returns the integer `0`.
pub const ZERO: Rule = |_| Expression::from(0);
