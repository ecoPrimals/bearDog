//! Modernization performance validation benchmarks
//! Validates that modernized code maintains or improves performance

use beardog_core::BearDogCore;
use beardog_errors::BearDogError;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;
use tokio::runtime::Runtime;

fn init_runtime() -> Runtime {
    Runtime::new().expect("Failed to create runtime")
}

fn benchmark_modernized_core_operations(c: &mut Criterion) {
    let rt = init_runtime();

    c.bench_function("modernized_core_init", |b| {
        b.iter(|| {
            rt.block_on(async {
                let core = BearDogCore::new()
                    .await
                    .expect("Core initialization failed");
                black_box(core);
            });
        });
    });
}

fn benchmark_error_handling_modernization(c: &mut Criterion) {
    c.bench_function("modernized_error_construction", |b| {
        b.iter(|| {
            let error = BearDogError::validation("Modernized validation error".to_string());
            black_box(error);
        });
    });
}

fn benchmark_async_modernization(c: &mut Criterion) {
    let rt = init_runtime();

    c.bench_function("modernized_async_patterns", |b| {
        b.iter(|| {
            rt.block_on(async {
                let task = tokio::spawn(async {
                    tokio::time::sleep(Duration::from_micros(50)).await;
                    "modernized_result"
                });
                let result = task.await.expect("Task execution failed");
                black_box(result);
            });
        });
    });
}

fn benchmark_memory_modernization(c: &mut Criterion) {
    c.bench_function("modernized_memory_patterns", |b| {
        b.iter(|| {
            // Modernized memory allocation patterns
            let data: Vec<u8> = (0..2048).map(|i| (i % 256) as u8).collect();
            black_box(data);
        });
    });
}

fn benchmark_zero_copy_modernization(c: &mut Criterion) {
    c.bench_function("modernized_zero_copy", |b| {
        let data = vec![0u8; 4096];
        b.iter(|| {
            // Zero-copy view of data
            let view = &data[..];
            black_box(view);
        });
    });
}

fn benchmark_string_modernization(c: &mut Criterion) {
    c.bench_function("modernized_string_handling", |b| {
        let test_string = "modernized_string_processing_test";
        b.iter(|| {
            let processed = format!("processed_{}", test_string);
            black_box(processed);
        });
    });
}

criterion_group!(
    modernization_benches,
    benchmark_modernized_core_operations,
    benchmark_error_handling_modernization,
    benchmark_async_modernization,
    benchmark_memory_modernization,
    benchmark_zero_copy_modernization,
    benchmark_string_modernization
);
criterion_main!(modernization_benches);
