//! Multiplication Operation

use crate::core::*;

crate::mkstructure!{Multiplication, 2, "({} * {})", multiplication}

impl std::ops::Mul<Expression> for Expression {
    type Output = Expression;
    fn mul(self, other: Expression) -> Self::Output {
        Multiplication(self, other).into()
    }
}

impl std::ops::Mul<Expression> for i64 {
    type Output = Expression;
    fn mul(self, other: Expression) -> Self::Output {
        Multiplication(self.into(), other).into()
    }
}

impl std::ops::Mul<i64> for Expression {
    type Output = Expression;
    fn mul(self, other: i64) -> Self::Output {
        Multiplication(self, other.into()).into()
    }
}
