trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: 为字符串动态数组实现 `AppendBar` trait。
// `append_bar` 应将字符串 "Bar" 追加到动态数组中。

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
