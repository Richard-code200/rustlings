fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: 让动态数组 `vec0` 和 `vec1` 可以同时访问，
    // 以修复测试中的编译错误。
    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        assert_eq!(vec0, [22, 44, 66]);
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
