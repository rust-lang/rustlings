#[allow(dead_code)]
#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
}

#[derive(Debug)]
struct Point {
    x: u8,
    y: u8,
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize { w: 10, h: 30 },
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
