//! Irreducible Structures

use pure::prelude::*;

#[test]
fn booleans() {
    let b1: Expression = Boolean(true).into();
    let b2: Expression = false.into();

    assert_eq!(b1.kind(), "Boolean");
    assert_eq!(b1.value(), Value::Boolean(true));
    assert_eq!(b2.kind(), "Boolean");
    assert_eq!(b2.value(), Value::Boolean(false));

    assert_eq!(b1.clone(), Expression::from(true));
    assert_eq!(b2.clone(), Expression::from(false));

    assert!(b1.clone() != b2.clone());
}

#[test]
fn integers() {
    let i1: Expression = Integer(0).into();
    let i2: Expression = 1.into();
    let i3 = integer(2);

    assert_eq!(i1.kind(), "Integer");
    assert_eq!(i1.value(), Value::Integer(0));
    assert_eq!(i2.kind(), "Integer");
    assert_eq!(i2.value(), Value::Integer(1));
    assert_eq!(i3.kind(), "Integer");
    assert_eq!(i3.value(), Value::Integer(2));

    assert_eq!(i1.clone(), Expression::from(0));
    assert_eq!(i2.clone(), Expression::from(1));
    assert_eq!(i3.clone(), Expression::from(2));

    assert!(i1.clone() != i2.clone());
    assert!(i2.clone() != i3.clone());
    assert!(i3.clone() != i1.clone());
}

#[test]
fn symbols() {
    let x: Expression = Symbol(String::from("x")).into();
    let y: Expression = "y".into();
    let z = symbol("z");

    assert_eq!(x.kind(), "Symbol");
    assert_eq!(x.value(), Value::Symbol(String::from("x")));
    assert_eq!(y.kind(), "Symbol");
    assert_eq!(y.value(), Value::Symbol(String::from("y")));
    assert_eq!(z.kind(), "Symbol");
    assert_eq!(z.value(), Value::Symbol(String::from("z")));

    assert_eq!(x.clone(), symbol("x"));
    assert_eq!(y.clone(), symbol("y"));
    assert_eq!(z.clone(), symbol("z"));

    assert!(x.clone() != y.clone());
    assert!(y.clone() != z.clone());
    assert!(z.clone() != x.clone());
}
