use std::iter::Sum;
use std::ops::Div;
use std::convert::TryFrom;

//use Div as we need to div sum by size of slice &[T]
//Use Sum so that .sum() can be used
//Use Copy as .copied() requires it
//use TryFrom to use T::try_from as `as T` is not allowed

fn avg<T>(input: &[T]) -> Result<T, T::Error>
where
    T: Div<Output = T> + Sum + TryFrom<usize> + Copy,
{
    Ok(input.iter().copied().sum::<T>() / T::try_from(input.len())?)
}
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
        let ans: u8 = avg(&input).unwrap();
        assert_eq!(ans, 6);
    }

    fn test_i32() {
        let input: [i32; 5] = [546, 263, 8764, 4198, 7654];
        let ans: i32 = avg(&input).unwrap();
        assert_eq!(ans, 4285);
    }
}
