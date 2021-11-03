//! Mathematical Expressions
//!
//! The following module containes the definition of a mathematical
//! `Expression`, as well as its associated types and implementations.

use crate::Structure;

/// Represents a collection of mathematical expressions.
pub type Expressions = Vec<Expression>;

/// Represents a mathematical expression, which is a pointer to some underlying
/// mathematical structure.
///
/// `Expressions` are Rust `struct`s which automatically dereference their
/// contained `Box<dyn Structure>`. As such, any method attached to the
/// `Structure` trait may be called from them.
#[derive(Clone, Debug)]
pub struct Expression(pub box dyn Structure>);
