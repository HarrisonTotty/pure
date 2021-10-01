//! Builtin Transformation Tests

use pure::prelude::*;
use pure::transformations::*;

#[test]
fn arithmetic() {
    let arithmetic = &ARITHMETIC;

    // ----- Integer arithmetic -----
    assert_eq!(arithmetic(Expression::from(2) + 3), Expression::from(5));
    assert_eq!(arithmetic(Expression::from(2) - 3), Expression::from(-1));
    assert_eq!(arithmetic(Expression::from(2) * 3), Expression::from(6));
    assert_eq!(arithmetic(Expression::from(4) / 2), Expression::from(2));

    assert_eq!(arithmetic(Expression::from(2) + 3 + 4), Expression::from(9));
    assert_eq!(arithmetic(Expression::from(2) - 3 - 4), Expression::from(-5));
    assert_eq!(arithmetic(Expression::from(2) * 3 * 4), Expression::from(24));
    assert_eq!(arithmetic(Expression::from(4) / 2 / 2), Expression::from(1));

    // ----- Identities -----

    // 0 + x = x
    assert_eq!(arithmetic(0 + symbol("x")), symbol("x"));

    // x + 0 = x
    assert_eq!(arithmetic(symbol("x") + 0), symbol("x"));

    // x - 0 = x
    assert_eq!(arithmetic(symbol("x") - 0), symbol("x"));

    // x - x = 0
    assert_eq!(arithmetic(symbol("x") - symbol("x")), integer(0));

    // 1 * x = x
    assert_eq!(arithmetic(1 * symbol("x")), symbol("x"));

    // x * 1 = x
    assert_eq!(arithmetic(symbol("x") * 1), symbol("x"));

    // x / 1 = x
    assert_eq!(arithmetic(symbol("x") / 1), symbol("x"));

    // x / x = 1
    assert_eq!(arithmetic(symbol("x") / symbol("x")), integer(1));
}

#[test]
fn simplify() {
    assert_eq!(
        pure::prelude::simplify(Expression::from(1) + 1),
        Expression::from(2)
    );
    assert_eq!(
        pure::prelude::simplify(Expression::from(1) - 1),
        Expression::from(0)
    );
    assert_eq!(
        pure::prelude::simplify(Expression::from(1) * 1),
        Expression::from(1)
    );
    assert_eq!(
        pure::prelude::simplify(Expression::from(1) / 1),
        Expression::from(1)
    );
}
