fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        // 在这里使用 `y`。
        y.push(42);
        // 可变引用 `y` 此后不再使用，
        // 因此可以创建新的引用。
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
