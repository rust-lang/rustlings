// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "3"; // don't change this line
    println!("Number {}", number);
    {
        let number = &(33).to_string();
        println!("Number {}", number);
    }
}
