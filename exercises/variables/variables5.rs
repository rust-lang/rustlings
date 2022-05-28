// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // if we want to reuse a varibale name we need to use
                    // Shadowing, this is different from making a var mutable
    println!("Number plus two is : {}", number + 2);
}
