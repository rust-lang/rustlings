#![deny(clippy::use_self)] // practice using the `Self` type

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

impl Fireworks {
    fn new() -> Self {
        Self { rockets: 0 }
    }

    fn add_rockets(&mut self, rockets: usize) {
        self.rockets += rockets
    }

    fn start(self) -> String {
        "🚀".repeat(self.rockets)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_some_fireworks() {
        let f = Fireworks::new();
        assert_eq!(f.start(), "");

        let mut f = Fireworks::new();
        f.add_rockets(3);
        assert_eq!(f.start(), "🚀🚀🚀");

        let mut f = Fireworks::new();
        f.add_rockets(7);
        // We don't use method syntax in the last test to ensure the `start`
        // function takes ownership of the fireworks.
        assert_eq!(Fireworks::start(f), "🚀🚀🚀🚀🚀🚀🚀");
    }
}
