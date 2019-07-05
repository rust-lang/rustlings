// modules1.rs
// Make me compile! Scroll down for hints :)

mod sausage_factory {
    pub fn make_sausage() {
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}




























// Everything is private in Rust by default-- but there's a keyword we can use
// to make something public! The compiler error should point to the thing that
// needs to be public.
