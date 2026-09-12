// TODO: 在冒号 `:` 后补上参数 `num` 缺少的类型。
fn call_me(num: i32) {
    for i in 0..num {
        println!("叮铃！第 {} 次呼叫", i + 1);
    }
}

fn main() {
    call_me(3);
}
