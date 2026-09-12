// 函数参数必须标注类型。
// 这里添加了类型标注 `u64`。
fn call_me(num: u64) {
    for i in 0..num {
        println!("叮铃！第 {} 次呼叫", i + 1);
    }
}

fn main() {
    call_me(3);
}
