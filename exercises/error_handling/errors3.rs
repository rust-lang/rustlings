// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a hint.

use std::num::ParseIntError;

fn main() {
    let mut tokens = 1000;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input);

    match cost {
        Ok(cost) if cost > tokens => println!("You can't afford that many!"),
        Ok(cost) if cost <= tokens => {
            tokens -= cost;
            println!("You now have {} tokens.", tokens);
        }
        Err(error) => println!("err:{:?}", error),
        _ => println!("other"),
    };
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
