use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashMap;
use std::sync::Arc;

// Simulate the old cloning approach
fn clone_heavy_operation(data: &Vec<u8>) -> Vec<u8> {
    data.clone()
}

// Simulate the new Arc-based approach
fn arc_shared_operation(data: &Arc<Vec<u8>>) -> Arc<Vec<u8>> {
    Arc::clone(data)
}

// Benchmark string allocation patterns
fn string_allocation_old(count: usize) -> Vec<String> {
    let mut results = Vec::new();
    for i in 0..count {
        results.push(format!("item_{}", i).to_string());
    }
    results
}

fn string_allocation_optimized(count: usize) -> Vec<String> {
    let mut results = Vec::with_capacity(count);
    for i in 0..count {
        results.push(format!("item_{}", i)); // No unnecessary .to_string()
    }
    results
}

// Benchmark HashMap creation patterns
fn hashmap_creation_old() -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("key1".to_string(), "value1".to_string());
    map.insert("key2".to_string(), "value2".to_string());
    map.insert("key3".to_string(), "value3".to_string());
    map
}

fn hashmap_creation_optimized() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::with_capacity(3);
    map.insert("key1", "value1");
    map.insert("key2", "value2");
    map.insert("key3", "value3");
    map
}

fn benchmark_data_cloning(c: &mut Criterion) {
    let data = vec![0u8; 1024 * 1024]; // 1MB of data
    let arc_data = Arc::new(data.clone());

    c.bench_function("clone_heavy_1mb", |b| {
        b.iter(|| clone_heavy_operation(black_box(&data)))
    });

    c.bench_function("arc_shared_1mb", |b| {
        b.iter(|| arc_shared_operation(black_box(&arc_data)))
    });
}

fn benchmark_string_allocation(c: &mut Criterion) {
    c.bench_function("string_allocation_old_1000", |b| {
        b.iter(|| string_allocation_old(black_box(1000)))
    });

    c.bench_function("string_allocation_optimized_1000", |b| {
        b.iter(|| string_allocation_optimized(black_box(1000)))
    });
}

fn benchmark_hashmap_creation(c: &mut Criterion) {
    c.bench_function("hashmap_creation_old", |b| {
        b.iter(|| hashmap_creation_old())
    });

    c.bench_function("hashmap_creation_optimized", |b| {
        b.iter(|| hashmap_creation_optimized())
    });
}

criterion_group!(
    benches,
    benchmark_data_cloning,
    benchmark_string_allocation,
    benchmark_hashmap_creation
);
criterion_main!(benches);
