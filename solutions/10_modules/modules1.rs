mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // Added `pub` before `fn` to make the function accessible outside the module.
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
