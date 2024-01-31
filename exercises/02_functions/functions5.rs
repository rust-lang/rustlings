// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    // you can either add an explicit return to this expression
    // or remove the semicolon to make it an implicit return
    // you cannot just have an expression with a semicolon
    return num * num;
}
