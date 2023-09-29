// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// I AM DONE

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // Mutable borrow can not occur more than twice.
    let z = &mut x;
    // *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
