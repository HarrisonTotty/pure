//! Multiplication Operation

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Multiplication(pub Expression, pub Expression);

impl Structure for Multiplication {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::ArithmeticOperation,
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::BinaryOperation(String::from("ArithmeticMultiplication"), self.0.clone(), self.1.clone())
    }
}

impl std::convert::From<Multiplication> for Expression {
    fn from(op: Multiplication) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Multiplication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} * {})", self.0, self.1)
    }
}

impl std::ops::Mul<Expression> for Expression {
    type Output = Expression;
    fn mul(self, other: Expression) -> Self::Output {
        Multiplication(self, other).into()
    }
}

impl std::ops::Mul<i64> for Expression {
    type Output = Expression;
    fn mul(self, other: i64) -> Self::Output {
        Multiplication(self, other.into()).into()
    }
}
