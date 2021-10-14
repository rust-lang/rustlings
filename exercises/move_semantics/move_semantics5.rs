// move_semantics5.rs
// Make me compile only be reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x = 100;
    let y = &mut x;
   
    *y += 100;   //suma 100 a x, x=200

    let z = &mut x;
    *z += 1000; //suma 1000 a x, x=1200
    
    assert_eq!(x, 1200);
}
