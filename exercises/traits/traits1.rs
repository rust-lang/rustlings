// traits1.rs
//
// Time to implement some traits! Your task is to implement the trait
// `AppendBar` for the type `String`. The trait AppendBar has only one function,
// which appends "Bar" to any object implementing this trait.
//
// Execute `rustlings hint traits1` or use the `hint` watch subcommand for a
// hint.

<<<<<<< HEAD
=======
// I AM NOT DONE

>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
<<<<<<< HEAD
    fn append_bar(self) -> Self {
        self + "Bar"
    }
=======
    // TODO: Implement `AppendBar` for type `String`.
>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
