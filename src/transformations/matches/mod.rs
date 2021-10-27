//! Transformation Matches

use crate::core::*;

fn from_short_kind(kind: &str) -> &str {
    match kind {
        "+" => "Addition",
        "-" => "Subtraction",
        "*" => "Multiplication",
        "/" => "Division",
        "b" => "Boolean",
        "cos" => "Cosine",
        "cot" => "Cotangent",
        "csc" => "Cosecant",
        "s" => "Symbol",
        "sec" => "Secant",
        "sin" => "Sine",
        "tan" => "Tangent",
        "z" => "Integer",
        _ => kind
    }
}

/// Returns a match object matching an expression whose sub-elements are all
/// identical to the specified expression.
pub fn all(root: &'static str, expression: Expression) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.elements().iter().all(|x| *x == expression)
}

/// Returns a match object matching an expression whose flat sub-elements are
/// all identical to the specified expression.
pub fn all_flat(root: &'static str, expression: Expression) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.flat_elements().iter().all(|x| *x == expression)
}

/// Returns a match object matching an expression whose sub-elements are all the
/// specified kind.
pub fn all_flat_kind(root: &'static str, kind: &'static str) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.flat_element_kinds().iter().all(|x| x == from_short_kind(kind) || kind == "_")
}

/// Returns a match object matching an expression whose sub-elements are all the
/// specified kind.
pub fn any_flat_kind(root: &'static str, kind: &'static str) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.flat_element_kinds().iter().any(|x| x == from_short_kind(kind) || kind == "_")
}

/// Returns a match object matching an expression whose sub-elements are all the
/// specified kind.
pub fn all_kind(root: &'static str, kind: &'static str) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.element_kinds().iter().all(|x| x == from_short_kind(kind) || kind == "_")
}

/// Returns a match object matching an expression whose sub-elements are all the
/// specified kind.
pub fn any_kind(root: &'static str, kind: &'static str) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.element_kinds().iter().any(|x| x == from_short_kind(kind) || kind == "_")
}

/// Returns a match object matching an expression whose first element is
/// structurally identical to the specified expression.
pub fn first(root: &'static str, expression: Expression) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.first() == expression
}

/// Returns a match object which matches the kind of an expression and its first
/// and last elements.
pub fn first_last(root: &'static str, first: Expression, last: Expression) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.first() == first && e.last() == last
}

/// Returns a match object which matches the kind of an expression and the kinds
/// of its first and last elements.
pub fn first_last_kind(root: &'static str, first: &'static str, last: &'static str) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && (first == "_" || e.first().kind() == from_short_kind(first)) && (last == "_" || e.last().kind() == from_short_kind(last))
}

/// Returns a match object for matching against the specified kind.
pub fn kind(root: &'static str) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root))
}

/// Returns a match object matching an expression whose last element is
/// structurally identical to the specified expression.
pub fn last(root: &'static str, expression: Expression) -> impl Fn(Expression) -> bool {
    move |e| (root == "_" || e.kind() == from_short_kind(root)) && e.last() == expression
}
