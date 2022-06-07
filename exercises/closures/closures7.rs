// closure7.rs
// You can use patterns in closure (and function) parameters.
// However there is one big difference between the patterns
// you can use in these sorts of places (like let statements)
// and conditional places (like match). It's called Irrefutability.
// Make me compile!

// Execute `rustlings hint closures7` for hints!

// I AM NOT DONE

fn main() {

    let clo = |(definitely, _)| {
        println!("maybe {:?}", definitely);
    };

    clo(("oh yes", "oh no"));

    let maybe = Some("yes");
    let list = vec![maybe];

    list.into_iter().inspect(|Some(value)| {
        println!("maybe {:?}", value);
    });
}