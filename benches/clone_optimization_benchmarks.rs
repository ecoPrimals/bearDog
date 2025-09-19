//! Clone optimization benchmarks for BearDog performance analysis
//! Demonstrates zero-copy patterns and memory efficiency improvements

use beardog_utils::optimization::{global_interner_stats, intern_string};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::sync::Arc;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct OriginalConnection {
    connection_id: String,
    user_id: String,
    endpoint: String,
    metadata: std::collections::HashMap<String, String>,
}

impl OriginalConnection {
    fn new(connection_id: &str, user_id: &str, endpoint: &str) -> Self {
        let mut metadata = std::collections::HashMap::with_capacity(16);
        metadata.insert("status".to_string(), "active".to_string());
        metadata.insert("protocol".to_string(), "https".to_string());
        metadata.insert("version".to_string(), "1.0".to_string());

        Self {
            connection_id: connection_id.to_string(),
            user_id: user_id.to_string(),
            endpoint: endpoint.to_string(),
            metadata,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OptimizedConnection {
    connection_id: Arc<str>,
    user_id: Arc<str>,
    endpoint: Arc<str>,
    metadata: Arc<std::collections::HashMap<Arc<str>, Arc<str>>>,
}

impl OptimizedConnection {
    fn new(connection_id: &str, user_id: &str, endpoint: &str) -> Self {
        let mut metadata = std::collections::HashMap::with_capacity(16);
        metadata.insert(Arc::from("status"), Arc::from("active"));
        metadata.insert(Arc::from("protocol"), Arc::from("https"));
        metadata.insert(Arc::from("version"), Arc::from("1.0"));

        Self {
            connection_id: Arc::from(connection_id),
            user_id: Arc::from(user_id),
            endpoint: Arc::from(endpoint),
            metadata: Arc::new(metadata),
        }
    }
}

fn benchmark_original_cloning(c: &mut Criterion) {
    c.bench_function("original_connection_clone", |b| {
        let conn = OriginalConnection::new("conn_123", "user_456", "https://api.example.com");
        b.iter(|| {
            let _cloned = black_box(conn.clone());
        });
    });
}

fn benchmark_optimized_cloning(c: &mut Criterion) {
    c.bench_function("optimized_connection_clone", |b| {
        let conn = OptimizedConnection::new("conn_123", "user_456", "https://api.example.com");
        b.iter(|| {
            let _cloned = black_box(conn.clone());
        });
    });
}

fn benchmark_string_interning(c: &mut Criterion) {
    c.bench_function("string_interning", |b| {
        b.iter(|| {
            let _interned = black_box(intern_string("test_string_for_interning"));
        });
    });
}

fn benchmark_batch_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch_operations");

    for batch_size in [10, 100, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("original", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let connections: Vec<_> = (0..size)
                        .map(|i| {
                            OriginalConnection::new(
                                &format!("conn_{}", i),
                                &format!("user_{}", i),
                                "https://api.example.com",
                            )
                        })
                        .collect();
                    black_box(connections);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("optimized", batch_size),
            batch_size,
            |b, &size| {
                b.iter(|| {
                    let connections: Vec<_> = (0..size)
                        .map(|i| {
                            OptimizedConnection::new(
                                &format!("conn_{}", i),
                                &format!("user_{}", i),
                                "https://api.example.com",
                            )
                        })
                        .collect();
                    black_box(connections);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(
    benches,
    benchmark_original_cloning,
    benchmark_optimized_cloning,
    benchmark_string_interning,
    benchmark_batch_operations
);
criterion_main!(benches);
