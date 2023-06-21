// variables6.rs
// Execute `rustlings hint variables6` or use the `hint` watch subcommand for a hint.



const NUMBER: i32 = 4;
fn main() {
    println!("Number {}", NUMBER);
    println!("added {}",add(1,4));
}

fn add(x: i32, y: i32) ->  i32 {
    x + y
}