// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

//ref is used to borrow the value without having to match a comletely different type of &Foo
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }

    y; // Fix without deleting this line.
}
