// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move,
    ChangeColor {
        color: String,
    }
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("str".to_string()));
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor{color:"test".to_string()});
}
