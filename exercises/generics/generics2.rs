// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// I AM NOT DONE
struct Wrapper<u32> {
    value: u32
}

impl<u32> Wrapper<u32> {
    pub fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value,  42);
    }

    #[test]
    fn store_str_in_wrapper() {
        // TODO: Delete this assert and uncomment the one  below once you have  finished the exercise.
        assert!(false);
        // assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}