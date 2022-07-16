// variables4.rs
// Make me compile! Execute the command `rustlings hint variables4` if you want a hint :)

fn main() {
    let x: Option<i32> = Some(5);
    match x {
        Some(x) => println!("Number {}", x),
        None => println!("kein wert"),
    }
}
