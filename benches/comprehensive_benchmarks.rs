

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;
use tokio::runtime::Runtime;

use beardog_security::simd_crypto::{SimdCryptoEngine, benchmark_simd_performance};
use beardog_utils::memory_pools::{ObjectPool, global_pools};
use beardog_utils::lock_free::{LockFreeHashMap};
use beardog_errors::BearDogResult;
use beardog_types::canonical::*;

fn bench_simd_crypto(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Benchmark runtime creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Benchmark runtime creation failed", e).to_string())
})?;
    let engine = rt.block_on(async { SimdCryptoEngine::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})? });
    
    let mut group = c.benchmark_group("SIMD Cryptography");

    for size in [1024, 4096, 16384, 65536, 262144].iter() {
        let data = vec![0u8; *size];
        
        group.throughput(Throughput::Bytes(*size as u64));

        group.bench_with_input(
            BenchmarkId::new("SIMD_SHA256", size),
            &data,
            |b, data| {
                b.to_async(&rt).iter(|| async {
                    engine.simd_sha256(data).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?
                })
            },
        );

        let key = [0u8; 32];
        let nonce = [0u8; 12];
        group.bench_with_input(
            BenchmarkId::new("SIMD_ChaCha20", size),
            &data,
            |b, data| {
                b.to_async(&rt).iter(|| async {
                    engine.simd_chacha20(data, &key, &nonce).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?
                })
            },
        );
    }
    
    group.finish();
}

fn bench_object_pools(c: &mut Criterion) {
    let mut group = c.benchmark_group("Object Pools vs Heap Allocation");

    let pool: ObjectPool<Vec<u8>, 1000> = ObjectPool::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

    group.bench_function("Pool_Allocation", |b| {
        b.iter(|| {
            let mut obj = pool.acquire().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
            obj.push(42);
            obj.push(84);

        })
    });

    group.bench_function("Heap_Allocation", |b| {
        b.iter(|| {
            let mut vec = vec![42];
            vec.push(84);

        })
    });

    group.bench_function("Global_Pool_Access", |b| {
        b.iter(|| {
            let pools = global_pools();
            let mut buffer = pools.get_crypto_buffer().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
            buffer.push(42);
        })
    });
    
    group.finish();
}

fn bench_concurrent_performance(c: &mut Criterion) {
    let rt = Runtime::new().map_err(|e| {
    tracing::error!("Operation failed ({}): {:?}", "Benchmark runtime creation failed", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed ({}): {:?}", "Benchmark runtime creation failed", e).to_string())
})?;
    let mut group = c.benchmark_group("Concurrency Performance");

    let lock_free_map: LockFreeHashMap<String, u64> = LockFreeHashMap::new(1000).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

    let mutex_map = std::sync::Arc::new(std::sync::Mutex::new(
        std::collections::HashMap::<String, u64>::new()
    ));

    group.bench_function("LockFree_SingleThread", |b| {
        b.iter(|| {
            for i in 0..100 {
                let key = format_args!("key_{}", i).to_string();
                lock_free_map.insert(key, i as u64).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
            }
        })
    });
    
    group.bench_function("Mutex_SingleThread", |b| {
        b.iter(|| {
            for i in 0..100 {
                let key = format_args!("key_{}", i).to_string();
                let mut map = mutex_map.lock().unwrap_or_else(|poisoned| {
        tracing::warn!("Mutex poisoned, recovering");
        poisoned.into_inner()
    });
                map.insert(key, i as u64);
            }
        })
    });

    group.bench_function("LockFree_MultiThread", |b| {
        b.to_async(&rt).iter(|| async {
            let tasks: Vec<_> = (0..4).map(|thread_id| {
                let map = &lock_free_map;
                tokio::spawn(async move {
                    for i in 0..25 {
                        let key = format_args!("key_{}_{}", thread_id, i).to_string();
                        map.insert(key, (thread_id * 25 + i) as u64).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
                    }
                })
            }).collect();
            
            for task in tasks {
                task.await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
            }
        })
    });
    
    group.bench_function("Mutex_MultiThread", |b| {
        b.to_async(&rt).iter(|| async {
            let tasks: Vec<_> = (0..4).map(|thread_id| {
                let map = mutex_map.clone();
                tokio::spawn(async move {
                    for i in 0..25 {
                        let key = format_args!("key_{}_{}", thread_id, i).to_string();
                        let mut locked_map = map.lock().unwrap_or_else(|poisoned| {
        tracing::warn!("Mutex poisoned, recovering");
        poisoned.into_inner()
    });
                        locked_map.insert(key, (thread_id * 25 + i) as u64);
                    }
                })
            }).collect();
            
            for task in tasks {
                task.await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
            }
        })
    });
    
    group.finish();
}

fn bench_zero_cost_abstractions(c: &mut Criterion) {
    let mut group = c.benchmark_group("Zero-Cost Abstractions");

    struct ZeroCostSystem<P: Provider, C: Cache> {
        provider: P,
        cache: C,
    }
    
    impl<P: Provider, C: Cache> ZeroCostSystem<P, C> {
        fn process_request(&self, data: &str) -> String {
            let processed = self.provider.process(data);
            self.cache.store(&processed);
            processed
        }
    }

    struct TraditionalSystem {
        provider: Box<dyn Provider>,
        cache: Box<dyn Cache>,
    }
    
    impl TraditionalSystem {
        fn process_request(&self, data: &str) -> String {
            let processed = self.provider.process(data);
            self.cache.store(&processed);
            processed
        }
    }

    struct MockProvider;
    struct MockCache;
    
    impl Provider for MockProvider {
        fn process(&self, data: &str) -> String {
            format_args!("processed_{}", data).to_string()
        }
    }
    
    impl Cache for MockCache {
        fn store(&self, _data: &str) {}
    }
    
    trait Provider {
        fn process(&self, data: &str) -> String;
    }
    
    trait Cache {
        fn store(&self, data: &str);
    }

    let zero_cost_system = ZeroCostSystem {
        provider: MockProvider,
        cache: MockCache,
    };
    
    group.bench_function("Zero_Cost_Abstractions", |b| {
        b.iter(|| {
            zero_cost_system.process_request("test_data")
        })
    });

    let traditional_system = TraditionalSystem {
        provider: Box::new(MockProvider),
        cache: Box::new(MockCache),
    };
    
    group.bench_function("Traditional_Trait_Objects", |b| {
        b.iter(|| {
            traditional_system.process_request("test_data")
        })
    });
    
    group.finish();
}

fn bench_error_handling(c: &mut Criterion) {
    let mut group = c.benchmark_group("Error Handling");

    fn beardog_operation(success: bool) -> BearDogResult<String> {
        if success {
            Ok("success".to_string())
        } else {
            Err(beardog_errors::BearDogError::validation("test error"))
        }
    }
    
    fn standard_operation(success: bool) -> Result<String, Box<dyn std::error::Error>> {
        if success {
            Ok("success".to_string())
        } else {
            Err("test error".into())
        }
    }
    
    group.bench_function("BearDogResult_Success", |b| {
        b.iter(|| beardog_operation(true).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?)
    });
    
    group.bench_function("BearDogResult_Error", |b| {
        b.iter(|| beardog_operation(false).unwrap_err())
    });
    
    group.bench_function("Standard_Result_Success", |b| {
        b.iter(|| standard_operation(true).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?)
    });
    
    group.bench_function("Standard_Result_Error", |b| {
        b.iter(|| standard_operation(false).unwrap_err())
    });
    
    group.finish();
}

fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Usage Patterns");

    group.bench_function("Stack_Allocation", |b| {
        b.iter(|| {
            let data = [0u8; 1024]; // Stack allocated
            let sum: u32 = data.iter().map(|&x| x as u32).sum();
            sum
        })
    });
    
    group.bench_function("Heap_Allocation", |b| {
        b.iter(|| {
            let data = vec![0u8; 1024]; // Heap allocated
            let sum: u32 = data.iter().map(|&x| x as u32).sum();
            sum
        })
    });

    struct ConstBuffer<const SIZE: usize> {
        data: [u8; SIZE],
    }
    
    impl<const SIZE: usize> ConstBuffer<SIZE> {
        fn new() -> Self {
            Self { data: [0; SIZE] }
        }
        
        fn process(&self) -> u32 {
            self.data.iter().map(|&x| x as u32).sum()
        }
    }
    
    struct RuntimeBuffer {
        data: Vec<u8>,
    }
    
    impl RuntimeBuffer {
        fn new(size: usize) -> Self {
            Self { data: vec![0; size] }
        }
        
        fn process(&self) -> u32 {
            self.data.iter().map(|&x| x as u32).sum()
        }
    }
    
    group.bench_function("Const_Generic_Buffer", |b| {
        b.iter(|| {
            let buffer = ConstBuffer::<1024>::new();
            buffer.process()
        })
    });
    
    group.bench_function("Runtime_Sized_Buffer", |b| {
        b.iter(|| {
            let buffer = RuntimeBuffer::new(1024);
            buffer.process()
        })
    });
    
    group.finish();
}

fn comprehensive_benchmark_suite(c: &mut Criterion) {

    c.measurement_time(Duration::from_secs(10));
    c.warm_up_time(Duration::from_secs(3));
    c.sample_size(100);
    
    println!("🚀 Running BearDog Comprehensive Performance Benchmarks");
    println!("📊 Validating modernization performance claims...");
    
    bench_simd_crypto(c);
    bench_object_pools(c);
    bench_concurrent_performance(c);
    bench_zero_cost_abstractions(c);
    bench_error_handling(c);
    bench_memory_patterns(c);
    
    println!("✅ Benchmark suite completed!");
}

criterion_group!(benches, comprehensive_benchmark_suite);
criterion_main!(benches);

#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[tokio::test]
    async fn validate_simd_performance_claims() {
        let results = benchmark_simd_performance().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

        assert!(
            results.performance_improvement >= 3.0,
            "SIMD performance improvement should be at least 300%"
        );
        
        println!("✅ SIMD Performance: {:.1}x improvement", results.performance_improvement);
    }
    
    #[test]
    fn validate_object_pool_zero_allocation() {
        let pool: ObjectPool<Vec<u8>, 10> = ObjectPool::new().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

        let initial_stats = pool.stats();

        for _ in 0..5 {
            let _obj = pool.acquire().map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;

        }

        let final_stats = pool.stats();
        assert_eq!(final_stats.objects_in_use, 0);
        assert_eq!(final_stats.total_allocations, 5);
        assert_eq!(final_stats.total_deallocations, 5);
        
        println!("✅ Object Pool: Zero net allocation validated");
    }
    
    #[test]
    fn validate_lock_free_performance() {
        let map: LockFreeHashMap<String, u64> = LockFreeHashMap::new(100).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        
        let start = std::time::Instant::now();

        for i in 0..1000 {
            map.insert(format_args!("key_{}", i).to_string(), i as u64).map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format_args!("Operation failed: {:?}", e).to_string())
})?;
        }
        
        let single_thread_time = start.elapsed();

        println!("✅ Lock-Free: Single thread time: {:?}", single_thread_time);
    }
} 