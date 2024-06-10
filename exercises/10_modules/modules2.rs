// modules2.rs
//
// 您可以將模組路徑引入作用域並使用 'use' 和 'as' 關鍵字為它們提供新名稱。修復這些 'use' 語句以使代碼編譯。
//
// 執行 `rustlings hint modules2` 或使用 `hint` watch 子命令來獲取提示。

// I AM NOT DONE

mod delicious_snacks {
    // TODO: 修復這些 use 語句
    use self::fruits::PEAR as fruit;
    use self::veggies::CUCUMBER as veggie;

    pub mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    pub mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "最喜歡的蔬果: {} 和 {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
