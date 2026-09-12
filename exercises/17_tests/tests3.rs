struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // 请勿修改此函数。
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // 这里返回 `Result` 会更好，但我们想学习
            // 如何测试可能触发 panic 的函数。
            panic!("矩形的宽和高必须为正数");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        // TODO: 此测试应检查矩形的尺寸
        // 是否与传给构造函数的尺寸一致。
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // 检查宽度
        assert_eq!(todo!(), 20); // 检查高度
    }

    // TODO: 此测试应检查尝试创建宽度为负数的矩形时，
    // 程序是否触发 panic。
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: 此测试应检查尝试创建高度为负数的矩形时，
    // 程序是否触发 panic。
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
