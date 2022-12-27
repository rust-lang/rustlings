// AsMut allows for cheap mutable reference-to-reference conversions.
// Read more about it at https://doc.rust-lang.org/std/convert/trait.AsMut.html.
// Execute `rustlings hint as_mut1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.
fn num_sq<T>(arg: &mut T) {
    // TODO: Implement the function's body.
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
