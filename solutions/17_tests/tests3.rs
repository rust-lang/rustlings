struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // Don't change this function.
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // Returning a `Result` would be better here. But we want to learn
            // how to test functions that can panic.
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // Check width
        assert_eq!(rect.height, 20); // Check height
    }

    #[test]
    #[should_panic] // Added this attribute to check that the test panics.
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic] // Added this attribute to check that the test panics.
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
