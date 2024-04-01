// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(5);
}

fn call_me(num: i32) {
    let big = if num > 0 { num } else {0};
    let small = if num < 0 { num } else {0};

    for i in small..big {
        println!("Ring! Call number {}", i + 1);
    }
}
