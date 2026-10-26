// closures1.rs
//
// "Why do we even need closures?" is a question that gets asked from time to time.
// While it's true that most things that closures can do can also be done with
// regular old structs and enums, closures can make things a lot more clear with a lot
// less clutter compared to structs.
//
// Below is a good example of how one could implement a capturing closure using structs,
// and how closures simplifies this greatly.
//
// Execute `rustlings hint closures1` or use the `hint` watch subcommand for a hint.

trait Doorman {
    fn greet_customer(&self, customer_name: &str);
}

struct GreeterWithState<'a> {
    greeting: &'a str,
}

impl Doorman for GreeterWithState<'_> {
    fn greet_customer(&self, customer_name: &str) {
        println!("{}, {}?", self.greeting, customer_name);
    }
}

fn greet_customers(doorman: impl Doorman) {
    doorman.greet_customer("Bill");
    doorman.greet_customer("Alex");
    doorman.greet_customer("John");
    doorman.greet_customer("Jessie");
}

fn greet_customers_closure(doorman: impl Fn(&str)) {
    doorman("Bill");
    doorman("Alex");
    doorman("John");
    doorman("Jessie");
}

fn main() {
    let greeting = String::from("Hello! How are you");

    // Method 1 for passing in functions with state.
    // Just create a struct, store the state, and add a method.
    // If you need to be generic, it can be a trait method.
    let doorman = GreeterWithState {
        greeting: &greeting,
    };
    greet_customers(doorman);

    // Method 2 for passing in functions with state.
    // Notice that the body of this closure is exactly the same
    // as GreeterWithState's Doorman implementation.
    //
    // This makes things much cleaner with less clutter, but
    // we are forgetting something very important.
    greet_customers_closure(|customer_name| {
        println!("{}, {}?", greeting, customer_name); // Capture greeting by reference
    })
}
