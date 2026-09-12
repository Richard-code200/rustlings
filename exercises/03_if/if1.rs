fn bigger(a: i32, b: i32) -> i32 {
    // TODO: 完成此函数，返回两个数中较大的一个！
    // 如果两个数相等，返回任意一个即可。
    // 不允许使用：
    // - 其他函数调用
    // - 额外的变量
    if a > b { a } else { b }
}

fn main() {
    // 你可以在这里自由尝试。
}

// 暂时不用关心下面的内容 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
