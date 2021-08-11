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
// you can do this!

fn main() {}

trait AppendBar {
    fn append_bar(self) -> Self;
}

//TODO: Add your code here
impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut new_vec = Vec::new();
        for item in self {
            new_vec.push(format!("{}", item));
            new_vec.push(format!("{}", String::from("Bar")));
        }
        new_vec
    }
}

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
