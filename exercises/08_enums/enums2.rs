// enums2.rs
//
// 執行 `rustlings hint enums2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

#[derive(Debug)]
enum Message {
    // TODO: 定義下面使用的不同變體
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
