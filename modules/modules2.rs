// Make me compile! Scroll down for hints :)

mod us_presidential_frontrunners {
    use self::democrats::HILLARY_CLINTON as democrat;
    use self::republicans::DONALD_TRUMP as republican;

    mod democrats {
        pub const HILLARY_CLINTON: &'static str = "Hillary Clinton";
        pub const BERNIE_SANDERS: &'static str = "Bernie Sanders";
    }

    mod republicans {
        pub const DONALD_TRUMP: &'static str = "Donald Trump";
        pub const JEB_BUSH: &'static str = "Jeb Bush";
    }
}

fn main() {
    println!("candidates: {} and {}",
             us_presidential_frontrunners::democrat,
             us_presidential_frontrunners::republican);
}

















// The us_presidential_frontrunners module is trying to present an external
// interface (the `democrat` and `republican` constants) that is different than
// its internal structure (the `democrats` and `republicans` modules and
// associated constants). It's almost there except for one keyword missing for
// each constant.
