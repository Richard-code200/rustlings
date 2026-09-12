fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    //  ^^^ 新增部分

    vec.push(88);

    vec
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // `vec0` 已被移动到 `fill_vec` 中，因此无法再访问它。
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
