#![allow(clippy::needless_late_init)]

fn main() {
    // Rust 不允许读取尚未初始化的变量！
    // 因此，我们需要先为其赋值。
    let x: i32 = 42;

    println!("数值 {x}");

    // 可以先声明变量，稍后再初始化。
    // 但在初始化之前不能使用它。
    let y: i32;
    y = 42;
    println!("数值 {y}");
}
