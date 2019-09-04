// traits1.rs
// Time to implement some traits!
// 
// Your task is to implement the trait
// `AppendBar' for the type `String'.
// 
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    //Add your code here

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
    fn is_FooBar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_BarBar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

}





















































// A discussion about Traits in Rust can be found below.
// https://doc.rust-lang.org/1.30.0/book/second-edition/ch10-02-traits.html
