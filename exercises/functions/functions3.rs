// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

fn main() {
    call_me(None);
}

fn call_me(num: Option<i32>) {
    for i in 0..num.unwrap_or(0) {
        println!("Ring! Call number {}", i + 1);
    }
}
