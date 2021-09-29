//! Division Operation

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Division(pub Expression, pub Expression);

impl Structure for Division {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::ArithmeticOperation,
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::BinaryOperation(String::from("ArithmeticDivision"), self.0.clone(), self.1.clone())
    }
}

impl std::convert::From<Division> for Expression {
    fn from(op: Division) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Division {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} / {})", self.0, self.1)
    }
}

impl std::ops::Div<Expression> for Expression {
    type Output = Expression;
    fn div(self, other: Expression) -> Self::Output {
        Division(self, other).into()
    }
}

impl std::ops::Div<i64> for Expression {
    type Output = Expression;
    fn div(self, other: i64) -> Self::Output {
        Division(self, other.into()).into()
    }
}
