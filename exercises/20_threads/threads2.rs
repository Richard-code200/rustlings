// 延续上一道练习，我们希望所有线程都完成各自的任务。
// 不过这一次，创建的线程需要负责更新一个共享值：
// `JobStatus.jobs_done`

use std::{sync::Arc, thread, time::Duration};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: 如果需要“可变”的共享状态，仅有 `Arc` 还不够。
    let status = Arc::new(JobStatus { jobs_done: 0 });

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: 更新共享值之前，必须先执行一个操作。
            status_shared.jobs_done += 1;
        });
        handles.push(handle);
    }

    // 等待所有任务完成。
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: 输出 `JobStatus.jobs_done` 的值。
    println!("已完成的任务数：{}", todo!());
}
