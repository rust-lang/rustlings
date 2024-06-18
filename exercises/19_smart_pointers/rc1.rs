// rc1.rs
//
// 在這個練習中，我們想要通過 Rc<T> 類型來表達多個所有者的概念。這是一個我們的太陽系模型 - 有一個 Sun 類型和多個行星。行星擁有太陽，表示它們圍繞著太陽旋轉。
//
// 通過使用適當的 Rc 原語來使這個代碼編譯，表示太陽有多個所有者。
//
// 執行 `rustlings hint rc1` 或使用 `hint` 子命令以獲取提示。

// I AM NOT DONE

use std::rc::Rc;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

#[test]
fn main() {
    let sun = Rc::new(Sun {});
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 1 個引用

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 2 個引用
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 3 個引用
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 4 個引用
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 5 個引用
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 6 個引用
    jupiter.details();

    // TODO
    let saturn = Planet::Saturn(Rc::new(Sun {}));
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 7 個引用
    saturn.details();

    // TODO
    let uranus = Planet::Uranus(Rc::new(Sun {}));
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 8 個引用
    uranus.details();

    // TODO
    let neptune = Planet::Neptune(Rc::new(Sun {}));
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 9 個引用
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 8 個引用

    drop(uranus);
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 7 個引用

    drop(saturn);
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 6 個引用

    drop(jupiter);
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 5 個引用

    drop(mars);
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 4 個引用

    // TODO
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 3 個引用

    // TODO
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 2 個引用

    // TODO
    println!("引用計數 = {}", Rc::strong_count(&sun)); // 1 個引用

    assert_eq!(Rc::strong_count(&sun), 1);
}
