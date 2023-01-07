// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x = 100;
    let y = &mut x;
    let y2 = &mut x;
    let z = &mut x;
     asd(z,1100);
    assert_eq!(x, 1200);
}

fn asd(r:&mut i32, n:i32) {
    *r += n;
}