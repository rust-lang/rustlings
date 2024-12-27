// generics2.rs
//
// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

struct Wrapper<T> {
    value: T
}

impl <T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // This empty main function is needed, because the compiler expects it,
    // while rustlings don't use this function for execution.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42u32).value, 42u32);
    }

    #[test]
    fn store_string_in_wrapper() {
      let x = Wrapper::new(String::from("Foo"));
      assert_eq!(x.value, String::from("Foo"));
    }

    #[test]
    fn store_f64_in_wrapper() {
      assert_eq!(Wrapper::new(42.0_f64).value, 42.0_f64);
    }
}
