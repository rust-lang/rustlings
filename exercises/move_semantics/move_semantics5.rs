// move_semantics5.rs
// Make me compile without adding any newlines or removing any of the lines.
// Execute `rustlings hint move_semantics5` for hints :)


fn main() {
    let mut x = 100;
    {
        let y = &mut x;
        *y += 100;
    }
    let z = &mut x;
    // *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
