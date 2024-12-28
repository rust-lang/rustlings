#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // Solution 1: Matching over the `Option` (not `&Option`) but without moving
    // out of the `Some` variant.
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        //   ^^^ added
        _ => panic!("No match!"),
    }

    // Solution 2: Matching over a reference (`&Option`) by added `&` before
    // `optional_point`.
    match &optional_point {
        //^ added
        Some(p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}");
}
