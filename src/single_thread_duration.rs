use std::time::{Duration, Instant};

pub fn benchmark_single_thread_cpu() -> Duration {
    let start = Instant::now();
    let mut x = 1u64;
    for _ in 0..1_000_000_000 {
        x = x.wrapping_mul(3).wrapping_add(1);
    }
    start.elapsed()
}
