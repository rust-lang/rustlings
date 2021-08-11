// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// Exercise explanation
//you can declare a new variable with the same name as a previous variable.
// Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used.
// We can shadow a variable by using the same variable’s name and repeating the use of the let keyword

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}
