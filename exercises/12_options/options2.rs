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
        let placeholder: Option<usize> = None;
        assert_eq!(optional_index, Some(19));

        // TODO: Replace `placeholder` with the optional value defined above.
        if let Some(index) = placeholder {
            found_index = Some(index);
        }

        assert_eq!(found_index, Some(19));
    }

    #[test]
    fn while_let() {
        let mut numbers = vec![1, 2];
        numbers.push(3);
        let mut sum = 0;
        let placeholder: Option<i32> = None;

        // TODO: Replace `placeholder` with an expression that removes and
        // returns the last element of `numbers`.
        while let Some(number) = placeholder {
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

        // TODO: Add another `Some` to the pattern so that the loop stops when
        // it encounters the `None` stored in the vector.
        while let Some(_) = optional_integers.pop() {
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
