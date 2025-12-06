//fn avg<???>(???) -> ???
// TODO: write a fuction that takes in a slice of number-like primitives, eg u8, i16, usize
//      and returns the mean of the slice
//      you do not to implement  this for floats due to a language limitation

fn main() {
    // You can optionally experiment here.
}

//you may add `.unwrap()` to the avg fuction calls if needed
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let input: [u8; 5] = [2, 4, 6, 8, 10];
        let ans: u8 = avg(&input);
        assert_eq!(ans, 6);
    }

    fn test_i32() {
        let input: [i32; 5] = [546, 263, 8764, 4198, 7654];
        let ans: i32 = avg(&input);
        assert_eq!(ans, 4285);
    }
}
