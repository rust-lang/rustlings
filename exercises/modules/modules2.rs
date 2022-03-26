// modules2.rs
// You can bring module paths into scopes and provide new names for them with the
// 'use' and 'as' keywords. Fix these 'use' statements to make the code compile.
// Make me compile! Execute `rustlings hint modules2` for hints :)


mod delicious_snacks {

    // TODO: Fix these use statements
    pub use self::fruits::PEAR as fruit; // so I'm telling this that PEAR is a fruit.
    pub use self::veggies::CUCUMBER as veggie; // and here I'm saying CUCUMBER is a veggie.

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
        delicious_snacks::fruit, // So we're looking for a delicious snack, but it has to be fruit.
        delicious_snacks::veggie // and here it needs to be veggie
    );
}
