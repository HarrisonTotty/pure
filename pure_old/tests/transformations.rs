//! Builtin Transformation Tests

use pure::prelude::*;
use pure::transformations::transformations::arithmetic::*;

#[test]
fn arithmetic() {
    let integer_arithmetic = &INTEGER_ARITHMETIC;
    let arithmetic_identities = &ARITHMETIC_IDENTITIES;

    // ----- Integer arithmetic -----
    assert!(integer_arithmetic.has_match(Expression::from(2) + 3).is_some());
    assert!(integer_arithmetic.has_match(Expression::from(2) - 3).is_some());
    assert!(integer_arithmetic.has_match(Expression::from(2) * 3).is_some());
    assert!(integer_arithmetic.has_match(Expression::from(4) / 2).is_some());


    assert_eq!(integer_arithmetic(Expression::from(2) + 3), Expression::from(5));
    assert_eq!(integer_arithmetic(Expression::from(2) - 3), Expression::from(-1));
    assert_eq!(integer_arithmetic(Expression::from(2) * 3), Expression::from(6));
    assert_eq!(integer_arithmetic(Expression::from(4) / 2), Expression::from(2));

    assert!(integer_arithmetic.has_match(Expression::from(2) + 3 + 4).is_some());
    assert!(integer_arithmetic.has_match(Expression::from(2) - 3 - 4).is_some());
    assert!(integer_arithmetic.has_match(Expression::from(2) * 3 * 4).is_some());
    assert!(integer_arithmetic.has_match(Expression::from(4) / 2 / 2).is_some());

    assert_eq!(integer_arithmetic(Expression::from(2) + 3 + 4), Expression::from(9));
    assert_eq!(integer_arithmetic(Expression::from(2) - 3 - 4), Expression::from(-5));
    assert_eq!(integer_arithmetic(Expression::from(2) * 3 * 4), Expression::from(24));
    assert_eq!(integer_arithmetic(Expression::from(4) / 2 / 2), Expression::from(1));

    // ----- Identities -----

    // 0 + x = x
    assert!(arithmetic_identities.has_match(0 + symbol("x")).is_some());
    assert_eq!(arithmetic_identities(0 + symbol("x")), symbol("x"));

    // x + 0 = x
    assert!(arithmetic_identities.has_match(symbol("x") + 0).is_some());
    assert_eq!(arithmetic_identities(symbol("x") + 0), symbol("x"));

    // x - 0 = x
    assert!(arithmetic_identities.has_match(symbol("x") - 0).is_some());
    assert_eq!(arithmetic_identities(symbol("x") - 0), symbol("x"));

    // x - x = 0
    assert!(arithmetic_identities.has_match(symbol("x") - symbol("x")).is_some());
    assert_eq!(arithmetic_identities(symbol("x") - symbol("x")), integer(0));

    // 1 * x = x
    assert!(arithmetic_identities.has_match(1 * symbol("x")).is_some());
    assert_eq!(arithmetic_identities(1 * symbol("x")), symbol("x"));

    // x * 1 = x
    assert!(arithmetic_identities.has_match(symbol("x") * 1).is_some());
    assert_eq!(arithmetic_identities(symbol("x") * 1), symbol("x"));

    // x / 1 = x
    assert!(arithmetic_identities.has_match(symbol("x") / 1).is_some());
    assert_eq!(arithmetic_identities(symbol("x") / 1), symbol("x"));

    // x / x = 1
    assert!(arithmetic_identities.has_match(symbol("x") / symbol("x")).is_some());
    assert_eq!(arithmetic_identities(symbol("x") / symbol("x")), integer(1));
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
