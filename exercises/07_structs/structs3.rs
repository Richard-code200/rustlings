// 结构体不仅可以包含数据，还可以拥有相关逻辑。在本练习中，
// 我们定义了 `Package` 结构体，并希望测试它的部分逻辑。

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // 这并不是 Rust 中推荐的错误处理方式，
            // 我们将在后面的练习中学习错误处理。
            panic!("无法寄送重量低于 10 克的包裹");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: 为函数签名添加正确的返回类型。
    fn is_international(&self) -> bool {
        // TODO: 阅读使用此方法的测试，确定什么情况下
        // 包裹应被视为国际包裹。
        self.sender_country != self.recipient_country
    }

    // TODO: 为函数签名添加正确的返回类型。
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // TODO: 计算包裹的运费。
        self.weight_in_grams * cents_per_gram
    }
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
