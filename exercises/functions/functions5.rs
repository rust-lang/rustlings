// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

// This  funtion implicitly returns, there is no need for the ; after num * num
fn square(num: i32) -> i32 {
    num * num
}
