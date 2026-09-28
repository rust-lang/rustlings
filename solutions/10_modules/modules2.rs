mod delicious_snacks {
    // Added `pub` and used the expected alias after `as`.
    pub use self::fruits::PEAR as FRUIT;
    pub use self::veggies::CUCUMBER as VEGGIE;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::FRUIT,
        delicious_snacks::VEGGIE,
    );
}
