// `AsRef` and `AsMut` let a function accept different input types while
// borrowing the value it needs. For example, both `&str` and `String` can be
// viewed as `&str`, and both `u32` and `Box<u32>` can be viewed as `&mut u32`.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// Obtain the number of bytes (not characters) in the given argument. The
// `AsRef<str>` bound allows this function to work with both `&str` and `String`.
// (`.len()` returns the number of bytes in a string.)
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().len()
}

// Obtain the number of characters (not bytes) in the given argument. Reuse the
// same `AsRef<str>` bound so this function also accepts `&str` and `String`.
// TODO: Add the `AsRef` trait appropriately as a trait bound.
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Square a number through a mutable reference. The `AsMut<u32>` bound allows
// this function to work with values such as `Box<u32>` as well as `u32`.
// TODO: Add the `AsMut<u32>` trait bound.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implement the function body.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
