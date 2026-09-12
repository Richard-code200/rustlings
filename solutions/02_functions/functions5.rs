fn square(num: i32) -> i32 {
    // 移除下面这行末尾的分号 `;`，以隐式返回结果。
    num * num
}

fn main() {
    let answer = square(3);
    println!("3 的平方是 {answer}");
}
