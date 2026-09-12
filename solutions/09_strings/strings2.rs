fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green");

    if is_a_color_word(&word) {
        //             ^ 添加后得到 `&String`，编译器会自动
        //               将其强制转换为 `&str`。
        println!("这是我认识的颜色词！");
    } else {
        println!("这不是我认识的颜色词。");
    }
}
