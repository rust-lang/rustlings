// Type casting in Rust is done via the usage of the `as` operator.
// Please note that the `as` operator is not only used when type casting.
// It also helps with renaming imports.

// I AM NOT DONE
// The goal is to make sure that the division does not fail to compile
fn average(values: &[f64]) -> f64 {
    let total = values
        .iter()
        .fold(0.0, |a, b| a + b);
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}