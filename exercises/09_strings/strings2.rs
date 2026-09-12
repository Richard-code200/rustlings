// TODO: 修复 `main` 函数中的编译错误，但不要修改此函数。
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green"); // 请勿修改这一行。

    if is_a_color_word(&word) {
        println!("这是我认识的颜色词！");
    } else {
        println!("这不是我认识的颜色词。");
    }
}
