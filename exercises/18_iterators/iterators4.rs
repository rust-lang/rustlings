// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.

// NOTE: 難

pub fn factorial(num: u64) -> u64 {
    // Complete this function to return the factorial of num
    // Do not use:
    // - return
    // Try not to use:
    // - imperative style loops (for, while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    // Execute `rustlings hint iterators4` for hints.

    struct Num {
        current: u64,
        end: u64
    }

    impl Iterator for Num {
        type Item = u64;
        fn next(&mut self) -> Option<Self::Item>{
            if self.current > self.end {
                return None;
            }

            let num = self.current;
            self.current += 1;
            
            Some(num)
        }
    }

    let num_iter = Num {
        current: 1,
        end: num
    };

    let total = num_iter.fold(1, |acc, x| acc * x);

    total
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
