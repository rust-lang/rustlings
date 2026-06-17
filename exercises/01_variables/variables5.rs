fn main() {
    let number = "T-H-R-E-E"; // Don't change this line
    println!("Spell a number: {number}");

    // TODO: Fix the compiler error by making `number` mutable.
    // This exercise is about mutability, not shadowing.
    number = 3;
    println!("Number plus two is: {}", number + 2);
}
