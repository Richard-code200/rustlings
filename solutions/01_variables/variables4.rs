fn main() {
    // 在 Rust 中，变量默认是不可变的。
    // 在 `let` 后添加 `mut` 关键字，可将声明的变量设为可变。
    let mut x = 3;
    println!("数值 {x}");

    x = 5;
    println!("数值 {x}");
}
