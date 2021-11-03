//! Trigonometric Operations
//!
//! The following module contains structure defintions for the various
//! trigonometric operations, such as `Sine` and `Cosine`.

mod arccosecant;
mod arccosine;
mod arccotangent;
mod arcsecant;
mod arcsine;
mod arctangent;
mod cosecant;
mod cosine;
mod cotangent;
mod secant;
mod sine;
mod tangent;

pub use arccosecant::Arccosecant;
pub use arccosine::Arccosine;
pub use arccotangent::Arccotangent;
pub use arcsecant::Arcsecant;
pub use arcsine::Arcsine;
pub use arctangent::Arctangent;
pub use cosecant::Cosecant;
pub use cosine::Cosine;
pub use cotangent::Cotangent;
pub use secant::Secant;
pub use sine::Sine;
pub use tangent::Tangent;

pub use arccosecant::arccsc;
pub use arccosine::arccos;
pub use arccotangent::arccot;
pub use arcsecant::arcsec;
pub use arcsine::arcsin;
pub use arctangent::arctan;
pub use cosecant::csc;
pub use cosine::cos;
pub use cotangent::cot;
pub use secant::sec;
pub use sine::sin;
pub use tangent::tan;
