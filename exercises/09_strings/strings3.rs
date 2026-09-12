fn trim_me(input: &str) -> &str {
    // TODO: 移除字符串两端的空白字符。
    input.trim()
}

fn compose_me(input: &str) -> String {
    // TODO: 在字符串后追加 " world!"！有多种方式可以实现。
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    // TODO: 将字符串中的 "cars" 替换为 "balloons"。
    input.replace("cars", "balloons")
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
