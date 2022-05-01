// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y), // It seems that the only issue was that we were referencing y (below) without addressing the fact that it was partially moved?
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
