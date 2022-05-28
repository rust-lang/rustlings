// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// Need to pass in an unsigned 32 bit num to get this to work.
fn main() {
    call_me(10);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
