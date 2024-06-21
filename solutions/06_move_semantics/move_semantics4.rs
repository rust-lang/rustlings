fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics5() {
        let mut x = 100;
        let y = &mut x;
        // `y` used here.
        *y += 100;
        // The mutable reference `y` is not used anymore,
        // therefore a new reference can be created.
        let z = &mut x;
        *z += 1000;
        assert_eq!(x, 1200);
    }
}
