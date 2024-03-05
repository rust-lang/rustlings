// options3.rs
//
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a
// hint.


struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });



    
    // match y {
    //     Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
    //     _ => panic!("no match!"),
    // }
    // y; // Fix without deleting this line.

    //  By default, match statements consume all they can, which can sometimes be a problem, when you don’t really need the value to be moved and owned: you just need to borrow it.
    //  The ref keyword can be used to take references to the values in the pattern. This is useful when you want to match on a value, but don’t want to take ownership of it.
    //  The ref keyword can be used in the pattern to take references to the values in the pattern. This is useful when you want to match on a value, but don’t want to take ownership of it.

    match y {
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
