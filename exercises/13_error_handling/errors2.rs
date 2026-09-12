// 假设我们正在编写一个可以用代币购买物品的游戏。
// 每件物品需要 5 枚代币，每次购买还要收取 1 枚代币的手续费。
// 玩家会输入要购买的物品数量，
// `total_cost` 函数负责计算总费用。
// 由于数量由玩家输入，我们得到的是字符串。
// 玩家可能输入任何内容，不一定是数字！
//
// 目前，此函数完全没有处理错误情况。我们希望实现以下行为：
// 如果传给 `total_cost` 的字符串不是数字，
// 解析时会产生 `ParseIntError`。
// 此时应立即从函数返回该错误，
// 不再尝试进行乘法和加法运算。
//
// 至少有两种正确的实现方式，
// 但其中一种要简短得多！

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // TODO: 按照上面的说明处理错误情况。
    let qty = item_quantity.parse::<i32>();

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
