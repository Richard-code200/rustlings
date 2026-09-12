// 本练习探索 `Cow`（写时克隆）智能指针。
// 它可以包装借用的数据并提供不可变访问，
// 仅在需要修改数据或获取所有权时才进行克隆。
// 该类型通过 `Borrow` trait 支持通用的借用数据。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // 如果尚未拥有数据，则将其克隆为动态数组。
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // 你可以在这里自由尝试。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // 由于需要修改 `input`，会发生克隆。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // 由于不需要修改 `input`，不会发生克隆。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_no_mutation() {
        // 也可以直接传入不带 `&` 的 `vec`，让 `Cow` 直接拥有它。
        // 这里不需要修改数据（所有数都已是自身的绝对值），
        // 因此也不会克隆。但结果仍然拥有数据，
        // 因为它从一开始就不是借用的，也未被修改。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }

    #[test]
    fn owned_mutation() {
        // 即使需要修改数据（并非所有数都已是自身的绝对值），
        // 结果也同样拥有数据。此时 `abs_all` 中的 `to_mut()`
        // 返回的引用仍然指向原来的那份数据。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        // TODO: 将 `todo!()` 替换为 `Cow::Owned(_)` 或 `Cow::Borrowed(_)`。
        assert!(matches!(input, todo!()));
    }
}
