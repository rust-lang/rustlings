// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` for hints!


#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = numbers.1; // This is pretty interesting, and a great example of where Rust differs strongly from python. I had guessed the answer was &numbers[1], but the compiler corrected me!

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
}
