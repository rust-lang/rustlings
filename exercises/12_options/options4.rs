#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // Exercise 1: Use ok_or to convert optional_point to a Result.
    // If the point exists, print its coordinates.
    // If it doesn't, print the error message "Point does not exist".
    // TODO: Fix the match statement to handle Result properly.

    let result_point = optional_point.ok_or(???);
    match result_point {
        Ok(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        Err(e) => println!("{}", e),
    }

    // Exercise 2: Use ok_or_else to achieve the same functionality as above,
    // but with a closure to generate the error message.
    // TODO: Fix the match statement to handle Result properly.
    let result_point_else = optional_point.ok_or_else(???);
    match result_point_else {
        Ok(p) => println!("Co-ordinates are {},{}", p.x, p.y),
        Err(e) => println!("{}", e),
    }

    // Exercise 3: Use and_then to chain operations on optional_point.
    // If the point exists, compute the distance from the origin and return it as an Option.
    // If it doesn't, return None.
    let distance_from_origin = optional_point.and_then(
        ???{
        let distance = ((p.x.pow(2) + p.y.pow(2)) as f64).sqrt();
        Some(distance)
    }
    );
    // TODO: Fix the match statement to handle Option properly.
    match distance_from_origin {
        Some(distance) => println!("Distance from origin is {:.2}", distance),
        None => println!("No point to calculate distance"),
    }

    println!("{optional_point:?}"); // Don't change this line.
}
