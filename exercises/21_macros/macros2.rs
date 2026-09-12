fn main() {
    my_macro!();
}

// TODO: 移动此宏的完整定义，修复编译错误。
macro_rules! my_macro {
    () => {
        println!("看看我写的宏！");
    };
}
