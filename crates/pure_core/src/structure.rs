//! Mathematical Structure Trait
//!
//! The following module containes the definition of a mathematical
//! `Structure`, as well as its associated types and implementations.

use crate::*;

pub type Expression = Box<dyn Structure>;
pub type Expressions = Vec<Expression>;

/// Represents a mathematical structure.
///
/// The `Structure` trait provides a common interface for all mathematical
/// expressions. While the implementation details of any `struct` that
/// implements the `Structure` trait may be unknown, logically `Structure`
/// objects may be thought of as trees, optionally containing sub-expressions
/// called its _elements_. Each structure has an associated `kind` (such as
/// `Addition` or `Integer`).
pub trait Structure: std::fmt::Debug + std::fmt::Display {
    /// The collection of elements contained within this structure.
    fn elements(&self) -> Option<Expressions> {
        None
    }
}
