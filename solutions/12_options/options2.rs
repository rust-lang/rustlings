fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn if_let() {
        let text = "learning rust with rustlings";
        let optional_index = text.find("rustlings");
        let mut found_index = None;
        assert_eq!(optional_index, Some(19));

        // Run the block only when `optional_index` contains an index.
        if let Some(index) = optional_index {
            found_index = Some(index);
        }

        assert_eq!(found_index, Some(19));
    }

    #[test]
    fn while_let() {
        let mut numbers = vec![1, 2];
        numbers.push(3);
        let mut sum = 0;

        // `pop` returns `Some(number)` until the vector is empty.
        while let Some(number) = numbers.pop() {
            sum += number;
        }

        assert_eq!(sum, 6);
    }

    #[test]
    fn nested_options() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // The outer `Some` matches `pop`, and the inner one matches the value
        // stored in the vector. The loop stops on either layer of `None`.
        while let Some(Some(_)) = optional_integers.pop() {
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
