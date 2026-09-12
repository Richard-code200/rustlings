fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    // 这里两种答案都可以。
    // `.into()` 将一种类型转换为上下文期望的类型。
    // 如果上下文期望 `String`，它就会将 `&str` 转换为 `String`。
    string("nice weather".into());
    // 如果上下文期望 `&str`，则不需要转换，`&str` 仍保持为 `&str`。
    // 如果删除 `#[allow(…)]` 这一行，Clippy 会提示移除下面的 `.into()`，因为这是一次无用的转换。
    #[allow(clippy::useless_conversion)]
    string_slice("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // 注意：这里按字节索引，而不是按字符索引。
    // 按字符索引可以使用 `s.chars().nth(INDEX)`。
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
