// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)

fn main() {
    // Just specifying the type is not sufficient as Rust doesn't allow uninitalized variables to be used later.alloc
    // Just specifying the value is fine as type is inferred,
    // but if it doesn't match with the value being compared later, we get error. let x = 10.0;
    // You can always specify both.
    let x = 10;
    if x == 10 {
        println!("Ten!");
    } else {
        println!("Not ten!");
    }
}
