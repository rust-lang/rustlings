// iterators4.rs
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a hint.

pub fn factorial(num: u64) -> u64 {
    let _not_use: u64 = (1..=num).product();
    // result is the same
    (1..=num).fold(1, |acc, v| acc * v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
