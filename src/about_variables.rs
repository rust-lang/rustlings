#[allow(dead_code)]
pub fn guess_this () -> i32 {
    let one = 5;
    let two = 7;
    let three = 3;
    let result = (one + two) / three;
    return result;
}

mod tests {
    use super::*;

    pub fn test_complicated () {
        assert_eq!(1, guess_this());
    }
}

pub fn exec () {
    tests::test_complicated();
}
