//! Multiplication Operation

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Subtraction(pub Expression, pub Expression);

impl Structure for Subtraction {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::ArithmeticOperation,
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::BinaryOperation(String::from("ArithmeticSubtraction"), self.0.clone(), self.1.clone())
    }
}

impl std::convert::From<Subtraction> for Expression {
    fn from(op: Subtraction) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Subtraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} - {})", self.0, self.1)
    }
}

impl std::ops::Sub<Expression> for Expression {
    type Output = Expression;
    fn sub(self, other: Expression) -> Self::Output {
        Subtraction(self, other).into()
    }
}

impl std::ops::Sub<Expression> for i64 {
    type Output = Expression;
    fn sub(self, other: Expression) -> Self::Output {
        Subtraction(self.into(), other).into()
    }
}

impl std::ops::Sub<i64> for Expression {
    type Output = Expression;
    fn sub(self, other: i64) -> Self::Output {
        Subtraction(self, other.into()).into()
    }
}
