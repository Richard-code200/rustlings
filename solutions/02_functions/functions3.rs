fn call_me(num: u8) {
    for i in 0..num {
        println!("叮铃！第 {} 次呼叫", i + 1);
    }
}

fn main() {
    // `call_me` 需要一个参数。
    call_me(5);
}
