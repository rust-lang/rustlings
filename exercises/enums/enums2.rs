// enums2.rs
// Make me compile! Scroll down for hints

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move{ x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit
    ];

    for message in &messages {
        message.call();
    }
}

































// Hint: you can create enumerations that have different variants with different types
// such as no data, anonymous structs, a single string, tuples, ...etc
