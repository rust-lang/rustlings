fn main() {
    let number = "T-H-R-E-E";
    println!("Spell a number: {number}");

    // Using variable shadowing
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let number = 3;
    println!("Number plus two is: {}", number + 2);
}
