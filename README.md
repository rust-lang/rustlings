# rustlings

A cool thing that is currently in development.

## How it's structured

Ideally, like RubyKoans, all exercises can be run by executing one command, in this case
`cargo run` (most likely). This runs `src/main.rs`, which in turn runs all of the exercises.
Each exercise is contained in a Rust file called `about_<exercise topic>.rs`. A minimal exercise looks
somewhat like this:

```rust
fn exercise_function() {
  "hello"
}

mod tests {
  use super::*;
  
  pub fn test() {
    verify!("REPLACE ME", exercise_function(), "Function description");
  }
}

pub fn exec() {
  tests::test();
}
```

Each exercise file is supposed to have one `exec` function which gets called by the `main.rs` file.
This function, in turn, calls all individual test functions.

The tests themselves can generally be structured in whatever way is desired. Two macros are provided
for convenience. The `verify!` macro is essentially a specialized `assert_eq!`, but it doesn't panic
if the values mismatch, instead it prints out a helpful error message and keeps going. The
`verify_easy!` macro is designed as a drop-in replacement for the `verify!` macro for if the learner needs help solving the exercise. It prints the expected value, too.

This is roughly what the console output for a simple exercise looks right now:

![](https://i.imgur.com/gGgjvLW.png)

Keep in mind that this is a very early draft of how things work. Anything here might be changed
at any time, and this documentation should be updated accordingly.

