//! Transformation Matches

use crate::core::*;


/// Arithmetic addition between an expression and the integer `0`.
pub const ADDITION_IDENTITY: Match = |e| e.elements().iter().any(|i| ZERO(i.clone()));

/// Arithmetic addition over integers.
pub const ADDITION_OVER_INTEGERS: Match = |e| ADDITION(e.clone()) && ALL_INTEGERS_FLAT(e);

/// An expression whose elements are all integers.
pub const ALL_INTEGERS: Match = |e| e.element_kinds().iter().all(|i| i == "Integer");

/// An expression whose flattened elements are all integers.
pub const ALL_INTEGERS_FLAT: Match = |e| e.flat_element_kinds().iter().all(|i| i == "Integer");

/// An expression whose elements are structurally identical.
pub const ALL_SAME: Match = |e| e.elements().iter().all(|i| *i == e.first());

/// An expression that contains at least one integer as an element.
pub const ANY_INTEGERS: Match = |e| e.element_kinds().iter().any(|i| i == "Integer");

/// An expression that contains at least one integer within its flattened
/// elements.
pub const ANY_INTEGERS_FLAT: Match = |e| e.flat_element_kinds().iter().any(|i| i == "Integer");

/// Any expression.
pub const ANYTHING: Match = |_| true;

/// Arithmetic addition, division, multiplication, or subtraction.
pub const ARITHMETIC: Match = |e| ADDITION(e.clone()) || DIVISION(e.clone()) || MULTIPLICATION(e.clone()) || SUBTRACTION(e);

/// Arithmetic addition, division, multiplication, or subtraction over integers.
pub const ARITHMETIC_OVER_INTEGERS: Match = |e| ARITHMETIC(e.clone()) && ALL_INTEGERS_FLAT(e);

/// Any binary operation.
pub const BINARY_OPERATION: Match = |e| match e.value() { Value::BinaryOperation(_, _, _) => true, _ => false };

/// An expression with no sub-elements.
pub const EMPTY: Match = |e| e.elements().len() == 0;

/// An expression whose first element is the integer `1`.
pub const FIRST_ONE: Match = |e| e.first() == Expression::from(1);

/// An expression whose first element is the integer `0`.
pub const FIRST_ZERO: Match = |e| e.first() == Expression::from(0);

/// An expression whose last element is the integer `1`.
pub const LAST_ONE: Match = |e| e.last() == Expression::from(1);

/// An expression whose last element is the integer `0`.
pub const LAST_ZERO: Match = |e| e.last() == Expression::from(0);

/// The integer `1`.
pub const ONE: Match = |e| e == Expression::from(1);

/// Trigonometric Secant.
pub const SEC: Match = |e| e.kind() == "Secant";

/// Trigonometric Sine.
pub const SIN: Match = |e| e.kind() == "Sine";

/// Arithmetic subtraction.
pub const SUBTRACTION: Match = |e| e.kind() == "Subtraction";

/// Any symbol.
pub const SYMBOL: Match = |e| e.kind() == "Symbol";

/// Trigonometric Tangent.
pub const TAN: Match = |e| e.kind() == "Tangent";

/// The integer `0`.
pub const ZERO: Match = |e| e == Expression::from(0);
