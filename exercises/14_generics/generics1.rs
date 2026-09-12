// `Vec<T>` 使用类型参数 `T` 实现泛型。多数情况下，编译器能够推断 `T`，
// 例如向动态数组中追加一个具体类型的值之后。
// 但在本练习中，需要通过类型标注帮助编译器。

fn main() {
    // TODO: 为动态数组标注类型 `Vec<T>`，修复编译错误。
    // 为 `T` 选择一种整数类型，要求它既能从 `u8` 转换而来，
    // 也能从 `i8` 转换而来。
    let mut numbers = Vec::new();

    // 请勿修改下面的代码。
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
