//! Expression Transformation Definition

use crate::core::Expression;

pub type Match = fn(Expression) -> bool;
pub type Rule = fn(Expression) -> Expression;
pub type Map = (Match, Rule);
pub type Maps = Vec<Map>;

#[derive(Clone, Debug)]
pub struct Transformation {
    pub rules: Maps
}

impl Transformation {
    /// Applies this transformation until the structure of the output expression
    /// does not change.
    pub fn apply(&self, expression: Expression) -> Expression {
        self.apply_once(expression)
    }

    /// Applied this transformation once.
    pub fn apply_once(&self, expression: Expression) -> Expression {
        for (rmatch, rule) in &self.rules {
            if rmatch(expression.clone()) {
                return rule(expression.clone())
            }
        }
        expression.clone()
    }

    pub fn has_match(&self, expression: Expression) -> bool {
        self.rules.iter().any(|(i, _)| i(expression.clone()))
    }
}

impl Fn<(Expression,)> for Transformation {
    extern "rust-call" fn call(&self, args: (Expression, )) -> Self::Output {
        self.apply(args.0)
    }
}

impl FnMut<(Expression,)> for Transformation {
    extern "rust-call" fn call_mut(&mut self, args: (Expression, )) -> Self::Output {
        self.apply(args.0)
    }
}

impl FnOnce<(Expression,)> for Transformation {
    type Output = Expression;
    extern "rust-call" fn call_once(self, args: (Expression, )) -> Self::Output {
        self.apply(args.0)
    }
}

impl std::ops::Add<Transformation> for Transformation {
    type Output = Transformation;
    fn add(self, other: Transformation) -> Self::Output {
        let mut nrules = Vec::new();
        nrules.extend(self.rules);
        nrules.extend(other.rules);
        Transformation {
            rules: nrules
        }
    }
}
