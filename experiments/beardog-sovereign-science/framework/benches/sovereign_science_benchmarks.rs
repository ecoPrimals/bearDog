//! Benchmarks for the BearDog Sovereign Science Framework

use criterion::{criterion_group, criterion_main, Criterion};
use beardog_sovereign_science::*;

fn benchmark_framework_initialization(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("framework_initialization", |b| {
        b.to_async(&rt).iter(|| async {
            let _framework = SovereignScienceFramework::new().await.unwrap();
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

criterion_group!(benches, benchmark_framework_initialization, benchmark_config_creation);
criterion_main!(benches); 