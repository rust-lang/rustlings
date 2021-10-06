// enums2.rs
// Make me compile! Execute `rustlings hint enums2` for hints!

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move{x:i32,y:i32},
    Echo(String),
    ChangeColor(i32,i32,i32),
    Quit
    
}



impl Message {
    fn call(&self) {
        println!("{:?}", &self.Quit);
    }
    fn test(&self) {
        println!("{:?}",self.Echo );
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];
    let t=Message::Echo(String::from("test"));
    t.test();
    for message in &messages {
        message.call();
    }
}
