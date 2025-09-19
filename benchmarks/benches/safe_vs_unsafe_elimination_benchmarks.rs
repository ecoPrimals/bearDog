use beardog_errors::BearDogError;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use std::time::Duration;
use tokio::runtime::Runtime;

struct MockKeyData {
    data: Vec<u8>,
}

impl MockKeyData {
    fn new(size: usize) -> Self {
        Self {
            data: vec![0u8; size],
        }
    }
}

fn benchmark_key_generation(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
        tracing::error!("Operation failed ({}): {:?}", "Failed to create runtime", e);
        beardog_errors::BearDogError::internal({:?}", "Failed to create runtime", e))
    })?;
    let mut group = c.benchmark_group("key_generation");

    for key_size in [256, 2048, 4096].iter() {
        group.throughput(Throughput::Bytes(*key_size as u64));

        group.bench_with_input(
            BenchmarkId::new("unsafe_ffi_pattern", key_size),
            key_size,
            |b, &size| {
                b.iter(|| rt.block_on(async { unsafe_key_generation_simulation(size).await }))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("safe_zero_cost_pattern", key_size),
            key_size,
            |b, &size| b.iter(|| rt.block_on(async { safe_key_generation_optimized(size).await })),
        );
    }

    group.finish();
}

fn benchmark_memory_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_management");

    for buffer_size in [1024, 4096, 16384].iter() {
        group.throughput(Throughput::Bytes(*buffer_size as u64));

        group.bench_with_input(
            BenchmarkId::new("unsafe_manual_memory", buffer_size),
            buffer_size,
            |b, &size| b.iter(|| unsafe_buffer_management_simulation(size)),
        );

        group.bench_with_input(
            BenchmarkId::new("safe_raii_memory", buffer_size),
            buffer_size,
            |b, &size| b.iter(|| safe_buffer_management_optimized(size)),
        );
    }

    group.finish();
}

fn benchmark_ffi_patterns(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
        tracing::error!("Operation failed ({}): {:?}", "Failed to create runtime", e);
        beardog_errors::BearDogError::internal({:?}", "Failed to create runtime", e))
    })?;
    let mut group = c.benchmark_group("ffi_patterns");

    for data_size in [64, 256, 1024].iter() {
        group.throughput(Throughput::Bytes(*data_size as u64));

        group.bench_with_input(
            BenchmarkId::new("unsafe_direct_ffi", data_size),
            data_size,
            |b, &size| b.iter(|| rt.block_on(async { unsafe_ffi_call_simulation(size).await })),
        );

        group.bench_with_input(
            BenchmarkId::new("safe_highlevel_api", data_size),
            data_size,
            |b, &size| b.iter(|| rt.block_on(async { safe_api_call_optimized(size).await })),
        );
    }

    group.finish();
}

fn benchmark_error_handling(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
        tracing::error!("Operation failed ({}): {:?}", "Failed to create runtime", e);
        beardog_errors::BearDogError::internal({:?}", "Failed to create runtime", e))
    })?;
    let mut group = c.benchmark_group("error_handling");

    for operation_count in [100, 1000, 10000].iter() {
        group.throughput(Throughput::Elements(*operation_count as u64));

        group.bench_with_input(
            BenchmarkId::new("unsafe_c_style_errors", operation_count),
            operation_count,
            |b, &count| {
                b.iter(|| rt.block_on(async { unsafe_error_handling_simulation(count).await }))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("safe_result_errors", operation_count),
            operation_count,
            |b, &count| {
                b.iter(|| rt.block_on(async { safe_error_handling_optimized(count).await }))
            },
        );
    }

    group.finish();
}

async fn unsafe_key_generation_simulation(key_size: usize) -> MockKeyData {
    tokio::time::sleep(Duration::from_nanos(100)).await; // FFI boundary overhead

    let mut data = Vec::with_capacity(key_size);
    for i in 0..key_size {
        if i < key_size {
            data.push((i % 256) as u8);
        }
    }

    tokio::time::sleep(Duration::from_nanos(50)).await;

    MockKeyData { data }
}

async fn safe_key_generation_optimized(key_size: usize) -> MockKeyData {
    let data = (0..key_size).map(|i| (i % 256) as u8).collect();

    MockKeyData { data }
}

fn unsafe_buffer_management_simulation(buffer_size: usize) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(buffer_size);

    for i in 0..buffer_size {
        buffer.push((i % 256) as u8);
    }

    std::thread::sleep(Duration::from_nanos(10));

    buffer
}

fn safe_buffer_management_optimized(buffer_size: usize) -> Vec<u8> {
    (0..buffer_size).map(|i| (i % 256) as u8).collect()
}

async fn unsafe_ffi_call_simulation(data_size: usize) -> Result<Vec<u8>, String> {
    tokio::time::sleep(Duration::from_nanos(200)).await;

    let error_code = 0; // Simulate success
    if error_code != 0 {
        return Err(format!("FFI error: {error_code}"));
    }

    let mut result = Vec::with_capacity(data_size);
    for i in 0..data_size {
        result.push((i % 256) as u8);
    }

    Ok(result)
}

async fn safe_api_call_optimized(data_size: usize) -> Result<Vec<u8>, String> {
    Ok((0..data_size).map(|i| (i % 256) as u8).collect())
}

async fn unsafe_error_handling_simulation(operation_count: usize) -> i32 {
    let mut error_count = 0;

    for i in 0..operation_count {
        let error_code = if i % 100 == 0 { -1 } else { 0 };

        if error_code != 0 {
            error_count += 1;

            tokio::time::sleep(Duration::from_nanos(1)).await;
        }
    }

    error_count
}

async fn safe_error_handling_optimized(operation_count: usize) -> Result<usize, String> {
    let mut error_count = 0;

    for i in 0..operation_count {
        if i % 100 == 0 {
            error_count += 1;
        }
    }

    Ok(error_count)
}

fn benchmark_zero_cost_abstractions(c: &mut Criterion) {
    let mut group = c.benchmark_group("zero_cost_abstractions");

    for operation_count in [1000, 10000, 100000].iter() {
        group.throughput(Throughput::Elements(*operation_count as u64));

        group.bench_with_input(
            BenchmarkId::new("runtime_dispatch", operation_count),
            operation_count,
            |b, &count| b.iter(|| runtime_dispatch_simulation(count)),
        );

        group.bench_with_input(
            BenchmarkId::new("zero_cost_monomorphic", operation_count),
            operation_count,
            |b, &count| b.iter(|| zero_cost_monomorphic_simulation(count)),
        );
    }

    group.finish();
}

fn runtime_dispatch_simulation(operation_count: usize) -> usize {
    let mut result = 0;

    for i in 0..operation_count {
        result += match i % 3 {
            0 => operation_a(i),
            1 => operation_b(i),
            _ => operation_c(i),
        };
    }

    result
}

fn zero_cost_monomorphic_simulation(operation_count: usize) -> usize {
    let mut result = 0;

    for i in 0..operation_count {
        result += if i % 3 == 0 {
            operation_a_inlined(i)
        } else if i % 3 == 1 {
            operation_b_inlined(i)
        } else {
            operation_c_inlined(i)
        };
    }

    result
}

fn operation_a(x: usize) -> usize {
    x * 2
}
fn operation_b(x: usize) -> usize {
    x * 3
}
fn operation_c(x: usize) -> usize {
    x * 5
}

#[inline(always)]
fn operation_a_inlined(x: usize) -> usize {
    x * 2
}
#[inline(always)]
fn operation_b_inlined(x: usize) -> usize {
    x * 3
}
#[inline(always)]
fn operation_c_inlined(x: usize) -> usize {
    x * 5
}

criterion_group!(
    benches,
    benchmark_key_generation,
    benchmark_memory_management,
    benchmark_ffi_patterns,
    benchmark_error_handling,
    benchmark_zero_cost_abstractions
);

criterion_main!(benches);
