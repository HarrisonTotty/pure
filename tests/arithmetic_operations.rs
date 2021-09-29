//! Arithmetic Operations Tests

use pure::prelude::*;

#[test]
fn addition_explicit() {
    let x = symbol("x");
    let y = symbol("y");
    let add = Addition(x, y);

    assert_eq!(add.kind(), "ArithmeticAddition");
    assert_eq!(
        Expression::from(add.clone()),
        Expression::from(Addition(symbol("x"), symbol("y")))
    );
    assert!(
        Expression::from(add.clone()) != Expression::from(Addition(symbol("y"), symbol("x")))
    );
}

#[test]
fn addition_implicit() {
    let x = symbol("x");
    let y = symbol("y");
    let add = x + y;

    assert_eq!(add.kind(), "ArithmeticAddition");
    assert_eq!(
        add,
        symbol("x") + symbol("y")
    );
    assert!(
        add.clone() != symbol("y") + symbol("x")
    );
}
