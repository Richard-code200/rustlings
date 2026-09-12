# 类型转换

Rust 提供了多种方式，可以将一种类型的值转换为另一种类型。

最简单的类型转换形式是类型转换表达式，使用二元运算符 `as` 表示。例如，`println!("{}", 1 + 1.0);` 无法编译，因为 `1` 是整数，而 `1.0` 是浮点数。但 `println!("{}", 1 as f32 + 1.0)` 可以编译。练习 [`using_as`](using_as.rs) 将介绍这种用法。

Rust 还提供了一些 trait，实现后便能方便地进行类型转换。它们位于 [`convert`](https://doc.rust-lang.org/std/convert/index.html) 模块中。
这些 trait 包括：

- `From` 和 `Into`：参见 [`from_into`](from_into.rs)
- `TryFrom` 和 `TryInto`：参见 [`try_from_into`](try_from_into.rs)
- `AsRef` 和 `AsMut`：参见 [`as_ref_mut`](as_ref_mut.rs)

此外，`std::str` 模块提供了 [`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) trait，可通过字符串的 `parse` 方法将字符串转换为目标类型。如果为 `Person` 类型正确实现了该 trait，那么 `let p: Person = "Mark,20".parse().unwrap()` 就能够编译并运行，而不会触发 panic。

以上就是***标准库中***将数据转换为目标类型的主要方式。

## 延伸阅读

书中没有直接讲解这些内容，但标准库提供了详尽的文档。

- [类型转换](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)
