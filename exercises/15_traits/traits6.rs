// TODO: Design a trait that enables you to calculate the digit sum over both an
// integer as well as a string.

// TODO: Change this function signature so it works with both integers and
// strings, based on your trait.
fn calculate_digit_sum(number: ???) -> usize {
    // TODO: Calculate the digit sum of `number`.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integer() {
        assert_eq!(calculate_digit_sum(0), 1);
        assert_eq!(calculate_digit_sum(1), 1);
        assert_eq!(calculate_digit_sum(1235), 11);
        assert_eq!(calculate_digit_sum(789), 24);
    }

    #[test]
    fn string() {
        assert_eq!(calculate_digit_sum(""), 1);
        assert_eq!(calculate_digit_sum("1"), 1);
        assert_eq!(calculate_digit_sum("1235"), 11);
        assert_eq!(calculate_digit_sum("789"), 24);
    }
}
