use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let mut handles = vec![];

    // 启动10个线程
    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));  // 每个线程睡眠 250 ms
            println!("thread {} is complete", i);       // 打印线程完成信息
            start.elapsed().as_millis()                 // 返回该线程的执行时间
        }));
    }

    let mut results: Vec<u128> = vec![];
    
    // 等待每个线程结束并获取结果
    for handle in handles {
        let result = handle.join().expect("Thread panicked!"); // 获取线程的执行时间
        results.push(result);   // 将执行时间放入results向量中
    }

    // 检查是否有 10 个结果（确保所有线程都完成）
    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    println!();
    
    // 打印每个线程的执行时间
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
