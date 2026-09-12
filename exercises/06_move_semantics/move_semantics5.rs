#![allow(clippy::ptr_arg)]

// TODO: 仅通过添加或删除引用符号（字符 `&`）来修复编译错误，
// 不要做其他修改。

// 不应获取所有权
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 应获取所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
