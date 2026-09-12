// 本练习与前面的 `from_into` 类似，但这一次需要实现 `FromStr`，
// 并在失败时返回错误，而不是回退到默认值。
// 此外，实现 `FromStr` 后，就可以调用字符串的 `parse` 方法，
// 生成实现该 trait 的类型的对象。
// 更多信息请参阅文档：
// https://doc.rust-lang.org/std/str/trait.FromStr.html

use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

// 我们将在 `FromStr` 实现中使用此错误类型。
#[derive(Debug, PartialEq)]
enum ParsePersonError {
    // 字段数量不正确
    BadLen,
    // 姓名字段为空
    NoName,
    // 包装 parse::<u8>() 返回的错误
    ParseInt(ParseIntError),
}

// TODO: 完成此 `FromStr` 实现，使其能够从 "Mark,20" 形式的字符串
// 解析出一个 `Person`。
// 注意：需要使用类似 `"4".parse::<u8>()` 的方式，
// 将年龄部分解析为 `u8`。
//
// 步骤：
// 1. 按逗号分割给定的字符串。
// 2. 如果分割得到的元素数量不等于 2，
//    则返回错误 `ParsePersonError::BadLen`。
// 3. 使用分割后的第一个元素作为姓名。
// 4. 如果姓名为空，则返回错误 `ParsePersonError::NoName`。
// 5. 将分割后的第二个元素解析为 `u8`，作为年龄。
// 6. 如果年龄解析失败，则返回错误 `ParsePersonError::ParseInt`。
impl FromStr for Person {
    type Err = ParsePersonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {}
}

fn main() {
    let p = "Mark,20".parse::<Person>();
    println!("{p:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ParsePersonError::*;

    #[test]
    fn empty_input() {
        assert_eq!("".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }

    #[test]
    fn missing_age() {
        assert!(matches!("John,".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn invalid_age() {
        assert!(matches!("John,twenty".parse::<Person>(), Err(ParseInt(_))));
    }

    #[test]
    fn missing_comma_and_age() {
        assert_eq!("John".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn missing_name() {
        assert_eq!(",1".parse::<Person>(), Err(NoName));
    }

    #[test]
    fn missing_name_and_age() {
        assert!(matches!(",".parse::<Person>(), Err(NoName | ParseInt(_))));
    }

    #[test]
    fn missing_name_and_invalid_age() {
        assert!(matches!(
            ",one".parse::<Person>(),
            Err(NoName | ParseInt(_)),
        ));
    }

    #[test]
    fn trailing_comma() {
        assert_eq!("John,32,".parse::<Person>(), Err(BadLen));
    }

    #[test]
    fn trailing_comma_and_some_string() {
        assert_eq!("John,32,man".parse::<Person>(), Err(BadLen));
    }
}
