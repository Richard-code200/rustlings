// 本练习是 `errors4` 的变体，用到了一些后面才会学习的概念，
// 例如 `Box` 和 `From` trait。
// 目前不必详细理解这些概念，感兴趣的话也可以提前阅读。
// 现在可以把 `Box<dyn ???>` 理解为这样一种类型：
// “我需要任何具备 ??? 能力的值”。
//
// 简而言之，这种 `Box` 用法适用于以下情况：你希望拥有某个值，
// 但只关心它的类型是否实现了某个特定的 trait。
// 为此，可以将 `Box` 声明为 `Box<dyn Trait>`，
// 其中 `Trait` 是编译器要求该上下文中的值必须实现的 trait。
// 在本练习中，这些值就是
// 可能通过 `Result` 返回的各种错误。

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 为了让 `CreationError` 能够实现 `Error`，必须提供此实现。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "数值为负数",
            CreationError::Zero => "数值为零",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// TODO: 添加正确的返回类型 `Result<(), Box<dyn ???>>`。可以用什么
// 来描述这两种错误？它们是否实现了某个共同的 trait？
fn main() {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("输出={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
