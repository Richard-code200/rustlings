fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    // TODO: 仅通过调整测试中各行的顺序来修复编译错误。
    // 不要添加、修改或删除任何一行。
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
