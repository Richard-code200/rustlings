// 此函数返回冰箱中剩余的冰淇淋数量。
// 22:00 之前（24 小时制）还剩 5 勺。到了 22:00，
// 有人把它全部吃光，因此剩余数量为 0。
// 如果 `hour_of_day` 大于 23，则返回 `None`。
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    // TODO: 完成函数体。
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: 修复此测试。如何取出
        // Option 中包含的值？
        let ice_creams = maybe_ice_cream(12);

        assert_eq!(ice_creams, 5); // 请勿修改这一行。
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
