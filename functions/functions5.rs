// Make me compile! Scroll down for hints :)

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num
}





























// This is a really common error that can be fixed by removing one character.
// It happens because Rust distinguishes between expressions and statements: expressions return
// a value and statements don't. We want to return a value from the `square` function, but it
// isn't returning one right now...
