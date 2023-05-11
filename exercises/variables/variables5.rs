// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.




// // Using parsing:
// fn main() {
//     let mut number = "T-H-R-E-E"; // don't change this line
//     println!("Spell a Number : {}", number);
//     number = "3"; // don't rename this variable
//     println!("Number plus two is : {}", number.parse::<i32>().unwrap() + 2);
// }


// Using shadowing:
fn main() {
    {
        let number = "T-H-R-E-E"; // don't change this line
        println!("Spell a Number : {}", number);
    }
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}