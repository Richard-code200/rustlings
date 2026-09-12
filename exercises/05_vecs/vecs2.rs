fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: 将切片 `input` 中的每个元素乘以 2，
        // 再追加到动态数组 `output` 中。
        output.push(element * 2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 这是映射后收集为动态数组的示例。
    // 将切片 `input` 中的每个元素映射为原值加 1。
    // 如果输入是 `[1, 2, 3]`，输出就是 `[2, 3, 4]`。
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: 这里同样需要将切片 `input` 中的每个元素乘以 2，
    // 但要使用迭代器映射，而不是手动将元素追加到
    // 一个空的动态数组中。
    // 请参考上面的 `vec_map_example` 函数。
    input.iter().map(|element| element * 2).collect()
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
