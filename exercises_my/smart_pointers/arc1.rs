#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers); // 将 numbers 封装进 Arc 中
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers); // 每个线程获得 Arc 的一个克隆
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.iter()
                .filter(|&&n| n % 8 == offset) // 根据 offset 过滤数字
                .sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    
    for handle in joinhandles.into_iter() {
        handle.join().unwrap(); // 等待所有线程完成
    }
}
