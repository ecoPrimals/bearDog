//! Hyperoptimized benchmarks for BearDog
//! Tests maximum performance patterns and zero-copy implementations

use beardog_core::BearDogCore;
use beardog_errors::BearDogError;
use beardog_utils::optimization::OptimizationEngine;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use tokio::runtime::Runtime;

fn init_runtime() -> Runtime {
    Runtime::new().expect("Failed to create runtime")
}

fn benchmark_hyperoptimized_core(c: &mut Criterion) {
    let rt = init_runtime();

    c.bench_function("hyperoptimized_initialization", |b| {
        b.iter(|| {
            rt.block_on(async {
                let core = BearDogCore::new().await.expect("Core init failed");
                black_box(core);
            });
        });
    });
}

fn benchmark_zero_copy_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_copy");

    group.bench_function("buffer_management", |b| {
        let engine = OptimizationEngine::new();
        b.iter(|| {
            let buffer = engine.get_zero_copy_buffer(1024);
            black_box(buffer);
        });
    });

    group.finish();
}

fn benchmark_memory_efficiency(c: &mut Criterion) {
    c.bench_function("memory_pool_allocation", |b| {
        b.iter(|| {
            let data = vec![0u8; 4096];
            black_box(data);
        });
    });
}

fn benchmark_async_operations(c: &mut Criterion) {
    let rt = init_runtime();

    c.bench_function("async_task_performance", |b| {
        b.iter(|| {
            rt.block_on(async {
                let task = tokio::spawn(async {
                    // Simulate work
                    tokio::time::sleep(Duration::from_micros(100)).await;
                    42
                });
                let result = task.await.expect("Task failed");
                black_box(result);
            });
        });
    });
}

fn benchmark_error_handling_performance(c: &mut Criterion) {
    c.bench_function("error_construction_speed", |b| {
        b.iter(|| {
            let error = BearDogError::system("Performance test error".to_string());
            black_box(error);
        });
    });
}

criterion_group!(
    benches,
    benchmark_hyperoptimized_core,
    benchmark_zero_copy_operations,
    benchmark_memory_efficiency,
    benchmark_async_operations,
    benchmark_error_handling_performance
);
criterion_main!(benches);
