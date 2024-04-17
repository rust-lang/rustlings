// Get a slice out of Array a where the ??? is so that the test passes.

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        let nice_slice = ???

        assert_eq!([2, 3, 4], nice_slice)
    }
}
