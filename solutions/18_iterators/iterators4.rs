// 3 possible solutions are presented.

// With `for` loop and a mutable variable.
fn factorial_for(num: u64) -> u64 {
    let mut result = 1;

    for x in 2..=num {
        result *= x;
    }

    result
}

// Equivalent to `factorial_for` but shorter and without a `for` loop and
// mutable variables.
fn factorial_fold(num: u64) -> u64 {
    // Case num==0: The iterator 2..=0 is empty
    //              -> The initial value of `fold` is returned which is 1.
    // Case num==1: The iterator 2..=1 is also empty
    //              -> The initial value 1 is returned.
    // Case num==2: The iterator 2..=2 contains one element
    //              -> The initial value 1 is multiplied by 2 and the result
    //                 is returned.
    // Case num==3: The iterator 2..=3 contains 2 elements
    //              -> 1 * 2 is calculated, then the result 2 is multiplied by
    //                 the second element 3 so the result 6 is returned.
    // And so on…
    #[allow(clippy::unnecessary_fold)]
    (2..=num).fold(1, |acc, x| acc * x)
}

// Equivalent to `factorial_fold` but with a built-in method that is suggested
// by Clippy.
fn factorial_product(num: u64) -> u64 {
    (2..=num).product()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial_for(0), 1);
        assert_eq!(factorial_fold(0), 1);
        assert_eq!(factorial_product(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial_for(1), 1);
        assert_eq!(factorial_fold(1), 1);
        assert_eq!(factorial_product(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial_for(2), 2);
        assert_eq!(factorial_fold(2), 2);
        assert_eq!(factorial_product(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial_for(4), 24);
        assert_eq!(factorial_fold(4), 24);
        assert_eq!(factorial_product(4), 24);
    }
}
