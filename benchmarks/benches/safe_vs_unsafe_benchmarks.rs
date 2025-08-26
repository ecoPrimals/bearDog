

use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box as std_black_box;

const BUFFER_SIZE: usize = 1024;
const SMALL_VEC_SIZE: usize = 32;

fn benchmark_buffer_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_operations");

    let test_data = vec![0xAA; BUFFER_SIZE];

    group.bench_function("safe_slice_access", |b| {
        let buffer = test_data.clone();
        b.iter(|| {
            let slice = buffer.as_slice();
            std_black_box(slice.len())
        })
    });

    group.bench_function("safe_mut_slice_access", |b| {
        b.iter(|| {
            let mut buffer = test_data.clone();
            let slice = buffer.as_mut_slice();
            std_black_box(slice.len())
        })
    });

    group.finish();
}

fn benchmark_safe_vector_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("safe_vector_operations");

    let test_data: Vec<u8> = (0..SMALL_VEC_SIZE).map(|i| (i % 256) as u8).collect();

    group.bench_function("safe_vector_creation", |b| {
        b.iter(|| {
            let mut safe_vec: Vec<u8> = Vec::with_capacity(SMALL_VEC_SIZE);
            for &byte in &test_data {
                safe_vec.push(byte);
            }
            std_black_box(safe_vec.len())
        })
    });

    group.bench_function("safe_vector_access", |b| {
        let mut safe_vec: Vec<u8> = Vec::with_capacity(SMALL_VEC_SIZE);
        for &byte in &test_data {
            safe_vec.push(byte);
        }

        b.iter(|| {
            let mut sum = 0u32;

            for &value in safe_vec.iter() {
                sum += u32::from(value);
            }
            std_black_box(sum)
        })
    });

    group.finish();
}

fn benchmark_encryption_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("encryption_operations");

    let test_data = vec![0x42; 1024];

    group.bench_function("safe_encryption_simulation", |b| {
        b.iter(|| {

            let mut encrypted = test_data.clone();
            for byte in &mut encrypted {
                *byte ^= 0xAA; // Simple XOR "encryption"
            }
            std_black_box(encrypted.len())
        })
    });

    group.finish();
}

fn benchmark_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");

    group.bench_function("stack_allocation", |b| {
        b.iter(|| {

            let buffer = [0u8; 256];
            let sum: u32 = buffer.iter().map(|&x| u32::from(x)).sum();
            std_black_box(sum)
        })
    });

    group.bench_function("heap_allocation", |b| {
        b.iter(|| {

            let buffer = vec![0u8; 256];
            let sum: u32 = buffer.iter().map(|&x| u32::from(x)).sum();
            std_black_box(sum)
        })
    });

    group.finish();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_safe_implementations_work() {

        let buffer = vec![1, 2, 3, 4];
        assert_eq!(buffer.as_slice(), &[1, 2, 3, 4]);

        let mut safe_vec: Vec<u8> = Vec::with_capacity(4);
        safe_vec.push(1);
        safe_vec.push(2);
        assert_eq!(safe_vec.len(), 2);

        let values: Vec<u8> = safe_vec.to_vec();
        assert_eq!(values[0], 1);
        assert_eq!(values[1], 2);
    }
}

criterion_group!(
    benches,
    benchmark_buffer_operations,
    benchmark_safe_vector_operations,
    benchmark_encryption_operations,
    benchmark_memory_patterns
);

criterion_main!(benches);

fn print_assembly_inspection_instructions() {
    println!("🔍 **ASSEMBLY INSPECTION INSTRUCTIONS**");
    println!("To verify zero-cost abstractions, inspect the generated assembly:");
    println!("💡 To verify: cargo asm --release --bench safe_vs_unsafe_benchmarks");
    println!("📊 Expected: Identical assembly for safe vs unsafe versions");
    println!("🎯 Goal: Prove safety without performance cost");
}
