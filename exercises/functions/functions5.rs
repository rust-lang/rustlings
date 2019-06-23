// functions5.rs
// Make me compile! Scroll down for hints :)

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num;
}





























// This is a really common error that can be fixed by removing one character.
// It happens because Rust distinguishes between expressions and statements: expressions return
// a value based on its operand, and statements simply return a () type which behaves just like `void` in C/C++ language.
// We want to return a value of `i32` type from the `square` function, but it is returning a `()` type...
// They are not the same. There are two solutions:
// 1. Add a `return` ahead of `num * num;`
// 2. remove `;`, make it to be `num * num`
