// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

fn call_me(num: i8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
