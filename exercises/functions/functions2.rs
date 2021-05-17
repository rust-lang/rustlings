// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)


fn main() {
	let num = 3;
    call_me(num);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
