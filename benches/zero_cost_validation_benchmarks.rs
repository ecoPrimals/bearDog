

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

use beardog_adapters::adapters::universal::{
    UniversalPrimalRegistry, 
    DefaultPrimalProvider,
    PrimalId
};

fn benchmark_provider_registry(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("zero_cost_provider_registration", |b| {
        b.to_async(&rt).iter(|| async {
            let registry = UniversalPrimalRegistry::new();

            let provider = Arc::new(DefaultPrimalProvider {
                ecosystem_id: "benchmark-ecosystem".to_string(),
                instance_id: "benchmark-instance".to_string(),
            });

            let result = registry.register_provider(black_box(provider)).await;
            black_box(result)
        })
    });
    
    c.bench_function("zero_cost_provider_retrieval", |b| {
        b.to_async(&rt).iter(|| async {
            let registry = UniversalPrimalRegistry::new();

            let provider = Arc::new(DefaultPrimalProvider {
                ecosystem_id: "benchmark-ecosystem".to_string(),
                instance_id: "benchmark-instance".to_string(),
            });
            registry.register_provider(provider).await.unwrap();

            let result = registry.get_provider(
                black_box("benchmark-ecosystem"),
                black_box("benchmark-instance")
            ).await;
            black_box(result)
        })
    });
    
    c.bench_function("zero_cost_primal_registration", |b| {
        b.to_async(&rt).iter(|| async {
            let registry = UniversalPrimalRegistry::new();

            let primal_id = PrimalId::new(
                "benchmark-primal",
                "Benchmark Primal", 
                "1.0.0"
            );
            
            let result = registry.register_primal(
                black_box(primal_id),
                black_box("https://benchmark.example.com".to_string()),
                black_box(HashMap::with_capacity(16))
            ).await;
            black_box(result)
        })
    });
}

fn benchmark_async_patterns(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("native_async_trait_calls", |b| {
        b.to_async(&rt).iter(|| async {

            let registry = UniversalPrimalRegistry::new();

            let primal_id = PrimalId::beardog();
            let _ = registry.register_primal(
                primal_id.clone(),
                "https://beardog.example.com".to_string(),
                HashMap::with_capacity(16)
            ).await;
            
            let discovered = registry.discover_primal(&primal_id.id).await;
            black_box(discovered)
        })
    });
    
    c.bench_function("batch_async_operations", |b| {
        b.to_async(&rt).iter(|| async {
            let registry = UniversalPrimalRegistry::new();

            let mut tasks = Vec::new();
            
            for i in 0..100 {
                let primal_id = PrimalId::new(
                    format_args!("primal-{}", i).to_string(),
                    format_args!("Primal {}", i).to_string(),
                    "1.0.0"
                );
                
                let task = registry.register_primal(
                    primal_id,
                    format_args!("https://primal-{}.example.com", i).to_string(),
                    HashMap::with_capacity(16)
                );
                tasks.push(task);
            }

            let results = futures::future::join_all(tasks).await;
            black_box(results)
        })
    });
}

fn benchmark_type_resolution(c: &mut Criterion) {
    c.bench_function("compile_time_type_resolution", |b| {
        b.iter(|| {

            let registry = UniversalPrimalRegistry::<DefaultPrimalProvider>::new();

            let provider = DefaultPrimalProvider {
                ecosystem_id: "compile-time-ecosystem".to_string(),
                instance_id: "compile-time-instance".to_string(),
            };

            let ecosystem_id = provider.ecosystem_id();
            let instance_id = provider.instance_id();
            
            black_box((ecosystem_id, instance_id))
        })
    });
    
    c.bench_function("generic_specialization", |b| {
        b.iter(|| {

            let registry1 = UniversalPrimalRegistry::<DefaultPrimalProvider>::new();
            let registry2 = UniversalPrimalRegistry::<DefaultPrimalProvider>::new();

            black_box((registry1, registry2))
        })
    });
}

fn benchmark_memory_patterns(c: &mut Criterion) {
    c.bench_function("zero_allocation_patterns", |b| {
        b.iter(|| {

            let primal_id = PrimalId::beardog(); // Stack allocated
            let ecosystem_id = primal_id.ecosystem_id(); // No allocation
            let instance_id = primal_id.instance_id(); // No allocation
            
            black_box((ecosystem_id, instance_id))
        })
    });
    
    c.bench_function("efficient_data_structures", |b| {
        b.iter(|| {

            let mut metadata = HashMap::with_capacity(16);
            metadata.insert("performance".to_string(), "optimized".to_string());
            metadata.insert("allocation".to_string(), "minimal".to_string());
            
            let primal_id = PrimalId::new(
                "efficient-primal",
                "Efficient Primal",
                "1.0.0"
            );
            
            black_box((primal_id, metadata))
        })
    });
}

fn benchmark_integration_workflow(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    
    c.bench_function("full_ecosystem_workflow", |b| {
        b.to_async(&rt).iter(|| async {

            let registry = UniversalPrimalRegistry::new();

            let beardog = PrimalId::beardog();
            let songbird = PrimalId::songbird();
            let nestgate = PrimalId::nestgate();
            
            registry.register_primal(
                beardog.clone(),
                "https://beardog.ecoprimals.com".to_string(),
                HashMap::with_capacity(16)
            ).await.unwrap();
            
            registry.register_primal(
                songbird.clone(),
                "https://songbird.ecoprimals.com".to_string(),
                HashMap::with_capacity(16)
            ).await.unwrap();
            
            registry.register_primal(
                nestgate.clone(),
                "https://nestgate.ecoprimals.com".to_string(),
                HashMap::with_capacity(16)
            ).await.unwrap();

            let discovered_beardog = registry.discover_primal(&beardog.id).await.unwrap();
            let discovered_songbird = registry.discover_primal(&songbird.id).await.unwrap();
            let discovered_nestgate = registry.discover_primal(&nestgate.id).await.unwrap();

            let all_primals = registry.list_primals().await.unwrap();
            
            black_box((discovered_beardog, discovered_songbird, discovered_nestgate, all_primals))
        })
    });
}

criterion_group!(
    benches,
    benchmark_provider_registry,
    benchmark_async_patterns,
    benchmark_type_resolution,
    benchmark_memory_patterns,
    benchmark_integration_workflow
);

criterion_main!(benches); 