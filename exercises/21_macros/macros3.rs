// TODO: 修复编译错误，
// 但不要将宏定义移出此模块。
mod macros {
    macro_rules! my_macro {
        () => {
            println!("看看我写的宏！");
        };
    }
}

fn main() {
    my_macro!();
}
