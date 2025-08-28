use beardog_errors::BearDogError;
use beardog_types::config::*;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;
use tokio::runtime::Runtime;

fn benchmark_database_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("database_config");

    let configs = vec![
        ("default", OptimizedDatabaseConfig::default()),
        ("production", OptimizedDatabaseConfig::default()), // Use default as production placeholder
        ("development", OptimizedDatabaseConfig::default()), // Use default as development placeholder
    ];

    for (name, config) in configs {
        group.bench_with_input(
            BenchmarkId::new("validation", name),
            &config,
            |b, config| {
                b.iter(|| {
                    config.validate().map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(
                            format_args!("Operation failed: {:?}", e).to_string(),
                        )
                    })?;
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("serialization", name),
            &config,
            |b, config| {
                b.iter(|| {
                    toml::to_string(&config).map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(
                            format_args!("Operation failed: {:?}", e).to_string(),
                        )
                    })?;
                });
            },
        );
    }

    group.finish();
}

fn benchmark_memory_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_config");

    let configs = vec![
        ("default", MemoryOptimizationConfig::default()),
        ("production", MemoryOptimizationConfig::production()),
        ("development", MemoryOptimizationConfig::development()),
    ];

    for (name, config) in configs {
        group.bench_with_input(
            BenchmarkId::new("pool_size_calculation", name),
            &config,
            |b, config| {
                b.iter(|| {
                    for pool_config in config.object_pooling.pools.values() {
                        let _ = memory::utils::calculate_optimal_pool_size(
                            pool_config.initial_size,
                            0.8, // 80% usage rate
                            1.2, // 20% growth rate
                        );
                    }
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("memory_estimation", name),
            &config,
            |b, config| {
                b.iter(|| {
                    for obj_type in config.object_pooling.pools.keys() {
                        let _ = memory::utils::estimate_object_memory_usage(obj_type);
                    }
                });
            },
        );
    }

    group.finish();
}

fn benchmark_async_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("async_config");

    let configs = vec![
        ("default", AsyncOptimizationConfig::default()),
        ("production", AsyncOptimizationConfig::production()),
        ("development", AsyncOptimizationConfig::development()),
    ];

    for (name, config) in configs {
        group.bench_with_input(
            BenchmarkId::new("batch_size_calculation", name),
            &config,
            |b, _config| {
                b.iter(|| {
                    let _ = async_optimization::utils::calculate_optimal_batch_size(
                        Duration::from_millis(10),
                        0.7,                    // 70% system load
                        8 * 1024 * 1024 * 1024, // 8GB available memory
                    );
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("concurrency_calculation", name),
            &config,
            |b, config| {
                b.iter(|| {
                    let _ = async_optimization::utils::calculate_optimal_concurrency(
                        config.parallel_processing.worker_threads,
                        0.6, // 60% I/O ratio
                        16,  // 16GB memory
                    );
                });
            },
        );
    }

    group.finish();
}

fn benchmark_unified_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("unified_config");

    let configs = vec![
        ("default", BearDogConfig::default()),
        ("production", BearDogConfig::default()), // Use default as production placeholder
        ("development", BearDogConfig::default()), // Use default as development placeholder
    ];

    for (name, config) in configs {
        group.bench_with_input(
            BenchmarkId::new("full_validation", name),
            &config,
            |b, config| {
                b.iter(|| {
                    config.validate().map_err(|e| {
                        tracing::error!("Operation failed: {:?}", e);
                        beardog_errors::BearDogError::internal(
                            format_args!("Operation failed: {:?}", e).to_string(),
                        )
                    })?;
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("resource_estimation", name),
            &config,
            |b, config| {
                b.iter(|| {
                    let _ = config.validate(); // Use the existing validate method
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("environment_selection", name),
            &config,
            |b, _config| {
                b.iter(|| {
                    let _ = BearDogConfig::from_env(); // Use the existing from_env method
                });
            },
        );
    }

    group.finish();
}

fn benchmark_parallel_processing_simulation(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_processing");

    let rt = Runtime::new().map_err(|e| {
        tracing::error!(
            "Operation failed ({}): {:?}",
            "Benchmark runtime creation failed",
            e
        );
        beardog_errors::BearDogError::internal(
            format_args!(
                "Operation failed ({}): {:?}",
                "Benchmark runtime creation failed", e
            )
            .to_string(),
        )
    })?;
    let configs = vec![
        ("default", AsyncOptimizationConfig::default()),
        ("production", AsyncOptimizationConfig::production()),
    ];

    for (name, config) in configs {
        let worker_threads = config.parallel_processing.worker_threads;

        group.throughput(Throughput::Elements(worker_threads as u64));
        group.bench_with_input(
            BenchmarkId::new("simulated_parallel_work", name),
            &config,
            |b, config| {
                b.iter(|| {
                    rt.block_on(async {
                        simulate_parallel_work(config.parallel_processing.worker_threads as usize)
                            .await
                    })
                });
            },
        );
    }

    group.finish();
}

async fn simulate_parallel_work(worker_count: usize) {
    let tasks: Vec<_> = (0..worker_count)
        .map(|i| {
            tokio::spawn(async move {
                let mut sum = 0;
                for j in 0..1000 {
                    sum += i * j;
                }
                sum
            })
        })
        .collect();

    for task in tasks {
        let _ = task.await;
    }
}

fn benchmark_cache_config_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("cache_config");

    let configs = vec![
        ("small_cache", create_small_cache_config()),
        ("large_cache", create_large_cache_config()),
        ("production_cache", MemoryOptimizationConfig::production()),
    ];

    for (name, config) in configs {
        group.bench_with_input(
            BenchmarkId::new("cache_simulation", name),
            &config,
            |b, config| {
                b.iter(|| {
                    simulate_cache_operations(config);
                });
            },
        );
    }

    group.finish();
}

fn create_small_cache_config() -> MemoryOptimizationConfig {
    let mut config = MemoryOptimizationConfig::default();
    config.cache.l1_cache.size_kb = 32;
    config.cache.l2_cache.size_mb = 2;
    config
}

fn create_large_cache_config() -> MemoryOptimizationConfig {
    let mut config = MemoryOptimizationConfig::default();
    config.cache.l1_cache.size_kb = 256;
    config.cache.l2_cache.size_mb = 32;
    config
}

fn simulate_cache_operations(config: &MemoryOptimizationConfig) {
    let l1_operations = config.cache.l1_cache.size_kb * 10;
    let l2_operations = config.cache.l2_cache.size_mb * 100;

    let mut total_operations = 0;
    for _ in 0..l1_operations {
        total_operations += 1;
    }
    for _ in 0..l2_operations {
        total_operations += 1;
    }

    let hit_ratio = total_operations as f64 / (total_operations as f64 + 100.0);
    let _ = hit_ratio;
}

fn benchmark_performance_monitoring_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("performance_monitoring");

    let configs = vec![
        ("minimal_monitoring", create_minimal_monitoring_config()),
        ("full_monitoring", create_full_monitoring_config()),
        ("production_monitoring", PerformanceConfig::production()),
    ];

    for (name, config) in configs {
        group.bench_with_input(
            BenchmarkId::new("monitoring_overhead", name),
            &config,
            |b, config| {
                b.iter(|| {
                    simulate_monitoring_overhead(config);
                });
            },
        );
    }

    group.finish();
}

fn create_minimal_monitoring_config() -> PerformanceConfig {
    let mut config = PerformanceConfig::default();
    config.monitoring.enabled = true;
    config.monitoring.enable_detailed_metrics = false;
    config.monitoring.metrics_interval = Duration::from_secs(60);
    config
}

fn create_full_monitoring_config() -> PerformanceConfig {
    let mut config = PerformanceConfig::default();
    config.monitoring.enabled = true;
    config.monitoring.enable_detailed_metrics = true;
    config.monitoring.metrics_interval = Duration::from_secs(1);
    config
}

fn simulate_monitoring_overhead(config: &PerformanceConfig) {
    if !config.monitoring.enabled {
        return;
    }

    let metrics_count = if config.monitoring.enable_detailed_metrics {
        50
    } else {
        10
    };
    let mut collected_metrics = Vec::new();

    for i in 0..metrics_count {
        let metric_value = i as f64 * 1.5;
        collected_metrics.push(metric_value);
    }

    let average = collected_metrics.iter().sum::<f64>() / collected_metrics.len() as f64;
    let _ = average;
}

criterion_group!(
    benches,
    benchmark_database_config,
    benchmark_memory_config,
    benchmark_async_config,
    benchmark_unified_config,
    benchmark_parallel_processing_simulation,
    benchmark_cache_config_impact,
    benchmark_performance_monitoring_overhead
);

criterion_main!(benches);
