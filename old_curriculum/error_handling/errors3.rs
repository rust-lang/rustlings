// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though--
// we can't call the `try!` macro in the `main()` function! Why not?
// What should we do instead? Scroll for hints!

use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = try!(total_cost(pretend_user_input));

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = try!(item_quantity.parse::<i32>());

    Ok(qty * cost_per_item + processing_fee)
}


















// Since the `try!` macro returns an `Err` early if the thing it's trying to
// do fails, you can only use the `try!` macro in functions that have a
// `Result` as their return type.

// The error that you get if you run this code is:

// ```
// error: mismatched types:
// expected `()`,
// found `std::result::Result<_, _>`
// ```

// which is saying that the expected return type of the `main` function is
// the empty tuple, but we tried to return a `Result`-- and that's happening
// in the implementation of `try!`. The `main` function never has a return type,
// so we have to use another way of handling a `Result` within `main`.

// Decide what we should do if `pretend_user_input` has a string value that does
// not parse to an integer, and implement that instead of calling the `try!`
// macro.
