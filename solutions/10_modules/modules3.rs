use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("距 1970-01-01 00:00:00 UTC 已过去 {} 秒！", n.as_secs()),
        Err(_) => panic!("系统时间早于 UNIX 纪元起点！"),
    }
}
