# 可选值（Option）

Option 类型表示一个可选值：每个 Option 要么是包含一个值的 Some，要么是不包含值的 None。
Option 在 Rust 代码中非常常见，具有多种用途：

- 表示初始值
- 作为未对整个输入范围定义结果的函数（偏函数）的返回值
- 通过返回 None 表示简单的错误
- 表示可选的结构体字段
- 表示可以被借用或“取走”的结构体字段
- 表示可选的函数参数
- 表示可空指针
- 在难以直接移出值的情况下，通过替换将值取出

## 延伸阅读

- [Option 枚举的定义形式](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)
- [Option 模块文档](https://doc.rust-lang.org/std/option/)
- [Option 枚举文档](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
