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
        if self.rockets < 5 {
            String::from("small")
        } else if self.rockets < 20 {
            String::from("medium")
        } else {
            String::from("big")
        }
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
        let mut f = Fireworks::new();
        f.add_rockets(3);
        assert_eq!(f.start(), "small");

        let mut f = Fireworks::new();
        f.add_rockets(15);
        assert_eq!(f.start(), "medium");

        let mut f = Fireworks::new();
        f.add_rockets(100);
        assert_eq!(Fireworks::start(f), "big");
        // We don't use method syntax in the last test to ensure the `start`
        // function takes ownership of the fireworks.
    }
}
