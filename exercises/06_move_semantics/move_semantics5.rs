// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
// Adding the curly braces makes the the scope explicit.
// This is just more readable and clear to people to reading.
fn main() {
    let mut x = 100;
     {
        let y = &mut x;
        *y += 100;
     }
     {
        let z = &mut x;
        *z += 1000;
     }
     assert_eq!(x, 1200);
 }

// fn main() {
   // let mut x = 100;

    // scopes x and here and then y is out of scope afterwards
    //let y = &mut x;
    //*y += 100;

    //let z = &mut x;
    //*z += 1000;
    //assert_eq!(x, 1200);
//}
