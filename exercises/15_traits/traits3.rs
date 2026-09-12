trait Licensed {
    // TODO: 为 `licensing_info` 添加默认实现，
    // 让下面这两个结构体等实现者能够共享默认行为，
    // 无需重复编写此函数。
    // 默认许可信息应为字符串 "Default license"。
    fn licensing_info(&self) -> String;
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // 请勿修改这一行。
impl Licensed for OtherSoftware {} // 请勿修改这一行。

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = "Default license";
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
