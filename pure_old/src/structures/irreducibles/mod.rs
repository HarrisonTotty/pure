//! Irreducible (Atomic) Structures
//!
//! Irreducible structures (commonly referred as "atomic" structures) are those
//! which do not contain any elements.

mod boolean;
mod integer;
mod symbol;

pub use boolean::Boolean;
pub use integer::Integer;
pub use symbol::Symbol;

pub use integer::integer;
pub use symbol::symbol;
