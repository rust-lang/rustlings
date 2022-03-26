// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)


mod sausage_factory { // adding pub here doesn't seem to matter . . . If I had to guess, moduless default to public.
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() { // adding pub here fixes the problem. I think the lesson is that functions are inherely private, and have to be explicitly made public?
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
