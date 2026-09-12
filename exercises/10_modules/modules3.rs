// 使用 `use` 关键字，可以将各处模块中的路径引入当前作用域，
// 标准库中的模块也不例外。

// TODO: 将 `std::time` 模块中的 `SystemTime` 和 `UNIX_EPOCH`
// 引入当前作用域。如果能用一行完成，写法就更漂亮了！
// use ???;

use std::time::{SystemTime, UNIX_EPOCH};
fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("距 1970-01-01 00:00:00 UTC 已过去 {} 秒！", n.as_secs()),
        Err(_) => panic!("系统时间早于 UNIX 纪元起点！"),
    }
}
