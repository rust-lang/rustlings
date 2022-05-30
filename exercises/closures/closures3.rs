// closure3.rs
// What is the difference between a closure and a function, part II? Can you get this code
// to compile?

// Execute `rustlings hint closures3` for hints!

// I AM NOT DONE

fn main() {
    let my_name = String::from("Lim Lady");

    fn actual() {
        println!("my name is: {}", my_name);
    }

    let actual_fn = actual;
    let closure = || {
        println!("my name is: {}", my_name);
    };

    println!("Who am I?");
    actual_fn();
    closure();
}