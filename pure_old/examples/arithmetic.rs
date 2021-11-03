//! Basic Arithmetic

use pure::prelude::*;

fn main() {
    // Define the symbols "x" and "y".
    let x = symbol("x");
    let y = symbol("y");

    // (x + 4) / y
    let expr = (x + 4) / y;
    println!("{}", expr);

    // Evaluate 1 + 1
    //println!("{}", Addition(integer(1), integer(1)).evaluate());
}
