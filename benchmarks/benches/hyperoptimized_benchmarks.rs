use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use pprof::criterion::{Output, PProfProfiler};
use std::time::Duration;

use beardog_errors::BearDogError;
use beardog_types::constants::domains::system::timeouts::default_timeout_ms;
use beardog_utils::zero_copy::advanced_optimization::ZeroCopyOptimizer;

fn benchmark_error_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("error_creation");
    group.throughput(Throughput::Elements(1));

    group.bench_function("security_error", |b| {
        b.iter(|| black_box(BearDogError::security("benchmark security error")))
    });

    group.bench_function("system_error", |b| {
        b.iter(|| black_box(BearDogError::system("benchmark system error")))
    });

    group.bench_function("validation_error", |b| {
        b.iter(|| black_box(BearDogError::validation("benchmark validation error")))
    });

    group.finish();
}

fn benchmark_zero_copy_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_copy_optimization");
    group.throughput(Throughput::Elements(1));

    let optimizer = ZeroCopyOptimizer::new();
    let test_strings = vec![
        "common_string_1",
        "common_string_2",
        "common_string_3",
        "unique_string_12345",
        "another_unique_string_67890",
    ];

    group.bench_function("string_interning", |b| {
        b.iter(|| {
            for s in &test_strings {
                black_box(optimizer.intern_string(s));
            }
        })
    });

    let test_data = "test_data_for_zero_copy_view";
    group.bench_function("zero_copy_view", |b| {
        b.iter(|| black_box(optimizer.create_zero_copy_view(&test_data)))
    });

    group.bench_function("shared_data_creation", |b| {
        b.iter(|| black_box(optimizer.create_shared_data("shared_test_data".to_string())))
    });

    group.finish();
}

fn benchmark_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    let sizes = vec![10, 100, 1000, 10000];

    for size in sizes {
        let test_string = "a".repeat(size);

        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("string_clone", size),
            &test_string,
            |b, s| b.iter(|| black_box(s.clone())),
        );

        group.bench_with_input(
            BenchmarkId::new("string_to_arc", size),
            &test_string,
            |b, s| b.iter(|| black_box(std::sync::Arc::<str>::from(s.as_str()))),
        );

        let optimizer = ZeroCopyOptimizer::new();
        group.bench_with_input(
            BenchmarkId::new("string_intern", size),
            &test_string,
            |b, s| b.iter(|| black_box(optimizer.intern_string(s))),
        );
    }

    group.finish();
}

fn benchmark_memory_allocation(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_allocation");

    let sizes = vec![64, 256, 1024, 4096, 16384];

    for size in sizes {
        group.throughput(Throughput::Bytes(size as u64));

        group.bench_with_input(
            BenchmarkId::new("vec_allocation", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let vec: Vec<u8> = black_box(vec![0; size]);
                    black_box(vec);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("vec_with_capacity", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut vec = Vec::with_capacity(size);
                    vec.resize(size, 0);
                    black_box(vec);
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("box_allocation", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let boxed = black_box(vec![0u8; size].into_boxed_slice());
                    black_box(boxed);
                })
            },
        );
    }

    group.finish();
}

fn benchmark_async_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_operations");
    group.throughput(Throughput::Elements(1));

    let rt = tokio::runtime::Runtime::new()
        .map_err(|e| BearDogError::system({:?}", e)))?;

    group.bench_function("simple_async", |b| {
        b.to_async(&rt)
            .iter(|| async { black_box(async_work().await) })
    });

    group.bench_function("async_with_timeout", |b| {
        b.to_async(&rt).iter(|| async {
            let timeout = Duration::from_millis(default_timeout_ms());
            black_box(tokio::time::timeout(timeout, async_work()).await)
        })
    });

    group.finish();
}

async fn async_work() -> u64 {

    tokio::task::yield_now().await;
    42
}

fn benchmark_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");

    let thread_counts = vec![1, 2, 4, 8, 16];

    for threads in thread_counts {
        group.bench_with_input(
            BenchmarkId::new("parallel_work", threads),
            &threads,
            |b, &threads| {
                b.iter(|| {
                    let handles: Vec<_> = (0..threads)
                        .map(|_| {
                            std::thread::spawn(|| {

                                let mut sum = 0u64;
                                for i in 0..1000 {
                                    sum = sum.wrapping_add(i);
                                }
                                black_box(sum)
                            })
                        })
                        .collect();

                    for handle in handles {
                        handle.join().map_err(|e| {
                            BearDogError::system({:?}", e))
                        })?;
                    }
                })
            },
        );
    }

    group.finish();
}

fn benchmark_data_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_structures");

    let sizes = vec![100, 1000, 10000];

    for size in sizes {
        group.throughput(Throughput::Elements(size as u64));

        group.bench_with_input(
            BenchmarkId::new("hashmap_insert", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut map = std::collections::HashMap::with_capacity(16);
                    for i in 0..size {
                        map.insert(i, i * 2);
                    }
                    black_box(map)
                })
            },
        );

        group.bench_with_input(BenchmarkId::new("vec_push", size), &size, |b, &size| {
            b.iter(|| {
                let mut vec = Vec::new();
                for i in 0..size {
                    vec.push(i);
                }
                black_box(vec)
            })
        });

        group.bench_with_input(
            BenchmarkId::new("btreemap_insert", size),
            &size,
            |b, &size| {
                b.iter(|| {
                    let mut map = std::collections::BTreeMap::new();
                    for i in 0..size {
                        map.insert(i, i * 2);
                    }
                    black_box(map)
                })
            },
        );
    }

    group.finish();
}

fn criterion_config() -> Criterion {
    Criterion::default()
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))
        .warm_up_time(Duration::from_millis(100))
        .measurement_time(Duration::from_millis(500))
        .sample_size(100)
}

criterion_group!(
    name = hyperoptimized_benches;
    config = criterion_config();
    targets =
        benchmark_error_creation,
        benchmark_zero_copy_optimization,
        benchmark_string_operations,
        benchmark_memory_allocation,
        benchmark_async_operations,
        benchmark_concurrent_operations,
        benchmark_data_structures
);

criterion_main!(hyperoptimized_benches);
