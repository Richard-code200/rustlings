fn main() {
    let number = "T-H-R-E-E";
    println!("拼写一个数字：{number}");

    // 使用变量遮蔽
    // https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    let number = 3;
    println!("该数加 2 的结果为：{}", number + 2);
}
