// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper {
    value: u32,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // You can optionally experiment here.
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
