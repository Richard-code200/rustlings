// 这家商店正在促销：如果价格是偶数，就减免 10 个 Rust 币；
// 如果价格是奇数，就减免 3 个 Rust 币。
// 暂时不必关心函数体，
// 我们目前只关注函数签名。

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: 修复函数签名。
fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("优惠后的价格为 {}", sale_price(original_price));
}
