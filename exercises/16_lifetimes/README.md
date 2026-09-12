# 生命周期

生命周期告诉编译器如何检查引用的有效时间
是否足以满足特定场景的要求。例如，生命周期可以表达：
“确保参数 a 的有效时间至少与参数 b 一样长，
从而保证返回值有效”。

生命周期约束用于借用，也就是引用。
通过复制或移动传入的值，在所属作用域内拥有所有权，
不能在其失效后继续被外部引用。借助生命周期，
编译器可以检查函数等调用处的代码，确保传入的引用有效。
因此，生命周期会对调用方施加约束。

如果你想进一步学习生命周期标注，可以尝试
[lifetimekata](https://tfpk.github.io/lifetimekata/) 项目。
它的练习风格与 Rustlings 类似，
但专门用于学习编写生命周期标注。

## 延伸阅读

- [生命周期（《通过例子学 Rust》）](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)
- [使用生命周期验证引用](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
