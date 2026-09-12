// `From` trait 用于值之间的类型转换。实现 `From` 后，
// 会自动获得相应的 `Into` 实现。
// 更多信息请参阅文档：
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 我们实现 Default trait，以便在传入的字符串
// 无法转换为 `Person` 对象时，使用默认值作为回退。
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: 完成此 `From` 实现，使其能够从 "Mark,20" 形式的字符串
// 解析出一个 `Person`。
// 注意：需要使用类似 `"4".parse::<u8>()` 的方式，
// 将年龄部分解析为 `u8`。
//
// 步骤：
// 1. 按逗号分割给定的字符串。
// 2. 如果分割得到的元素数量不等于 2，
//    则返回 `Person` 的默认值。
// 3. 使用分割后的第一个元素作为姓名。
// 4. 如果姓名为空，则返回 `Person` 的默认值。
// 5. 将分割后的第二个元素解析为 `u8`，作为年龄。
// 6. 如果年龄解析失败，则返回 `Person` 的默认值。
impl From<&str> for Person {
    fn from(s: &str) -> Self {}
}

fn main() {
    // 使用 `from` 函数。
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // 由于 Person 实现了 `From`，因此也可以使用 `Into`。
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
