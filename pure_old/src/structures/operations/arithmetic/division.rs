//! Division Operation

use crate::core::*;

crate::mkstructure!{Division, 2, "({} / {})", division}

impl std::ops::Div<Expression> for Expression {
    type Output = Expression;
    fn div(self, other: Expression) -> Self::Output {
        Division(self, other).into()
    }
}

impl std::ops::Div<Expression> for i64 {
    type Output = Expression;
    fn div(self, other: Expression) -> Self::Output {
        Division(self.into(), other).into()
    }
}

impl std::ops::Div<i64> for Expression {
    type Output = Expression;
    fn div(self, other: i64) -> Self::Output {
        Division(self, other.into()).into()
    }
}
