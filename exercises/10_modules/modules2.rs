// 使用 `use` 和 `as` 关键字，可以将模块路径引入作用域，
// 并为其指定新的名称。

mod delicious_snacks {
    // TODO: 修复以下两条 `use` 语句，然后将其添加到代码中。
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;
}

fn main() {
    println!(
        "最喜欢的零食：{} 和 {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
