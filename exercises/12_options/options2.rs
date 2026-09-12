fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: 将其改为匹配 `Some` 值的 if-let 语句。
        word = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: 将其改为 while-let 语句。注意，`Vec::pop()`
        // 会额外包裹一层 `Option`。可以在 if-let 和 while-let
        // 语句中使用嵌套模式匹配。
        integer = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
