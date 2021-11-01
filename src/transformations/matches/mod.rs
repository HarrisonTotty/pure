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

macro_rules! iskind { ($e:ident, $k:ident) => { ($k == "_" || $e.kind() == from_short_kind($k)) } }

/// Returns a match object matching an expression whose sub-elements are all
/// identical to the specified expression.
pub fn all(root: &'static str, expression: Expression) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && match e.elements() { Some(el) => el.iter().all(|x| *x == expression), _ => false }
}

/// Returns a match object matching an expression whose sub-elements are all
/// identical to each other.
pub fn all_eq(root: &'static str) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && match e.elements() { Some(el) => el.iter().all(|x| *x == el.first().unwrap().to_owned()), _ => false }
}

/// Returns a match object matching an expression whose flat sub-elements are
/// all identical to the specified expression.
pub fn all_flat(root: &'static str, expression: Expression) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && match e.flat_elements() { Some(el) => el.iter().all(|x| *x == expression), _ => false }
}

/// Returns a match object matching an expression whose sub-elements are all the
/// specified kind.
pub fn all_flat_kind(root: &'static str, kind: &'static str) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && (kind == "_" || match e.flat_element_kinds() { Some(el) => el.iter().all(|x| x == from_short_kind(kind)), _ => false })
}

/// Returns a match object matching an expression whose sub-elements are all the
/// specified kind.
pub fn any_flat_kind(root: &'static str, kind: &'static str) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && (kind == "_" || match e.flat_element_kinds() { Some(el) => el.iter().any(|x| x == from_short_kind(kind)), _ => false })
}

/// Returns a match object matching an expression whose sub-elements are all the
/// specified kind.
pub fn all_kind(root: &'static str, kind: &'static str) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && (kind == "_" || match e.element_kinds() { Some(el) => el.iter().all(|x| x == from_short_kind(kind)), _ => false })
}

/// Returns a match object matching an expression whose sub-elements are all the
/// specified kind.
pub fn any_kind(root: &'static str, kind: &'static str) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && (kind == "_" || match e.element_kinds() { Some(el) => el.iter().any(|x| x == from_short_kind(kind)), _ => false })
}

/// Returns a match object matching an expression whose first element is
/// structurally identical to the specified expression.
pub fn first(root: &'static str, expression: Expression) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && match e.first() { Some(f) => f == expression, _ => false }
}

/// Returns a match object which matches the kind of an expression and its first
/// and last elements.
pub fn first_last(root: &'static str, first: Expression, last: Expression) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && match (e.first(), e.last()) { (Some(f), Some(l)) => f == first && l == last, _ => false }
}

/// Returns a match object which matches the kind of an expression and the kinds
/// of its first and last elements.
pub fn first_last_kind(root: &'static str, first: &'static str, last: &'static str) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && match (e.first(), e.last()) { (Some(f), Some(l)) => iskind!(f, first) && iskind!(l, last), _ => false }
}

/// Returns a match object for matching against the specified kind.
pub fn kind(root: &'static str) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root)
}

/// Returns a match object matching an expression whose last element is
/// structurally identical to the specified expression.
pub fn last(root: &'static str, expression: Expression) -> impl Fn(Expression) -> bool {
    move |e| iskind!(e, root) && match e.last() { Some(l) => l == expression, _ => false }
}
