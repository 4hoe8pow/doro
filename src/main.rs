//mod benchmark_gpu;
mod benchmark_memory;
mod benchmark_multi_thread_cpu;
mod benchmark_storage;
mod single_thread_duration;

use crate::benchmark_memory::*;
use crate::benchmark_multi_thread_cpu::*;
use crate::benchmark_storage::*;
use crate::single_thread_duration::*;

#[tokio::main]
async fn main() {
    println!("Starting hardware benchmarks...");

    // Retrieve CPU core counts
    let physical_cores = num_cpus::get_physical();
    let logical_cores = num_cpus::get();
    println!(
        "CPU: {} physical cores, {} logical cores",
        physical_cores, logical_cores
    );

    // CPU Benchmarks
    let single_thread_duration = benchmark_single_thread_cpu();
    println!("Single-thread CPU Benchmark: {:?}", single_thread_duration);

    let multi_thread_duration = benchmark_multi_thread_cpu().await;
    println!("Multi-thread CPU Benchmark: {:?}", multi_thread_duration);

    // Memory Benchmark
    let memory_duration = benchmark_memory();
    println!("Memory Benchmark: {:?}", memory_duration);

    // Storage Benchmark
    let storage_duration = benchmark_storage();
    println!("Storage Benchmark: {:?}", storage_duration);

    // GPU Benchmark
    //let gpu_duration = benchmark_gpu().await;
    //println!("GPU Benchmark: {:?}", gpu_duration);

    println!("Benchmarks completed.");
}
