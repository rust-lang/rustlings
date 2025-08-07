#[derive(Debug)]
enum Message {
    // TODO: Define the message types used below.
    // These are simple enum variants with no data.
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}