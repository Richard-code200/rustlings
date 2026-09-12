fn main() {
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("这是字母！");
    } else if my_first_initial.is_numeric() {
        println!("这是数字！");
    } else {
        println!("既不是字母，也不是数字！");
    }

    // 使用表情符号的示例。
    let your_character = '🦀';

    if your_character.is_alphabetic() {
        println!("这是字母！");
    } else if your_character.is_numeric() {
        println!("这是数字！");
    } else {
        println!("既不是字母，也不是数字！");
    }
}
