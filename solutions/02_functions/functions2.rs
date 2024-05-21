// The type of function arguments must be annotated.
// Added the type annotation `u64`.
fn call_me(num: u64) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
