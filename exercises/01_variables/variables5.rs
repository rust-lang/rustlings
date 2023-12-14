// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut number: Result<u8, &str> = Err("T-H-R-E-E"); // don't change this line
    println!("Spell a Number : {:?}", number);
    number = Ok(3); // don't rename this variable
    println!("Number plus two is : {}", number.unwrap() + 2);
}
