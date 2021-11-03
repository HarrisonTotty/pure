//! Symbol Examples

use pure::prelude::*;

fn main() {
    // Explicit Symbol.into()
    let x: Expression = Symbol(String::from("x")).into();

    // Explicit &str.into()
    let y: Expression = "y".into();

    // Implicit from()
    let z = Expression::from("z");

    // Implicit via prelude helper function
    let q = symbol("q");

    println!("{}, {}, {}, {}", x, y, z, q);
}
