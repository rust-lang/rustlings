// tests4.rs
// Correct the tests to
// Do not change Rectangle::new body
// Execute `rustlings hint tests4` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    pub fn new(width: i32, height: i32) -> Self {
        if width < 0 || height < 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let _rect = Rectangle::new(10, 10);
    }

    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
