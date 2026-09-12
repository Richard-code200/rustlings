// 本测验涵盖以下章节：
// - 字符串
// - 动态数组
// - 移动语义
// - 模块
// - 枚举
//
// 我们来用函数构建一个小型处理器。输入是一个包含字符串和命令的列表。
// 命令决定对字符串执行何种操作，
// 可以是以下操作之一：
// - 将字符串转换为大写
// - 移除字符串两端的空白字符
// - 在字符串后追加指定次数的 "bar"
//
// 具体形式如下：
// - 输入是一个动态数组，其中每个元素都是二元组，
//   元组的第一个元素是字符串，第二个元素是命令。
// - 输出是一个字符串动态数组。

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(s, cmd)| match cmd {
                Command::Uppercase => s.to_uppercase(),
                Command::Trim => s.trim().to_string(),
                Command::Append(n) => s + &"bar".repeat(n),
            })
            .collect()
    }
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    // TODO: 需要导入什么，才能在当前作用域中使用 `transformer`？
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
