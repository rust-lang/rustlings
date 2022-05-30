// closure2.rs
// What is the difference between a closure and a function? Can you get this code
// to compile?

// Execute `rustlings hint closures2` for hints!

// I AM NOT DONE

fn output(closure) {
    println!("What am I?");
    closure();
}

fn main() {

    fn actual() {
        println!("I'm actually a function")
    }

    let actual_fn = actual;
    let closure = || println!("I'm a closure");
    output(closure);
    output(actual_fn);
}