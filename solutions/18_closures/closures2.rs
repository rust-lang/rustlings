// closures2.rs
//
// How do closures capture their state? Well, the answer is "it depends on how you use it!"
//
// Usage inside the closure body will tell the compiler how the value should be captured.
//
// Capture by shared reference? Mutable reference? Ownership? Let's try and see!
//
// Execute `rustlings hint closures2` or use the `hint` watch subcommand for a hint.

fn main() {
    // Using a non-Copy type because it makes reasoning about capturing easier
    let s = String::from("Hello, rustlings!");
    let capture_by_ref = || {
        println!("{s}"); // This only requires a &String, so it only captures a &String
    };
    // You can continue to use s as a &String outside the closure, but not &mut String or String.
    println!("Outside capture_by_ref closure: {s}");
    capture_by_ref();

    // Notice the mut here
    //   v
    let mut s = String::from("Hello, rustlings!");
    let mut capture_by_mut = || {
        s.truncate(5); // Requires &mut String: also can be written as String::truncate(&mut s, 5);
        println!("{s}"); // This should print nothing (and a line break)
                         // Since the "most" we need is mutable, it captures a single mutable reference to String.
    };
    capture_by_mut();

    let mut s = String::from("Hello, rustlings!");
    let capture_by_ownership = || {
        s.truncate(5); // Requires &mut String
        println!("{s}"); // This should print nothing (and a line break)
        let boxed = s.into_boxed_str(); // Requires ownership: String::into_boxed_str(s);
        println!("{boxed}"); // This should print nothing (and a line break)
    };
    capture_by_ownership();

    let s = String::from("Hello, rustlings!");
    let quiz = || {
        let captured_s = &s; // Using a shared reference fixes the issue.
        println!("Inside Closure quiz {captured_s}");
    };
    println!("Outside Closure quiz {s}");
    quiz();
}
