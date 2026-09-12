// 这个强大的包装类型能够存储一个正整数值。
// TODO: 使用泛型重写它，使其支持包装任意类型。
struct Wrapper {
    value: u32,
}

// TODO: 调整此结构体的实现，使被包装的值也使用泛型。
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
