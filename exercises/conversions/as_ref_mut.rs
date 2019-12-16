// AsRef and AsMut allow for cheap reference-to-reference conversions.
// Read more about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html
// and https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.

// I AM NOT DONE
// Obtain the number of bytes (not characters) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn byte_counter<T>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// I AM NOT DONE
// Obtain the number of characters (not bytes) in the given argument
// Add the AsRef trait appropriately as a trait bound
fn char_counter<T>(arg: T) -> usize {
    arg.as_ref().chars().collect::<Vec<_>>().len()
}

fn main() {
    let s = "Café au lait";
    println!("{}", char_counter(s));
    println!("{}", byte_counter(s));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }
}