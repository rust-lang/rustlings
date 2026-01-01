// modules1.rs
//
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a
// hint.

mod sausage_factory {
<<<<<<< HEAD
    pub fn make_sausage() {
=======
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
>>>>>>> 11d8aea96f2c744d970ed1ffb38785cf5b511e5e
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
