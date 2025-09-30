use criterion::{black_box, criterion_group, criterion_main, Criterion};
use beardog_core::ecosystem::primal_types::{ResponseTimeMetrics, LoadMetrics, ErrorRateMetrics};

fn benchmark_const_methods(c: &mut Criterion) {
    c.bench_function("ResponseTimeMetrics::zero", |b| {
        b.iter(|| {
            black_box(ResponseTimeMetrics::zero())
        })
    });

    c.bench_function("ResponseTimeMetrics::default", |b| {
        b.iter(|| {
            black_box(ResponseTimeMetrics::default())
        })
    });

    c.bench_function("LoadMetrics::zero", |b| {
        b.iter(|| {
            black_box(LoadMetrics::zero())
        })
    });

    c.bench_function("LoadMetrics::default", |b| {
        b.iter(|| {
            black_box(LoadMetrics::default())
        })
    });

    c.bench_function("ResponseTimeMetrics::is_healthy", |b| {
        let metrics = ResponseTimeMetrics::zero();
        b.iter(|| {
            black_box(metrics.is_healthy(100.0, 500.0))
        })
    });

    c.bench_function("LoadMetrics::is_high_load", |b| {
        let metrics = LoadMetrics::zero();
        b.iter(|| {
            black_box(metrics.is_high_load())
        })
    });

    c.bench_function("ErrorRateMetrics::is_healthy", |b| {
        let metrics = ErrorRateMetrics::default();
        b.iter(|| {
            black_box(metrics.is_healthy())
        })
    });
}

criterion_group!(benches, benchmark_const_methods);
criterion_main!(benches); 