fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // 元组索引语法。
        let second = numbers.1;

        assert_eq!(second, 2, "这不是元组中的第二个数！");
    }
}
