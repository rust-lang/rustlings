fn square(num: i32) -> i32 {
    // Removed the semicolon `;` at the end of the line below to implicitly return the result.
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
