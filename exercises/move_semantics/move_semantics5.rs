// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

// TOTHINK

fn main() {
    let mut x = 100;

    let z = &mut x;
    *z += 1000;
    
    let y = &mut x; //cannot borrow `x` as mutable more than once at a time
    *y += 100;


    assert_eq!(x, 1200);
}
