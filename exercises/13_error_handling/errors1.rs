// TODO: 如果传入空字符串，此函数就不会生成用于打印在姓名牌上的文字。
// 相比直接返回 `None`，如果它能说明问题出在哪里，
// 就更好了。幸运的是，Rust 提供了一种与 `Option` 类似的类型，
// 可以用来表示错误情况。请修改函数签名和函数体，
// 使其返回 `Result<String, String>`，
// 而不是 `Option<String>`。
fn generate_nametag_text(name: String) -> Option<String> {
    if name.is_empty() {
        // 不允许使用空姓名
        None
    } else {
        Some(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".to_string()).as_deref(),
            Ok("Hi! My name is Beyoncé"),
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
