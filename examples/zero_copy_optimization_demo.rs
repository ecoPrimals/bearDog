

use beardog_utils::zero_copy::*;
use beardog_errors::BearDogResult;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;

#[tokio::main]
async fn main() -> BearDogResult<()> {
    println!("🚀 BearDog Zero-Copy Optimization Demo");
    println!("====================================");

    let optimizer = ZeroCopyOptimizationSystem::new().await?;

    let test_capabilities = vec![
        "communication_mesh",
        "storage_services",
        "compute_orchestration",
    ];
    
    println!("\n🔍 Testing Zero-Copy Optimization by Service Capability...");
    
    for capability in test_capabilities {
        println!("\n📊 Testing {} capability...", capability);
        
        match test_zero_copy_optimization_for_capability(&optimizer, capability).await {
            Ok(result) => {
                println!("✅ Optimization Success for {}", capability);
                println!("   Memory Savings: {:.2}%", result.memory_savings_percent);
                println!("   Performance Gain: {:.2}%", result.performance_gain_percent);
                println!("   Zero-Copy Operations: {}", result.zero_copy_operations_count);
                
                if result.memory_savings_percent > 50.0 {
                    println!("   🌟 Excellent optimization results!");
                }
            }
            Err(e) => {
                println!("❌ Optimization Failed for {}: {}", capability, e);
            }
        }
    }

    println!("\n🔗 Testing Cross-Capability Zero-Copy Coordination...");
    
    let coordination_result = optimizer
        .test_cross_capability_zero_copy_coordination()
        .await?;
        
    println!("✅ Cross-Capability Coordination Results:");
    println!("   Total Memory Saved: {} bytes", coordination_result.total_memory_saved);
    println!("   Coordinated Operations: {}", coordination_result.coordinated_operations);
    println!("   Overall Performance Gain: {:.2}%", coordination_result.overall_performance_gain);
    
    println!("\n🎉 Zero-Copy Optimization Demo Complete!");
    Ok(())
}

async fn test_zero_copy_optimization_for_capability(
    optimizer: &ZeroCopyOptimizationSystem,
    capability: &str,
) -> BearDogResult<OptimizationResult> {

    let registry = global_registry();
    let services = registry
        .discover_by_capability(capability)
        .await?;
        
    if services.is_empty() {
        return Err(BearDogError::NoServiceFound {
            capability: capability.to_string(),
        });
    }

    let test_service = services
        .iter()
        .max_by_key(|service| service.performance_metrics.overall_score)
        .ok_or_else(|| BearDogError::NoSuitableService {
            capability: capability.to_string(),
        })?;

    let adapter = UniversalAdapterFactory::create_adapter(
        PrimalId::from_id(&test_service.service_id),
        test_service.primary_endpoint.clone(),
        test_service.auth_config.clone(),
    ).await?;

    let baseline_performance = optimizer
        .measure_baseline_performance(&adapter)
        .await?;
        
    let optimized_performance = optimizer
        .measure_optimized_performance(&adapter)
        .await?;
    
    let memory_baseline = optimizer
        .measure_baseline_memory_usage(&adapter)
        .await?;
        
    let memory_optimized = optimizer
        .measure_optimized_memory_usage(&adapter)
        .await?;
    
    Ok(OptimizationResult {
        service_id: test_service.service_id.clone(),
        capability: capability.to_string(),
        memory_savings_percent: ((memory_baseline - memory_optimized) as f64 / memory_baseline as f64) * 100.0,
        performance_gain_percent: ((optimized_performance - baseline_performance) as f64 / baseline_performance as f64) * 100.0,
        zero_copy_operations_count: optimizer.count_zero_copy_operations(&adapter).await?,
        baseline_performance,
        optimized_performance,
        memory_baseline,
        memory_optimized,
    })
}

async fn demo_id_optimization() -> BearDogResult<()> {
    info!("🆔 Demo 1: ID Optimization - Eliminating ID String Cloning");

    let request_id = generate_request_id("api");
    let task_id = generate_task_id("wrk", 42);
    let node_id = generate_node_id("server", 1);

    info!("📊 Generated IDs without cloning:");
    info!("   Request ID: {}", request_id);
    info!("   Task ID: {}", task_id);
    info!("   Node ID: {}", node_id);

    let shared_req_id = shared_request_id("req_12345");
    let shared_req_id_2 = shared_request_id("req_12345");

    assert!(Arc::ptr_eq(&shared_req_id, &shared_req_id_2));
    info!("✅ Shared IDs use same Arc instance - zero cloning!");

    let id_manager = global_id_manager();
    let stats = id_manager.get_stats();
    info!("📈 ID Manager Stats:");
    info!("   Cache Hits: {}", stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed));
    info!("   Cache Misses: {}", stats.cache_misses.load(std::sync::atomic::Ordering::Relaxed));
    info!("   IDs Created: {}", stats.ids_created.load(std::sync::atomic::Ordering::Relaxed));

    Ok(())
}

async fn demo_config_sharing() -> BearDogResult<()> {
    info!("⚙️  Demo 2: Configuration Sharing - Eliminating Config Cloning");

    let api_config = shared_api_config(|| ApiConfig {
        bind_address: "0.0.0.0:8080".to_string(),
        port: 8080,
        max_request_size: 1024 * 1024,
        timeout_secs: 30,
        cors_origins: vec!["https://beardog.local".to_string()],
    });

    let database_config = shared_database_config(|| DatabaseConfig {
        host: "beardog-db".to_string(),
        port: 5432,
        database: "beardog".to_string(),
        username: "beardog".to_string(),
        max_connections: 10,
        timeout_secs: 30,
    });

    let api_config_2 = get_shared_config::<ApiConfig>("api").unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
});
    let db_config_2 = get_shared_config::<DatabaseConfig>("database").unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
});

    assert!(Arc::ptr_eq(&api_config, &api_config_2));
    assert!(Arc::ptr_eq(&database_config, &db_config_2));

    info!("📊 Shared Configurations:");
    info!("   API Config - Bind: {} Port: {}", api_config.bind_address, api_config.port);
    info!("   DB Config - Host: {} Max Connections: {}", database_config.host, database_config.max_connections);

    let config_manager = global_config_manager();
    info!("📈 Config Manager Stats:");
    info!("   Hit Rate: {:.2}%", config_manager.hit_rate() * 100.0);
    info!("   Cache Size: {}", config_manager.cache_size());
    info!("   Memory Usage: {} bytes", config_manager.estimated_memory_usage());

    Ok(())
}

async fn demo_request_caching() -> BearDogResult<()> {
    info!("📨 Demo 3: Request/Response Caching - Eliminating Request Cloning");

    let request = ApiRequest {
        request_id: "req_cached_demo".to_string(),
        method: "GET".to_string(),
        path: "/api/users".to_string(),
        headers: HashMap::with_capacity(16),
        body: None,
    };

    let cached_request = cache_request(request);
    info!("💾 Cached request: {} {}", cached_request.method, cached_request.path);

    let cache_key = RequestCacheKey::from_http_request("GET", "/api/users", None, None);
    let retrieved_request = get_cached_request(&cache_key).unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
});
    
    assert!(Arc::ptr_eq(&cached_request, &retrieved_request));
    info!("✅ Retrieved same request instance - no cloning!");

    let response = ApiResponse::new(200, b"Success".to_vec())
        .with_header("Content-Type".to_string(), "application/json".to_string());
    
    let cached_response = cache_response(cache_key.clone(), response);
    info!("💾 Cached response: Status {} Body: {} bytes", 
          cached_response.status, cached_response.body.len());

    let request_cache = global_request_cache();
    info!("📈 Request Cache Stats:");
    info!("   Hit Rate: {:.2}%", request_cache.hit_rate() * 100.0);
    info!("   Cache Size: {}", request_cache.size());

    Ok(())
}

async fn demo_performance_comparison() -> BearDogResult<()> {
    info!("⚡ Demo 4: Performance Comparison - Before vs After Optimization");

    const ITERATIONS: usize = beardog_types::constants::performance::testing::LIGHT_ITERATIONS;

    let start = std::time::Instant::now();
    let mut cloned_strings = Vec::new();
    for i in 0..ITERATIONS {
        let base_string = format_args!("request_id_{}", i).to_string();

        let cloned1 = base_string.clone(); // First clone
        let cloned2 = base_string.clone(); // Second clone  
        let cloned3 = base_string.clone(); // Third clone
        cloned_strings.push((cloned1, cloned2, cloned3));
    }
    let clone_duration = start.elapsed();

    let start = std::time::Instant::now();
    let mut shared_strings = Vec::new();
    for i in 0..ITERATIONS {
        let base_string = format_args!("request_id_{}", i).to_string();
        let shared = shared_string(&base_string);

        let ref1 = shared.clone(); // Arc clone (cheap)
        let ref2 = shared.clone(); // Arc clone (cheap)
        let ref3 = shared.clone(); // Arc clone (cheap)
        shared_strings.push((ref1, ref2, ref3));
    }
    let shared_duration = start.elapsed();

    info!("📊 Performance Comparison ({} iterations):", ITERATIONS);
    info!("   BEFORE (String cloning): {:?}", clone_duration);
    info!("   AFTER  (Arc sharing):    {:?}", shared_duration);
    info!("   Improvement: {:.2}x faster", 
          clone_duration.as_nanos() as f64 / shared_duration.as_nanos() as f64);

    let clone_memory = cloned_strings.len() * 3 * 32; // Rough estimate
    let shared_memory = shared_strings.len() * 3 * 8; // Arc pointers
    info!("📊 Memory Comparison:");
    info!("   BEFORE (cloned strings): ~{} bytes", clone_memory);
    info!("   AFTER  (shared arcs):    ~{} bytes", shared_memory);
    info!("   Memory Saved: {:.2}x less", clone_memory as f64 / shared_memory as f64);

    Ok(())
}

async fn demo_memory_optimization() -> BearDogResult<()> {
    info!("🧠 Demo 5: Memory Optimization - Overall System Benefits");

    let common_ids = vec![
        "api", "metrics", "health", "admin", "beardog", "songbird", 
        "nestgate", "squirrel", "GET", "POST", "application/json",
    ];

    let mut shared_instances = Vec::new();
    for _ in 0..1000 {
        for common_id in &common_ids {
            let shared = shared_string(common_id);
            shared_instances.push(shared);
        }
    }

    info!("📊 Created {} string instances from {} unique values", 
          shared_instances.len(), common_ids.len());

    let first_api = shared_string("api");
    let last_api = shared_string("api");
    assert!(Arc::ptr_eq(&first_api, &last_api));

    let zero_copy_manager = global_zero_copy_manager();
    let stats = zero_copy_manager.get_stats();
    
    info!("📈 Zero-Copy Manager Overall Stats:");
    info!("   Cache Hits: {}", stats.cache_hits.load(std::sync::atomic::Ordering::Relaxed));
    info!("   Cache Misses: {}", stats.cache_misses.load(std::sync::atomic::Ordering::Relaxed));
    info!("   Clones Avoided: {}", stats.clones_avoided.load(std::sync::atomic::Ordering::Relaxed));
    info!("   Memory Saved: {} bytes", stats.memory_saved.load(std::sync::atomic::Ordering::Relaxed));

    zero_copy_manager.cleanup_expired();
    info!("🧹 Performed cleanup of expired references");

    Ok(())
}

mod refactoring_examples {
    use super::*;

        struct OldGeneticAutomation {
        request_id: String,
        genetic_parameters: String,
        node_id: String,
    }

        impl OldGeneticAutomation {
        fn process_request_old(&self, request: &ApiRequest) {

            let _request_id = request.request_id.clone();
            let _method = request.method.clone();
            let _path = request.path.clone();

        }
    }

        struct NewGeneticAutomation {
        request_id: IdString,
        genetic_parameters: Arc<String>,
        node_id: Arc<str>,
    }

        impl NewGeneticAutomation {
        fn process_request_new(&self, request: &Arc<ApiRequest>) {

            let _request_id = &request.request_id;
            let _method = &request.method;
            let _path = &request.path;

            let _shared_request = request.clone(); // Just Arc clone, not data clone
        }
    }

        fn old_hsm_pattern(provider_info: &ProviderInfo) -> HashMap<String, String> {
        let mut health_results = HashMap::with_capacity(16);

        health_results.insert("provider_type".to_string(), provider_info.provider_type.clone());
        health_results.insert("description".to_string(), provider_info.description.clone());
        health_results        
    }

        fn new_hsm_pattern(provider_info: &ProviderInfo) -> HashMap<Arc<str>, Arc<str>> {
        let mut health_results = HashMap::with_capacity(16);

        health_results.insert(
            shared_string("provider_type"), 
            shared_string(&provider_info.provider_type)
        );
        health_results.insert(
            shared_string("description"), 
            shared_string(&provider_info.description)
        );
        health_results
    }

        struct ProviderInfo {
        provider_type: String,
        description: String,
    }
}

mod benchmarks {
    use super::*;
    use std::time::Instant;

        pub fn benchmark_string_operations() {
        const ITERATIONS: usize = beardog_types::constants::performance::testing::STANDARD_ITERATIONS;

        let start = Instant::now();
        let mut cloned = Vec::new();
        for i in 0..ITERATIONS {
            let s = format_args!("request_{}", i % 100).to_string(); // Simulate common IDs
            cloned.push(s.clone());
        }
        let clone_time = start.elapsed();

        let start = Instant::now();
        let mut shared = Vec::new();
        for i in 0..ITERATIONS {
            let s = format_args!("request_{}", i % 100).to_string();
            shared.push(shared_string(s));
        }
        let shared_time = start.elapsed();

        println!("String Operations Benchmark ({} iterations):", ITERATIONS);
        println!("  Clone Time:  {:?}", clone_time);
        println!("  Shared Time: {:?}", shared_time);
        println!("  Improvement: {:.2}x", 
                clone_time.as_nanos() as f64 / shared_time.as_nanos() as f64);
    }

        pub fn benchmark_config_access() {
        const ITERATIONS: usize = beardog_types::constants::performance::testing::STANDARD_ITERATIONS / 2;

        let _shared_config = shared_api_config(|| ApiConfig {
            bind_address: "0.0.0.0:8080".to_string(),
            port: 8080,
            max_request_size: 1024 * 1024,
            timeout_secs: 30,
            cors_origins: vec!["https://beardog.local".to_string()],
        });

        let start = Instant::now();
        for _ in 0..ITERATIONS {
            let _config = get_shared_config::<ApiConfig>("api").unwrap_or_else(|e| {
    tracing::error!("Unwrap failed: {:?}", e);
    return Err(std::io::Error::new(
    std::io::ErrorKind::Other,
    format_args!("Operation failed: {:?}", e).to_string()
).into())
});

        }
        let shared_access_time = start.elapsed();

        println!("Config Access Benchmark ({} iterations):", ITERATIONS);
        println!("  Shared Access Time: {:?}", shared_access_time);
        println!("  Average per access: {:?}", shared_access_time / ITERATIONS as u32);
    }
} 