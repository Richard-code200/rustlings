// Rust 编译器需要知道如何检查传入的引用是否有效，
// 才能在引用可能于使用前失效时提醒程序员。
// 请记住：引用只是借用，并不拥有数据。
// 如果数据的所有者离开了作用域，会发生什么？

// TODO: 更新函数签名，修复编译错误。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // 你可以在这里自由尝试。
    let result = longest("12345", "abcd");
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
