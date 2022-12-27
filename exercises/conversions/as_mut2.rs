// AsMut allows for cheap reference-to-reference conversions.
// Read more about it at https://doc.rust-lang.org/std/convert/trait.AsMut.html.
//
// In conversions/as_mut1.rs, we implemented a function that would square a
// Box<u32> in-place using as_mut(). Now we're going to generalize the function
// to work with a Box containing any numeric type that supports multiplication 
// and assignment.
//
// Execute `rustlings hint as_mut2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

// Squares a number using as_mut().
// TODO: Add the appropriate trait bounds.
fn num_sq<T, U>(arg: &mut T) 
where 
    T: ???,
    U: ???,
{
    // TODO: Implement the function's body.
    ???
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mult_box_u32() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }

    #[test]
    fn mult_box_f32() {
        let mut num: Box<f32> = Box::new(3.0);
        num_sq(&mut num);
        assert_eq!(*num, 9.0);
    }
}
