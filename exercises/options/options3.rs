// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.

// I can use the Copy (and clone) typeclasses
//#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // if I use the typeclasses this stay clean
    //let y: Option<Point> = Some(Point { x: 100, y: 200 });
    // or I can use the reference in the pattern, adding the &
    let y: Option<&Point> = Some(&Point { x: 100, y: 200 });

    match y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
