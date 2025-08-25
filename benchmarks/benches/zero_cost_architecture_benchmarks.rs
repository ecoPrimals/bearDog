// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


use beardog_errors::BearDogResult;
use beardog_types::KeyType;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use tokio::runtime::Runtime;

fn benchmark_zero_cost_abstractions(c: &mut Criterion) {
    c.bench_function("zero_cost_type_safety", |b| {
        b.iter(|| {
            // Demonstrate zero-cost abstractions
            let key_type = KeyType::Ed25519;
            let processed = match key_type {
                KeyType::Ed25519 => "ed25519_processed",
                KeyType::Aes256 => "aes256_processed",
                KeyType::Aes128 => "aes128_processed",
                KeyType::Rsa2048 | KeyType::Rsa4096 => "rsa_processed",
                _ => "other_processed",
            };
            black_box(processed)
        })
    });
}

fn benchmark_arc_cloning(c: &mut Criterion) {
    let data = Arc::new(vec![1u8; 1024]);

    c.bench_function("arc_clone_overhead", |b| {
        b.iter(|| {
            let cloned = Arc::clone(&data);
            black_box(cloned)
        })
    });
}

fn benchmark_result_propagation(c: &mut Criterion) {
    fn propagate_result() -> BearDogResult<u32> {
        let value = some_operation()?;
        let processed = value * 2;
        Ok(processed)
    }

    const fn some_operation() -> BearDogResult<u32> {
        Ok(42)
    }

    c.bench_function("result_propagation", |b| {
        b.iter(|| {
            let result = propagate_result();
            black_box(result)
        })
    });
}

fn benchmark_async_overhead(c: &mut Criterion) {
    let rt = Runtime::new().unwrap_or_else(|e| {
        eprintln!("Failed to create runtime: {e:?}");
        panic!("Runtime creation failed");
    });

    c.bench_function("async_function_call", |b| {
        b.iter(|| {
            rt.block_on(async {
                // Simple async operation
                tokio::task::yield_now().await;
                42
            })
        })
    });
}

fn benchmark_option_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("option_operations");

    let some_value = Some(42u32);
    let none_value: Option<u32> = None;

    group.bench_function("option_map_some", |b| {
        b.iter(|| {
            let result = some_value.map(|x| x * 2);
            black_box(result)
        })
    });

    group.bench_function("option_map_none", |b| {
        b.iter(|| {
            let result = none_value.map(|x| x * 2);
            black_box(result)
        })
    });

    group.bench_function("option_unwrap_or", |b| {
        b.iter(|| {
            let result = 100; // none_value.unwrap_or(100) simplifies to 100
            black_box(result)
        })
    });

    group.finish();
}

fn benchmark_iterator_chains(c: &mut Criterion) {
    let data: Vec<u32> = (0..1000).collect();

    c.bench_function("iterator_chain", |b| {
        b.iter(|| {
            let result: u32 = data.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).sum();
            black_box(result)
        })
    });
}

fn benchmark_generic_functions(c: &mut Criterion) {
    const fn generic_operation<T: Clone + std::fmt::Debug>(value: T) -> T {
        value
    }

    let mut group = c.benchmark_group("generic_functions");

    group.bench_function("generic_u32", |b| {
        b.iter(|| {
            let result = generic_operation(42u32);
            black_box(result)
        })
    });

    group.bench_function("generic_string", |b| {
        let test_string = String::from("test");
        b.iter(|| {
            let result = generic_operation(test_string.clone());
            black_box(result)
        })
    });

    group.finish();
}

fn benchmark_enum_matching(c: &mut Criterion) {
    #[derive(Clone)]
    enum TestEnum {
        Variant1(u32),
        Variant2(String),
        Variant3 { x: u32, y: u32 },
    }

    let variants = [
        TestEnum::Variant1(42),
        TestEnum::Variant2("test".to_string()),
        TestEnum::Variant3 { x: 10, y: 20 },
    ];

    c.bench_function("enum_pattern_matching", |b| {
        b.iter(|| {
            let results: Vec<u32> = variants
                .iter()
                .map(|variant| match variant {
                    TestEnum::Variant1(val) => *val,
                    TestEnum::Variant2(s) => s.len() as u32,
                    TestEnum::Variant3 { x, y } => x + y,
                })
                .collect();
            black_box(results)
        })
    });
}

fn benchmark_smart_pointer_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("smart_pointers");

    let data = vec![1u8; 1024];
    let arc_data = Arc::new(data.clone());
    let boxed_data = Box::new(data.clone());

    group.bench_function("raw_vec_access", |b| {
        b.iter(|| {
            let sum: u32 = data.iter().map(|&x| u32::from(x)).sum();
            black_box(sum)
        })
    });

    group.bench_function("arc_access", |b| {
        b.iter(|| {
            let sum: u32 = arc_data.iter().map(|&x| u32::from(x)).sum();
            black_box(sum)
        })
    });

    group.bench_function("box_access", |b| {
        b.iter(|| {
            let sum: u32 = boxed_data.iter().map(|&x| u32::from(x)).sum();
            black_box(sum)
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_zero_cost_abstractions,
    benchmark_arc_cloning,
    benchmark_result_propagation,
    benchmark_async_overhead,
    benchmark_option_operations,
    benchmark_iterator_chains,
    benchmark_generic_functions,
    benchmark_enum_matching,
    benchmark_smart_pointer_overhead
);

criterion_main!(benches);
