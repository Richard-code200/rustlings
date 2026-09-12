# 特征（trait）

trait 是一组方法的集合。

数据类型可以实现 trait，也就是为该数据类型定义 trait 所要求的方法。例如，`String` 类型实现了 `From<&str>` trait，因此可以编写 `String::from("hello")`。

从这个角度看，trait 与 Java 的接口、C++ 的抽象类有些相似。

Rust 中其他常见的 trait 包括：

- `Clone`（提供 `clone` 方法）
- `Display`（支持通过 `{}` 进行格式化显示）
- `Debug`（支持通过 `{:?}` 进行调试格式化显示）

由于 trait 描述了不同数据类型共享的行为，因此在编写泛型代码时非常有用。

## 延伸阅读

- [trait：定义共享行为](https://doc.rust-lang.org/book/ch10-02-traits.html)
