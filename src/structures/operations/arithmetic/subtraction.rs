//! Multiplication Operation

use crate::core::*;

crate::mkstructure!{Subtraction, 2, "({} - {})", subtraction}

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
