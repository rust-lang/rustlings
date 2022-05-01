// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` for hints!

// Looking at the diff of this file and my changes should give enough exposition.

struct Wrapper<T> { // We modify this to accept any type T, and then we'll implement it down below
    value: T,
}

impl<u32> Wrapper<u32> { // so here you'll notice that I provide impl with a type, as I also specify a type for the wrapper, this allows us to handle the u32 case.
    pub fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
