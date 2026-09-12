// 字符类型（`char`）

fn main() {
    // 注意这里使用的是“单引号”，
    // 它与之前常见的双引号不同。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("这是字母！");
    } else if my_first_initial.is_numeric() {
        println!("这是数字！");
    } else {
        println!("既不是字母，也不是数字！");
    }

    // TODO: 仿照上面的示例，在下方声明一个名为 `your_character` 的变量，
    // 并将其设为你喜欢的字符。
    // 可以尝试字母、数字（用单引号包裹）、特殊字符、
    // 其他语言中的字符，或者表情符号 😉
    // let your_character = '';

    let your_character = ' ';
    if your_character.is_alphabetic() {
        println!("这是字母！");
    } else if your_character.is_numeric() {
        println!("这是数字！");
    } else {
        println!("既不是字母，也不是数字！");
    }
}
