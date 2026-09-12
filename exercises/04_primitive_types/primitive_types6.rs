fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: 使用元组索引访问 `numbers` 的第二个元素，
        // 并将其赋给名为 `second` 的变量。
        // let second = ???;
        let second = numbers.1;

        assert_eq!(second, 2, "这不是元组中的第二个数！");
    }
}
