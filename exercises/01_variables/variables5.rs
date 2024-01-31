// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);

    // This is shadowing. The compiler will see the second let
    // aka the "shadow". It will overwrite the first unless it's "shadowed"
    // or the scope ends.
    // You will get a compile error if you try to 
}
