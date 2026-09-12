// TODO: 添加一两个字符，修复编译错误。
#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("看看我写的宏！");
    }
    ($val:expr) => {
        println!("再看看这个宏：{}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
