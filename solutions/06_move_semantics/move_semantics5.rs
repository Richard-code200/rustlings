#![allow(clippy::ptr_arg)]

// 借用数据，而不是获取所有权。
// 这里推荐使用 `&str` 而不是 `&String`。不过我们还没有学习字符串，
// 因此目前这样写就足够了。
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 获取所有权，而不是借用。
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}
