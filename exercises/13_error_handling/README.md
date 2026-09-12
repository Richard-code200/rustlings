# 错误处理

大多数错误并没有严重到需要让整个程序停止运行。
有时，函数失败的原因很容易理解，也很容易采取相应措施。
例如，尝试打开文件时，如果操作因为文件不存在而失败，你可能希望创建该文件，而不是终止进程。

## 延伸阅读

- [错误处理](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [泛型](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Result](https://doc.rust-lang.org/rust-by-example/error/result.html)
- [使用 Box 包装错误](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)
