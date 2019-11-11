// modules2.rs
// Make me compile! Scroll down for hints :)

mod delicious_snacks {
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}

// The delicious_snacks module is trying to present an external
// interface (the `fruit` and `veggie` constants) that is different than
// its internal structure (the `fruits` and `veggies` modules and
// associated constants). It's almost there except for one keyword missing for
// each constant.
