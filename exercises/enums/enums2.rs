// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    Move { x:i32, y:i32 },
    Echo(String),
    ChangeColor(u8,u8,u8),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 }, // ### Assimilate :: as namespace operator for variants
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
