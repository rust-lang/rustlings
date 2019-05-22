// errors2.rs
// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy,
// and the `total_cost` function will calculate the total number of tokens.
// Since the player typed in the quantity, though, we get it as a string-- and
// they might have typed anything, not just numbers!

// Right now, this function isn't handling the error case at all (and isn't
// handling the success case properly either). What we want to do is:
// if we call the `parse` function on a string that is not a number, that
// function will return a `ParseIntError`, and in that case, we want to
// immediately return that error from our function and not try to multiply
// and add.

// There are at least two ways to implement this that are both correct-- but
// one is a lot shorter! Scroll down for hints to both ways.

use std::num::ParseIntError;

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().to_string(),
            "invalid digit found in string"
        );
    }
}

















// One way to handle this is using a `match` statement on
// `item_quantity.parse::<i32>()` where the cases are `Ok(something)` and
// `Err(something)`. This pattern is very common in Rust, though, so there's
// a `?` operator that does pretty much what you would make that match statement
// do for you! Take a look at this section of the Error Handling chapter:
// https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator
// and give it a try!
