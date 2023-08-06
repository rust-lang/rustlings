// primitive_types6.rs
//
// Use a tuple index to access the second element of `numbers`. You can put the
// expression for the second element where ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand
// for a hint.

<<<<<<< HEAD
fn main() {
    let numbers = (1, 2, 3);
    println!("The second number is {}", numbers.1);
=======
// I AM NOT DONE

#[test]
fn indexing_tuple() {
    let numbers = (1, 2, 3);
    // Replace below ??? with the tuple indexing syntax.
    let second = ???;

    assert_eq!(2, second,
        "This is not the 2nd number in the tuple!")
>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e
}
