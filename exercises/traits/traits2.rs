// traits2.rs
// 
// Your task is to implement the trait
// `AppendBar' for a vector of strings.
// 
// To implement this trait, consider for
// a moment what it means to 'append "Bar"'
// to a vector of strings.
// 
// No boiler plate code this time,
// you can do this! Hints at the bottom.

trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

}
















































// Notice how the trait takes ownership of 'self',
// and returns `Self'. Try mutating the incoming
// string vector.
//
// Vectors provide suitable methods for adding
// an element at the end. See the documentation
// at: https://doc.rust-lang.org/std/vec/struct.Vec.html
