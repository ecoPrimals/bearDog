//! Benchmarks for the BearDog Sovereign Science Framework

use beardog_sovereign_science::*;
use criterion::{criterion_group, criterion_main, Criterion};
use tokio::runtime::Runtime;

fn benchmark_framework_initialization(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("framework_initialization", |b| {
        b.iter(|| {
            rt.block_on(async {
                let _framework = SovereignScienceFramework::new().await.unwrap();
            })
        });
    });
}

fn benchmark_config_creation(c: &mut Criterion) {
    c.bench_function("config_creation", |b| {
        b.iter(|| {
            let _config = FrameworkConfig::default();
        });
    });
}

fn benchmark_validation_execution(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();

    c.bench_function("basic_validation_execution", |b| {
        b.iter(|| {
            rt.block_on(async {
                let config = FrameworkConfig {
                    enable_cryptographic: true,
                    enable_performance: true,
                    enable_distributed_security: false,
                    enable_human_dignity: false,
                    enable_enterprise: false,
                    ..Default::default()
                };

                let framework = SovereignScienceFramework::with_config(config)
                    .await
                    .unwrap();
                let _results = framework.execute_full_validation().await.unwrap();
            })
        });
    });
}

criterion_group!(
    benches,
    benchmark_framework_initialization,
    benchmark_config_creation,
    benchmark_validation_execution
);
criterion_main!(benches);
