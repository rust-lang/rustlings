// variables4.rs
//
// Execute `rustlings hint variables4` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {
    // Variable bindings are immutable by default
    // 变量绑定默认是不可变的
    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}
