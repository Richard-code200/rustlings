fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(2 * element);
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
    // 我们之后会深入学习迭代器，目前只需
    // 完成这些操作！
    // 进阶说明：这种方法效率更高，因为它会自动预分配足够的容量。
    // 在 `vec_loop` 中，也可以用 `Vec::with_capacity(input.len())`
    // 代替 `Vec::new()`，手动实现这一点。
    input.iter().map(|element| 2 * element).collect()
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
