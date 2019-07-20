// functions4.rs
// Make me compile! Scroll down for hints :)

// This store is having a sale where if the price is an even number, you get
// 10 (money unit) off, but if it's an odd number, it's 3 (money unit) less.

fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}

fn sale_price(price: i32) -> i32{
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn is_even(num: i32) -> bool {
    num % 2 == 0
}



















// The error message points to line 12 and says it expects a type after the
// `->`. This is where the function's return type should be-- take a look at
// the `is_even` function for an example!
