mod delicious_snacks {
    // 添加 `pub`，并在 `as` 后使用所需的别名。
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "最喜欢的零食：{} 和 {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
