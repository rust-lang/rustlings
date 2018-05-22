use title;
use verify;

// Variables in Rust are defined using the "let" keyword. Like this:

fn exercise_one() {
    let x = 5;
    verify(5, x);
    //      ^  ^
    //      |  |
    // What's  The variable
    //  in it  name
}

// Try to replace the "0" with the value of the variable, then run
// "cargo run" and see if it was correct!

// Here's a more complicated example:

fn guess_me() -> &'static str {
    let x = 10;
    if x == 10 {
        return "Ten!";
    } else {
        return "Not ten!";
    }
}

fn exercise_two() {
    let result = guess_me();
    verify("REPLACE ME", result);
}

pub fn exec() {
    title("Variables: Exercise 1");
    exercise_one();
    title("Variables: Exercise 2");
    exercise_two();
}
