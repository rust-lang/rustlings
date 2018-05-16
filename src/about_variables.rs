fn guess_this () -> i32 {
    let one = 5;
    let two = 7;
    let three = 3;
    let result = (one + two) / three;
    return result;
}

fn simple () -> &'static str {
    let hello = "Hello World!";
    return hello;
}

mod tests {
    use super::*;

    pub fn test_simple () {
        verify!("Hello World!", simple(), "Simple example");
    }

    pub fn test_complicated () {
        verify!(1, guess_this(), "Complicated example");
    }
}

pub fn exec () {
    tests::test_simple();
    tests::test_complicated();
}
