trait Digits {
    fn digits(&self) -> Vec<usize>;
}

impl Digits for usize {
    fn digits(&self) -> Vec<usize> {
        let mut res = Vec::new();
        let mut temp = *self;
        while temp != 0 {
            res.push(temp % 10);
            temp /= 10;
        }
        res.reverse();
        res
    }
}

impl Digits for &str {
    fn digits(&self) -> Vec<usize> {
        self.chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect()
    }
}

fn calculate_digit_sum(number: impl Digits) -> usize {
    number.digits().into_iter().sum()
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
