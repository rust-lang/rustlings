// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    Quit,
    Echo { x: i32, y: i32 },
    Move(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo{x:1,y:2});
    println!("{:?}", Message::Move(String::from("dddd")));
    println!("{:?}", Message::ChangeColor(255,255, 0));
}
