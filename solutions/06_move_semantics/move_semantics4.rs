fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        // `y` used here.
        y.push(42);
        // The mutable reference `y` is not used anymore,
        // therefore a new reference can be created.
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
