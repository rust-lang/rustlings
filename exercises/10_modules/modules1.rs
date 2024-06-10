// modules1.rs
//
// 執行 `rustlings hint modules1` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

mod sausage_factory {
    // 不要讓這個模組外的任何人看到這個！
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
