// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
