//! Expression Transformations
//!
//! The following module containes the definition of a mathematical
//! `Transformation`, as well as its associated types and implementations.

use crate::core::Expression;

/// A function which returns whether an input `Expression` matches a defined
/// criteria.
pub type Match = fn(Expression) -> bool;

/// A function which creates a new `Expression` optionally based on the
/// components of a given input `Expression.`
pub type Rule = fn(Expression) -> Expression;

/// A tuple containing a `Match` function as well as a `Rule` function to apply
/// if the result of the `Match` function is `true`.
pub type Map = (Match, Rule);

/// A vector of `Map` tuples.
pub type Maps = Vec<Map>;

/// Represents a transformation engine, responsible for converting one
/// `Expression` into another.
///
/// `Transformation` objects are defined with a set of `Map` tuples which
/// generate new `Expression` objects if their makeup matches some defined
/// criteria.
#[derive(Clone, Debug)]
pub struct Transformation {
    pub rules: Maps
}

impl Transformation {
    /// Applies this transformation until the structure of the output expression
    /// does not change.
    pub fn apply(&self, expression: Expression) -> Expression {
        let mut current = expression.clone();
        while let Some(rule) = self.has_match(current.clone()) {
            current = rule(current);
        }
        current
    }

    /// Applied this transformation to the elements of an expression until the
    /// collection of elements does not change.
    pub fn apply_elements(&self, expression: Expression) -> Expression {
        let mut current = expression.clone();
        loop {
            let new = self.apply_elements_once(current.clone());
            if new == current {
                break;
            } else {
                current = new;
            }
        }
        current
    }

    /// Applied this transformation to the elements of an expression.
    pub fn apply_elements_once(&self, expression: Expression) -> Expression {
        if let Some(elements) = expression.elements() {
            let mut new_elements: Vec<Expression> = Vec::new();
            for element in elements {
                match self.has_match(element.clone()) {
                    Some(rule) => new_elements.push(rule(element)),
                    None => new_elements.push(element)
                };
            }
            expression.from_elements(new_elements)
        } else {
            expression
        }
    }

    /// Applied this transformation once.
    pub fn apply_once(&self, expression: Expression) -> Expression {
        match self.has_match(expression.clone()) {
            Some(rule) => rule(expression.clone()),
            None => expression.clone()
        }
    }

    /// Returns whether the specified expression may be transformed by this
    /// transformation, and the rule that would transform it.
    pub fn has_match(&self, expression: Expression) -> Option<Rule> {
        for (rmatch, rule) in &self.rules {
            if rmatch(expression.clone()) {
                return Some(*rule)
            }
        }
        None
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
