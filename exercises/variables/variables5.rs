// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number: u32 = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
    {
        let number: f32 = 0.1;
        println!("Number plus 1 is : {}", number + 1.0)
    }
    println!("Number plust 0.1 is : {}", number + 1.1 as u32)
}
