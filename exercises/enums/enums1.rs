// enums1.rs
// No hints this time! ;)

#[derive(Debug)]
enum Message {
    ChangeColor,
    Echo,
    Move,
    Quit,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
