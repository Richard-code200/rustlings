// 对此函数的调用应替换为对 `string_slice` 或 `string` 的调用。
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 下面有一组值，其中一些是 `String`，另一些是 `&str`。
// 请判断每个值的类型，将 `placeholder(…)` 替换为
// `string_slice(…)` 或 `string(…)`。
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string_slice("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // 注意：这里按字节索引，而不是按字符索引。
    // 按字符索引可以使用 `s.chars().nth(INDEX)`。
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
