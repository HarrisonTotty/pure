//! pure
//!
//! A symbolic mathematics library for rust.

#![feature(unboxed_closures)]
#![feature(fn_traits)]

pub mod core;
pub mod prelude;
pub mod structures;
pub mod transformations;

#[macro_export]
macro_rules! mkstructure {
    {$i:ident, 0, $s:literal} => {
        #[derive(Clone, Debug)]
        pub struct $i;
        impl crate::core::Structure for $i {
            fn from_elements(&self, elements: crate::core::Expressions) -> crate::core::Expression {
                self.clone()
            }
            fn value(&self) -> crate::core::Value {
                crate::core::Value::Kind(stringify!($i))
            }
        }
        impl std::convert::From<$i> for crate::core::Expression {
            fn from(op: $i) -> crate::core::Expression {
                crate::core::Expression(std::rc::Rc::new(op))
            }
        }
        impl std::fmt::Display for $i {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $s)
            }
        }
    };
    {$i:ident, 1, $s:literal, $f:ident} => {
        #[derive(Clone, Debug)]
        pub struct $i(pub crate::core::Expression);
        impl crate::core::Structure for $i {
            fn from_elements(&self, elements: crate::core::Expressions) -> crate::core::Expression {
                $i(elements.first().unwrap().to_owned()).into()
            }
            fn value(&self) -> crate::core::Value {
                crate::core::Value::UnaryOperation(String::from(stringify!($i)), self.0.clone())
            }
        }
        impl std::convert::From<$i> for crate::core::Expression {
            fn from(op: $i) -> crate::core::Expression {
                crate::core::Expression(std::rc::Rc::new(op))
            }
        }
        impl std::fmt::Display for $i {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $s, self.0)
            }
        }
        pub fn $f(expression: crate::core::Expression) -> crate::core::Expression {
            $i(expression).into()
        }
    };
    {$i:ident, 2, $s:literal, $f:ident} => {
        #[derive(Clone, Debug)]
        pub struct $i(pub crate::core::Expression, pub crate::core::Expression);
        impl crate::core::Structure for $i {
            fn from_elements(&self, elements: crate::core::Expressions) -> crate::core::Expression {
                $i(elements.first().unwrap().to_owned(), elements.last().unwrap().to_owned()).into()
            }
            fn value(&self) -> crate::core::Value {
                crate::core::Value::BinaryOperation(String::from(stringify!($i)), self.0.clone(), self.1.clone())
            }
        }
        impl std::convert::From<$i> for crate::core::Expression {
            fn from(op: $i) -> crate::core::Expression {
                crate::core::Expression(std::rc::Rc::new(op))
            }
        }
        impl std::fmt::Display for $i {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, $s, self.0, self.1)
            }
        }
        pub fn $f(first: crate::core::Expression, last: crate::core::Expression) -> crate::core::Expression {
            $i(first, last).into()
        }
    };
}
