// modules2.rs
//
// You can bring module paths into scopes and provide new names for them with
// the 'use' and 'as' keywords. Fix these 'use' statements to make the code
// compile.
//
// Execute `rustlings hint modules2` or use the `hint` watch subcommand for a
// hint.

mod delicious_snacks {
    // TODO: Fix these use statements
    pub(crate) use self::fruits::PEAR as fruit;
    pub(crate) use self::veggies::CUCUMBER as veggie;

    mod fruits {
        // I can't remove the pub otherwise these constants won't be visible outside
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    // This won't work --- fruits is a private mod
    // println!("let's try to use it this way {}",
        // delicious_snacks::fruits::PEAR
    // );
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
