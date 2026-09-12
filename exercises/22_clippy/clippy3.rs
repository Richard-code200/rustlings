// 这里还有一些简单的 Clippy 修复练习，让你体会它的实用性 📎
// TODO: 修复所有 Clippy 警告。

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // 假设你不知道 `my_option` 的值。
    // 如果它是 `Some`，我们希望输出其中的值。
    if my_option.is_none() {
        println!("{}", my_option.unwrap());
    }

    let my_arr = &[
        -1, -2, -3
        -4, -5, -6
    ];
    println!("这就是我的数组：{my_arr:?}");

    let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("看，这个 Vec 是空的：{my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // 交换这两个值！
    value_a = value_b;
    value_b = value_a;
    println!("a 的值：{value_a}；b 的值：{value_b}");
}
