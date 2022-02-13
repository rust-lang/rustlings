// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100;
    let y = &mut x;
    println!("y: {}", y);
    *y += 100;
    println!("y: {}", y);
    let z = &mut x;
    println!("z: {}", z);
    *z += 1000;
    println!("z: {}", z);
    assert_eq!(x, 1200);
}
