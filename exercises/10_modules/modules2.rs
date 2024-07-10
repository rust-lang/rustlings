// You can bring module paths into scopes and provide new names for them with
// the `use` and `as` keywords.

#[allow(dead_code)]
mod delicious_snacks {
    // TODO: Add the following two `use` statements after fixing them.
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

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
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
