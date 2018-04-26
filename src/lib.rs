#[allow(dead_code)]
mod about_variables {
    pub fn guess_this () -> i32 {
        let one = 5;
        let two = 7;
        let three = 3;
        let result = (one + two) / three;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::about_variables::*;

    #[test]
    fn test_complicated () {
        assert_eq!(___, guess_this());
    }
}
