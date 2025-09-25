// Structs contain data, but can also have logic. In this exercise, we have
// defined the `Fireworks` struct and a couple of functions that work with it.
// Turn these free-standing functions into methods and associated functions
// to express that relationship more clearly in the code.

#![deny(clippy::use_self)] // practice using the `Self` type

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

// TODO: Turn this function into an associated function on `Fireworks`.
fn new_fireworks() -> Fireworks {
    Fireworks { rockets: 0 }
}

// TODO: Turn this function into a method on `Fireworks`.
fn add_rockets(fireworks: &mut Fireworks, rockets: usize) {
    fireworks.rockets += rockets
}

// TODO: Turn this function into a method on `Fireworks`.
fn start(fireworks: Fireworks) -> String {
    if fireworks.rockets < 5 {
        String::from("small")
    } else if fireworks.rockets < 20 {
        String::from("medium")
    } else {
        String::from("big")
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
