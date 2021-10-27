//! Addition Operation

use crate::core::*;

#[derive(Clone, Debug)]
pub struct Addition(pub Expression, pub Expression);

impl Structure for Addition {
    fn attributes(&self) -> Attributes {
        vec![
            Attribute::ArithmeticOperation,
            Attribute::AssociativeOperation,
            Attribute::CommutativeOperation,
            Attribute::Operation
        ]
    }

    fn value(&self) -> Value {
        Value::BinaryOperation(String::from("Addition"), self.0.clone(), self.1.clone())
    }
}

impl std::convert::From<Addition> for Expression {
    fn from(op: Addition) -> Expression {
        Expression(std::rc::Rc::new(op))
    }
}

impl std::fmt::Display for Addition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {})", self.0, self.1)
    }
}

impl std::ops::Add<Expression> for Expression {
    type Output = Expression;
    fn add(self, other: Expression) -> Self::Output {
        Addition(self, other).into()
    }
}

impl std::ops::Add<Expression> for i64 {
    type Output = Expression;
    fn add(self, other: Expression) -> Self::Output {
        Addition(self.into(), other).into()
    }
}

impl std::ops::Add<i64> for Expression {
    type Output = Expression;
    fn add(self, other: i64) -> Self::Output {
        Addition(self, other.into()).into()
    }
}
