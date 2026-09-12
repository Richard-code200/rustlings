// 本练习给定一个名为 `numbers` 的 `Vec<u32>`，
// 包含 0 到 99 的整数。我们希望在 8 个不同的线程中
// 同时使用这些数。每个线程从各自的偏移量开始，
// 每隔 8 个元素取一个值并求和。
//
// 第一个线程（偏移量 0）对 0、8、16、…… 求和。
// 第二个线程（偏移量 1）对 1、9、17、…… 求和。
// 第三个线程（偏移量 2）对 2、10、18、…… 求和。
// …
// 第八个线程（偏移量 7）对 7、15、23、…… 求和。
//
// 每个线程都应拥有一个指向该动态数组的引用计数指针。
// 但 `Rc` 不是线程安全的，因此需要使用 `Arc`。
//
// 暂时不必关注线程的创建和等待结束方式。
// 我们会在后面的线程练习中学习这些内容。

// 请勿修改下面的代码。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: 使用 `Arc` 定义 `shared_numbers`。
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: 使用 `shared_numbers` 定义 `child_numbers`。
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("偏移量 {offset} 对应的总和为 {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
