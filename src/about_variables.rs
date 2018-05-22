// Welcome to Rustlings! If you're here, that means you've either successfully
// downloaded Rustlings, or are looking at this on GitHub. Either way, let me
// introduce you to one of the most basic elements of Rust:
//
// === VARIABLES ===
//
// Variables are essentially little containers that hold, well, something. Think
// of them as a little cardboard box that you put stuff into. What can you put
// into a virtual cardboard box in Rust? All kinds of stuff, it turns out!
// Numbers, words, sequences, and much more. Let's start out simple, though.
// Here's our first exercise:

pub fn exercise_one() {
    let x = 5;
    verify!(0, x, "Number assignment");
    //      ^  ^
    //      |  |
    // What's  The variable
    //  in it  name
}

// Did you get all that? The "let" word basically tells us that we now want to
// define a variable, and what follows it (the "x") is the name of the variable.
// Each variable has a name, like a label you put on your cardboard box so you
// don't confuse it with another, similar looking one.
// The whole "verify!" deal essentially means that Rustlings is checking if you
// solved the exercise correctly. It compares the first argument with the
// second, so in this case "0" with "x", where "x" is the *value* of the variable
// we called "x". When you write "x", you pull out the cardboard box labelled "x"
// and take out what's inside of it.
// Speaking of which, what *is* inside of our "x" cardboard box? I don't think it's
// "0"... do you know? Replace the "0" with the value of the variable we defined.
// After that, run "cargo run" in your command line, and see if you put in the
// right answer.

pub fn exec() {
    exercise_one();
}
