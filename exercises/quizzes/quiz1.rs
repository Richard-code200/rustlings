// 本测验涵盖以下章节：
// - 变量
// - 函数
// - if 条件表达式
//
// Mary 正在买苹果。苹果的价格按以下规则计算：
// - 每个苹果 2 个 Rust 币。
// - 如果 Mary 购买超过 40 个苹果，那么整笔订单中
// 每个苹果的价格都降为 1 个 Rust 币！

// TODO: 编写一个函数，根据购买数量
// 计算苹果订单的总价。
// fn calculate_price_of_apples(???) -> ??? { ??? }
fn calculate_price_of_apples(price: i32) -> i32 {
    if price <= 40 { price * 2 } else { price }
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
