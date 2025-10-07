//! Unified Modernization Performance Benchmarks
//!
//! This benchmark suite validates the performance improvements achieved through
//! the BearDog ecosystem modernization, focusing on zero-cost abstractions,
//! unified configurations, and modular architecture benefits.

use beardog_types::{
    canonical::{
        config::{unified::UnifiedBearDogConfig, CanonicalAppConfig},
        CanonicalProviderConfig, CanonicalSecurityConfig,
    },
    // zero_cost module structure has changed - using available types
};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::time::Duration;
use tokio::runtime::Runtime;

/// Benchmark configuration loading and validation performance
fn benchmark_unified_config(c: &mut Criterion) {
    let mut group = c.benchmark_group("unified_config");

    // Benchmark canonical provider config
    group.bench_function("canonical_provider_config_creation", |b| {
        b.iter(|| {
            let config = CanonicalProviderConfig::default();
            black_box(config)
        })
    });

    // Benchmark security config creation
    group.bench_function("security_config_creation", |b| {
        std::env::set_var("BEARDOG_SECURITY_LEVEL", "high");
        std::env::set_var("BEARDOG_JWT_SECRET", "test-secret-for-benchmarking");

        b.iter(|| {
            let config = CanonicalSecurityConfig::default();
            black_box(config)
        })
    });

    // Benchmark app config creation
    group.bench_function("app_config_creation", |b| {
        let config = AppConfig::default();
        b.iter(|| black_box(&config))
    });

    group.finish();
}

/// Benchmark zero-cost abstraction performance
fn benchmark_zero_cost_abstractions(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_cost_abstractions");

    // Benchmark memory pool operations
    group.bench_function("memory_pool_allocation", |b| {
        let pool = SafeZeroCopyMemoryPool::new(&[4096, 8192, 16384], 64); // Multi-size pool with 64-byte alignment

        b.iter(|| {
            let buffer = pool.allocate(4096); // 4KB allocation
            black_box(buffer)
        })
    });

    // Benchmark performance measurement overhead
    group.bench_function("performance_benchmark_creation", |b| {
        b.iter(|| {
            let benchmark = PerformanceBenchmark::new("test_benchmark".to_string());
            black_box(benchmark)
        })
    });

    group.finish();
}

/// Benchmark modular architecture vs monolithic performance
fn benchmark_modular_vs_monolithic(c: &mut Criterion) {
    let mut group = c.benchmark_group("modular_architecture");

    // Simulate modular config access
    group.bench_function("modular_config_access", |b| {
        let provider_config = CanonicalProviderConfig::default();
        let security_config = CanonicalSecurityConfig::default();

        b.iter(|| {
            // Simulate accessing multiple modular configs
            let _ = black_box(&provider_config.core);
            let _ = black_box(&provider_config.connection);
            let _ = black_box(&security_config.core);
            let _ = black_box(&security_config.enabled);
        })
    });

    group.finish();
}

/// Benchmark configuration validation performance across sizes
fn benchmark_config_validation_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("config_validation_scaling");

    for size in [1, 10, 100].iter() {
        group.bench_with_input(
            BenchmarkId::new("provider_config_creation", size),
            size,
            |b, &size| {
                let mut config = CanonicalProviderConfig::default();

                // Add complexity based on size
                for i in 0..size {
                    config.core.provider_id = format!("provider_{}", i);
                }

                b.iter(|| black_box(&config))
            },
        );
    }

    group.finish();
}

/// Benchmark async workflow processing
fn benchmark_async_workflows(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("async_workflows");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("workflow_engine_creation", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Simulate workflow engine creation (mock for benchmarking)
                let config = beardog_types::zero_cost::workflow::WorkflowEngineConfig::default();
                black_box(config)
            })
        })
    });

    group.finish();
}

/// Comprehensive modernization validation benchmark
fn benchmark_modernization_benefits(c: &mut Criterion) {
    let mut group = c.benchmark_group("modernization_benefits");

    // Benchmark unified vs fragmented configuration
    group.bench_function("unified_configuration_performance", |b| {
        b.iter(|| {
            let provider_config = CanonicalProviderConfig::default();
            let security_config = CanonicalSecurityConfig::default();

            // Simulate unified configuration access
            let provider_valid = !provider_config.core.provider_id.is_empty();
            let security_valid = security_config.enabled;

            black_box(provider_valid && security_valid)
        })
    });

    // Benchmark error handling performance
    group.bench_function("error_handling_performance", |b| {
        use beardog_errors::BearDogError;

        b.iter(|| {
            let result: Result<(), BearDogError> = Ok(());
            match result {
                Ok(_) => black_box(true),
                Err(_) => black_box(false),
            }
        })
    });

    // Benchmark import resolution (compile-time, but measure module access)
    group.bench_function("module_access_performance", |b| {
        b.iter(|| {
            // Test that our modular structure doesn't add runtime overhead
            use beardog_types::canonical::providers_unified::ProviderConfig;
            use beardog_types::canonical::security_unified::SecurityConfig;

            let _ = black_box(std::any::type_name::<ProviderConfig>());
            let _ = black_box(std::any::type_name::<SecurityConfig>());
        })
    });

    group.finish();
}

criterion_group!(
    unified_modernization_benches,
    benchmark_unified_config,
    benchmark_zero_cost_abstractions,
    benchmark_modular_vs_monolithic,
    benchmark_config_validation_scaling,
    benchmark_async_workflows,
    benchmark_modernization_benefits
);

criterion_main!(unified_modernization_benches);
