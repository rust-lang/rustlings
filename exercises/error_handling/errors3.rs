// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!


use std::num::ParseIntError;

fn main() -> Result<(),ParseIntError> { // This isn't too bad, main should return (), so we just float the result structure up to main.
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?; // Here we're making cost a result

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(()) // :facepalm: So, I was on https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html at the final example.
            // Wish I had read further
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
