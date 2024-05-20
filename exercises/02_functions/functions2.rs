


// functions2.rs
//
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me(3);
}

// NOTE: It is interesting that the function is defined below main but main can still call this.
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
