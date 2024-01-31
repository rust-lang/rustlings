// functions1.rs
//
// Execute `rustlings hint functions1` or use the `hint` watch subcommand for a
// hint.


fn main() {
    println!("You define functions outside of main and call them here.");
    println!("Call me.");
    // Rust doesn't care where the function is defined, as long as it's within the scope.
    call_me();
}

fn call_me() {
    println!("Beep me. If you wanna reach me.");
}