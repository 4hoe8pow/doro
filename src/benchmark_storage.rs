use rand::Rng;
use std::fs::{remove_file, File};
use std::io::{Read, Write, Seek};
use std::time::{Duration, Instant};

pub fn benchmark_storage() -> Duration {
    let file_name = "benchmark_test_file.bin";
    let size = 1_000_000_000; // 1GBのファイルサイズ
    let mut rng = rand::thread_rng();

    let start = Instant::now();

    // Write benchmark with random data
    {
        let mut file = File::create(file_name).unwrap();
        let mut data = vec![0u8; size];

        // ランダムデータを生成して書き込む
        rng.fill(&mut data[..]);
        file.write_all(&data).unwrap();
    }

    // Read benchmark with random access
    {
        let mut file = File::open(file_name).unwrap();
        let mut buffer = vec![0u8; size];

        // ランダムアクセスで読み込む
        for _ in 0..100_000 {
            let pos = rng.gen_range(0..size);
            file.seek(std::io::SeekFrom::Start(pos as u64)).unwrap();
            file.read_exact(&mut buffer[0..1]).unwrap();
        }
    }

    // Cleanup
    remove_file(file_name).unwrap();

    start.elapsed()
}
