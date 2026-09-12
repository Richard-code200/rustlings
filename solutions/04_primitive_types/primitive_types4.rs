fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4  <- 索引
        //          -------
        //             |
        //             +--- 切片

        // 注意：不包含上界索引 4。
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // 使用 `..=` 语法（带 `=` 号）可以包含上界索引。
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
