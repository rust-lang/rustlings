#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    // These are simple enum variants (no values).
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
