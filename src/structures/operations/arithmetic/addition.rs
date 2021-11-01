//! Addition Operation

use crate::core::*;

crate::mkstructure!{Addition, 2, "({} + {})", addition}

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
