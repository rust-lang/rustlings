fn main() {}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore] //this line simply ignores the test
    fn ignore_test() {
        panic!("The test wasn't ignored");
    }
}
