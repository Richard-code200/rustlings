// 测试非常重要，它能帮助你确认代码
// 是否按预期运行。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    // TODO: 导入 `is_even`。可以使用通配符，
    // 导入外层模块中的所有内容。

    #[test]
    fn you_can_assert() {
        // TODO: 使用一些值测试 `is_even` 函数。
        assert!();
        assert!();
    }
}
