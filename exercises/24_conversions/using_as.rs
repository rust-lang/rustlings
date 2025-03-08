// Type casting in Rust is done via the usage of the `as` operator.
// Note that the `as` operator is not only used when type casting. It also helps
// with renaming imports.

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    // TODO: Make a conversion before dividing.
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
