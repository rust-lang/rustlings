// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

mod delicious_snacks {
<<<<<<< HEAD
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
=======
    // TODO: Fix these use statements
    use self::fruits::PEAR as ???
    use self::veggies::CUCUMBER as ???
>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e

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
