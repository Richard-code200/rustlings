fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // 数组

    // TODO: 创建一个名为 `v` 的动态数组，包含与数组 `a` 完全相同的元素。
    // 请使用创建动态数组的宏。
    // let v = ???;
    let v = vec![10, 20, 30, 40];

    (a, v)
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
