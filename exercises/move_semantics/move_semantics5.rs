// move_semantics5.rs
// Make me compile without adding, removing, or changing any of the
// lines in `main()`.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut *y;
    *z += 1000;
    *y += 100;
    assert_eq!(x, 1200);
}
