// functions4.rs
// Execute `rustlings hint functions4` or use the `hint` watch subcommand for a hint.

// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// (Don't worry about the function bodies themselves, we're only interested
// in the signatures for now. If anything, this is a good way to peek ahead
// to future exercises!)

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: u32) -> u32 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: u32) -> bool {
    num % 2 == 0
}
