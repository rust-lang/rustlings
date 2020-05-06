// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)

use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value.cmp(&0) {
            Ordering::Less => Err(CreationError::Negative),
            Ordering::Equal => Err(CreationError::Zero),
            Ordering::Greater => Ok(PositiveNonzeroInteger(value as u64)),
        }
        // if value > 0 {
        //     Ok(PositiveNonzeroInteger(value as u64))
        // } else if value == 0 {
        //     Err(CreationError::Zero)
        // } else {
        //     Err(CreationError::Negative)
        // }
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
