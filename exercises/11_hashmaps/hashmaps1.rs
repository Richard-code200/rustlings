// 请使用哈希映射定义一个水果篮。键表示水果名称，
// 值表示篮子中该种水果的数量。
// 你需要放入至少 3 种不同的水果
// （例如苹果、香蕉、芒果），
// 且水果总数至少为 5。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: 声明哈希映射。
    // let mut basket =

    let mut basket = HashMap::new();
    // 已经为你准备了两根香蕉 :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("e.g"), 0);
    basket.insert(String::from("apple"), 1);
    basket.insert(String::from("mango"), 3);

    // TODO: 往篮子里放入更多水果。

    basket
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
