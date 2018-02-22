// ex5.rs
// Make me compile!

enum Reaction<'a> {
    Sad(&'a str),
    Happy(&'a str),
}

fn express(sentiment: Reaction) {
    match sentiment {
        Reaction::Sad(s) => println!(":( {}", s),
        Reaction::Happy(s) => println!(":) {}", s),
    }
}

fn main () {
    let x = Reaction::Happy("It's a great day for Rust!");
    express(x);
    express(x);
    let y = Reaction::Sad("This code doesn't compile yet.");
    express(y);
}
