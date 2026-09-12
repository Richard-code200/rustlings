# 动态数组（Vec）

动态数组是 Rust 中最常用的数据结构之一。在某些编程语言中，
它们通常直接称为数组。不过，Rust 更接近底层：
普通局部数组通常存储在栈上，其长度固定，
不能增长或缩小，且必须在编译时确定；
Vec 的元素存储在堆上，不受这些长度限制。

虽然书中在较后的章节才介绍动态数组，
但我们认为它足够实用，值得提前学习。
另一种实用的数据结构——哈希映射——将在后面介绍。

## 延伸阅读

- [使用动态数组存储一组值](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [`iter_mut`](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
