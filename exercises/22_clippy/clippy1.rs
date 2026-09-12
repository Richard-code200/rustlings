// Clippy 工具提供了一组静态检查规则，用于分析代码，
// 帮助你发现常见错误并改进 Rust 代码。
//
// 在这些练习中，只要出现 Clippy 警告，代码就无法通过编译。
// 请查看输出中的 Clippy 建议来完成练习。

fn main() {
    // TODO: 修复这一行触发的 Clippy 警告。
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("半径为 {radius:.2} 的圆，其面积为 {area:.5}");
}
