//! Arithmetic Operations
//!
//! The following module provides structure definitions for the various
//! elementary arithmetic operations, such as `Addition` and `Division`.

mod addition;
mod division;
mod multiplication;
mod subtraction;

pub use addition::Addition;
pub use division::Division;
pub use multiplication::Multiplication;
pub use subtraction::Subtraction;

pub use addition::addition;
pub use division::division;
pub use multiplication::multiplication;
pub use subtraction::subtraction;
