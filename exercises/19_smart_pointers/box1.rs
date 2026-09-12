// 编译时，Rust 需要知道每种类型占用多少空间。
// 递归类型会带来问题，因为一个值的内部
// 可能包含另一个相同类型的值。为解决这个问题，可以使用
// `Box`——一种将数据存储在堆上的智能指针，
// 它也能用来包装递归类型。
//
// 本练习要实现的递归类型是“cons 列表”，
// 这是函数式编程语言中常见的数据结构。cons 列表中的每一项
// 包含两个元素：当前项的值，以及下一项。
// 列表末尾使用名为 `Nil` 的值表示结束。

// TODO: 在枚举定义中使用 `Box`，使代码能够编译。
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, List),
    Nil,
}

// TODO: 创建一个空的 cons 列表。
fn create_empty_list() -> List {
    todo!()
}

// TODO: 创建一个非空的 cons 列表。
fn create_non_empty_list() -> List {
    todo!()
}

fn main() {
    println!("这是一个空的 cons 列表：{:?}", create_empty_list());
    println!(
        "这是一个非空的 cons 列表：{:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
