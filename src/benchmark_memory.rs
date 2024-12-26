use rand::Rng;
use std::time::{Duration, Instant};

pub fn benchmark_memory() -> Duration {
    let start = Instant::now();

    let size = 1_000_000_000; // 1GBのメモリ使用
    let mut data = vec![0u8; size];

    // ランダムアクセスを行うことで、キャッシュ効率をテスト
    let mut rng = rand::thread_rng();
    for _ in 0..100_000 {
        let idx = rng.gen_range(0..size);
        data[idx] = rng.gen_range(0..256) as u8;
    }

    // メモリの読み書き（順次アクセス）も行う
    let mut sum: u64 = 0;
    for byte in data.iter() {
        sum = sum.wrapping_add(*byte as u64);
    }

    start.elapsed()
}
