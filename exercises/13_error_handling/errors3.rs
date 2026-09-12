// 这个程序尝试使用上一道练习中
// 已完成的 `total_cost` 函数，但它无法正常运行！
// 为什么？应该如何修复？

use std::num::ParseIntError;

// 请勿修改此函数。
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: 修改 `main` 函数的签名和函数体，
// 修复编译错误。
fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // 请勿修改这一行。
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("你的代币不够买这么多！");
    } else {
        tokens -= cost;
        println!("你现在还有 {tokens} 枚代币。");
    }
}
