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

trait AppendBar {
    fn append_bar(self) -> Self;
}


impl AppendBar for &mut Vec<String> {
    fn append_bar(self) -> Self {
        self.push("Bar".to_owned());
        println!("mut one");
        return self
    }
}

impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut v : Vec<String> = vec![];
        for s in self {
            v.push(s.to_string());
        }
        println!("non mut one");
        v.push("Bar".to_owned());
        return v
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        let mut a = vec![String::from("Foo")];
        let mut a = (&mut a).append_bar();
        
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
        assert_eq!(a.pop().unwrap(), String::from("Bar"));
        assert_eq!(a.pop().unwrap(), String::from("Foo"));
    }
}
