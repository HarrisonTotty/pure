//! Core Engine Tests

use pure::prelude::*;

#[derive(Clone, Debug, PartialEq)]
struct BasicStructure(bool);
impl Structure for BasicStructure {
    fn value(&self) -> Value {
        Value::Boolean(self.0)
    }
}

impl std::convert::From<BasicStructure> for Expression {
    fn from(s: BasicStructure) -> Expression {
        Expression(std::rc::Rc::new(s))
    }
}

impl std::fmt::Display for BasicStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct EmptyStructure;
impl Structure for EmptyStructure {}

impl std::convert::From<EmptyStructure> for Expression {
    fn from(s: EmptyStructure) -> Expression {
        Expression(std::rc::Rc::new(s))
    }
}

impl std::fmt::Display for EmptyStructure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Empty")
    }
}

#[test]
fn basic_structure() {
    let s = BasicStructure(true);
    assert_eq!(s.kind(), "Boolean");
    assert_eq!(s.label(), None);
    assert_eq!(s.value(), Value::Boolean(true));
}

#[test]
fn basic_structure_expression() {
    let s: Expression = BasicStructure(true).into();
    assert_eq!(s.kind(), "Boolean");
    assert_eq!(s.label(), None);
    assert_eq!(s.value(), Value::Boolean(true));
}

#[test]
fn empty_structure() {
    let s = EmptyStructure;
    assert_eq!(s.kind(), "None");
    assert_eq!(s.label(), None);
    assert_eq!(s.value(), Value::None);
}

#[test]
fn empty_structure_expression() {
    let s: Expression = EmptyStructure.into();
    assert_eq!(s.kind(), "None");
    assert_eq!(s.label(), None);
    assert_eq!(s.value(), Value::None);
}
