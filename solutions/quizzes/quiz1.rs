// Mary 正在买苹果。苹果的价格按以下规则计算：
// - 每个苹果 2 个 Rust 币。
// - 如果 Mary 购买超过 40 个苹果，那么整笔订单中
// 每个苹果的价格都降为 1 个 Rust 币！

fn calculate_price_of_apples(n_apples: u64) -> u64 {
    if n_apples > 40 {
        n_apples
    } else {
        2 * n_apples
    }
}

fn main() {
    // 你可以在这里自由尝试。
}

// 请勿修改测试！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
