use std::time::{Duration, Instant};
use tokio::task;

pub async fn benchmark_multi_thread_cpu() -> Duration {
    let start = Instant::now();

    let num_threads = num_cpus::get();
    let iterations_per_thread = 10_000_000_000 / num_threads;

    let handles: Vec<_> = (0..num_threads)
        .map(|_| {
            task::spawn_blocking(move || {
                let mut x = 1u64;
                for _ in 0..iterations_per_thread {
                    x = x.wrapping_mul(3).wrapping_add(1);
                    // 複雑な演算を追加
                    let y = (x.wrapping_add(123456789)) ^ (x.wrapping_mul(987654321));
                    x = y.wrapping_add(1);
                }
                x
            })
        })
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }

    start.elapsed()
}
