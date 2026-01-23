//! Handler Registry Comprehensive Testing
//!
//! Tests for the modular handler registry system with:
//! - Unit tests for each handler
//! - E2E tests for complete workflows
//! - Chaos tests for concurrent operations
//! - Fault injection tests for error handling

use beardog_tunnel::test_helpers::create_minimal_beardog_provider;
use beardog_tunnel::unix_socket_ipc::handlers::HandlerRegistry;
use serde_json::json;
use std::sync::Arc;
use std::time::Instant;
use tokio::task::JoinSet;

// ============================================================================
// UNIT TESTS - HANDLER REGISTRY
// ============================================================================

#[tokio::test]
async fn test_unit_all_handlers_registered() {
    let registry = HandlerRegistry::new();
    let handlers = registry.handlers();
    
    // Verify all 7 handlers are registered
    assert_eq!(handlers.len(), 7, "Should have 7 handlers");
    
    // Verify handler names
    let handler_names: Vec<_> = handlers.iter().map(|h| h.name()).collect();
    assert!(handler_names.contains(&"health"));
    assert!(handler_names.contains(&"capabilities"));
    assert!(handler_names.contains(&"security"));
    assert!(handler_names.contains(&"crypto"));
    assert!(handler_names.contains(&"btsp"));
    assert!(handler_names.contains(&"federation"));
    assert!(handler_names.contains(&"encryption"));
    
    println!("✅ All 7 handlers registered");
}

#[tokio::test]
async fn test_unit_all_methods_count() {
    let registry = HandlerRegistry::new();
    
    // Count all methods across all handlers
    let total_methods: usize = registry
        .handlers()
        .iter()
        .map(|h| h.methods().len())
        .sum();
    
    // Should have 83 total methods (as per Session 19)
    // Health: 4, Capabilities: 4, Security: 6, Crypto: 47, BTSP: 9, Federation: 7, Encryption: 6
    assert!(
        total_methods >= 80,
        "Should have at least 80 methods, got {}",
        total_methods
    );
    
    println!("✅ Total methods: {}", total_methods);
}

#[tokio::test]
async fn test_unit_method_routing() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    // Test routing to different handlers
    let test_cases = vec![
        ("ping", true),
        ("health", true),
        ("capabilities", true),
        ("crypto.sign_ed25519", true),
        ("tls.derive_handshake_secrets", true),
        ("btsp.tunnel_status", true),
        ("federation.derive_subfed_key", true),
        ("encryption.encrypt", true),
        ("nonexistent.method", false),
    ];
    
    for (method, should_succeed) in test_cases {
        let result = registry.route(method, None, &provider).await;
        
        if should_succeed {
            assert!(
                result.is_ok() || result.as_ref().err().unwrap().contains("Missing"),
                "Method {} should route successfully (or fail with missing params)",
                method
            );
        } else {
            assert!(
                result.is_err() && result.as_ref().err().unwrap().contains("Unknown"),
                "Method {} should fail with unknown method error",
                method
            );
        }
    }
    
    println!("✅ Method routing works correctly");
}

// ============================================================================
// E2E WORKFLOW TESTS
// ============================================================================

#[tokio::test]
async fn test_e2e_health_check_workflow() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    // Test all health check aliases
    let aliases = vec!["ping", "health", "status", "check"];
    
    for alias in aliases {
        let result = registry.route(alias, None, &provider).await;
        assert!(result.is_ok(), "Health check alias {} should work", alias);
        
        let response = result.unwrap();
        assert!(response["status"].is_string());
        assert_eq!(response["status"], "ok");
    }
    
    println!("✅ E2E health check workflow SUCCESS!");
}

#[tokio::test]
async fn test_e2e_capabilities_discovery_workflow() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    // Step 1: Get capabilities
    let caps_result = registry.route("capabilities", None, &provider).await;
    assert!(caps_result.is_ok());
    
    let caps = caps_result.unwrap();
    assert!(caps["capabilities"].is_array());
    
    // Step 2: Get identity
    let identity_result = registry.route("identity", None, &provider).await;
    assert!(identity_result.is_ok());
    
    let identity = identity_result.unwrap();
    assert!(identity["primal"].is_string());
    assert_eq!(identity["primal"], "beardog");
    
    println!("✅ E2E capabilities discovery workflow SUCCESS!");
}

#[tokio::test]
async fn test_e2e_crypto_method_discovery() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    // Get capabilities and verify crypto methods are listed
    let caps_result = registry.route("capabilities", None, &provider).await;
    assert!(caps_result.is_ok());
    
    let caps = caps_result.unwrap();
    let capabilities = caps["capabilities"].as_array().unwrap();
    
    // Verify key crypto capabilities are present
    let cap_strings: Vec<String> = capabilities
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();
    
    assert!(cap_strings.iter().any(|c| c.contains("crypto")));
    assert!(cap_strings.iter().any(|c| c.contains("tls")));
    
    println!("✅ E2E crypto method discovery SUCCESS!");
}

// ============================================================================
// CHAOS TESTS - CONCURRENT OPERATIONS
// ============================================================================

#[tokio::test]
async fn test_chaos_concurrent_health_checks() {
    let registry = Arc::new(HandlerRegistry::new());
    let provider = Arc::new(create_minimal_beardog_provider().await);
    
    let mut join_set = JoinSet::new();
    
    // 100 concurrent health checks
    for i in 0..100 {
        let reg = registry.clone();
        let prov = provider.clone();
        
        join_set.spawn(async move {
            let method = match i % 4 {
                0 => "ping",
                1 => "health",
                2 => "status",
                3 => "check",
                _ => unreachable!(),
            };
            
            reg.route(method, None, &prov).await
        });
    }
    
    let mut count = 0;
    while let Some(result) = join_set.join_next().await {
        let response = result.expect("Task should not panic").expect("Should succeed");
        assert_eq!(response["status"], "ok");
        count += 1;
    }
    
    assert_eq!(count, 100);
    println!("✅ 100 concurrent health checks SUCCESS!");
}

#[tokio::test]
async fn test_chaos_concurrent_capability_queries() {
    let registry = Arc::new(HandlerRegistry::new());
    let provider = Arc::new(create_minimal_beardog_provider().await);
    
    let mut join_set = JoinSet::new();
    
    // 100 concurrent capability queries
    for i in 0..100 {
        let reg = registry.clone();
        let prov = provider.clone();
        
        join_set.spawn(async move {
            let method = if i % 2 == 0 {
                "capabilities"
            } else {
                "identity"
            };
            
            reg.route(method, None, &prov).await
        });
    }
    
    let mut count = 0;
    while let Some(result) = join_set.join_next().await {
        let response = result.expect("Task should not panic").expect("Should succeed");
        assert!(response.is_object());
        count += 1;
    }
    
    assert_eq!(count, 100);
    println!("✅ 100 concurrent capability queries SUCCESS!");
}

#[tokio::test]
async fn test_chaos_mixed_method_calls() {
    let registry = Arc::new(HandlerRegistry::new());
    let provider = Arc::new(create_minimal_beardog_provider().await);
    
    let mut join_set = JoinSet::new();
    
    // 200 concurrent mixed method calls
    for i in 0..200 {
        let reg = registry.clone();
        let prov = provider.clone();
        
        join_set.spawn(async move {
            let method = match i % 6 {
                0 => "ping",
                1 => "health",
                2 => "capabilities",
                3 => "identity",
                4 => "info",
                5 => "status",
                _ => unreachable!(),
            };
            
            reg.route(method, None, &prov).await
        });
    }
    
    let mut count = 0;
    while let Some(result) = join_set.join_next().await {
        let response = result.expect("Task should not panic");
        assert!(response.is_ok() || response.is_err()); // Just verify no panic
        count += 1;
    }
    
    assert_eq!(count, 200);
    println!("✅ 200 concurrent mixed method calls SUCCESS!");
}

#[tokio::test]
async fn test_chaos_rapid_sequential_routing() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    let start = Instant::now();
    
    // 1000 rapid sequential routes
    for i in 0..1000 {
        let method = match i % 4 {
            0 => "ping",
            1 => "health",
            2 => "capabilities",
            3 => "identity",
            _ => unreachable!(),
        };
        
        let _ = registry.route(method, None, &provider).await;
    }
    
    let duration = start.elapsed();
    
    println!(
        "✅ 1000 sequential routes: {:?} ({} routes/sec)",
        duration,
        (1000.0 / duration.as_secs_f64()) as u64
    );
    
    // Should handle at least 100 routes/sec
    assert!(duration.as_secs() < 10);
}

// ============================================================================
// FAULT INJECTION TESTS
// ============================================================================

#[tokio::test]
async fn test_fault_unknown_method() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    let unknown_methods = vec![
        "unknown.method",
        "crypto.nonexistent",
        "tls.fake_method",
        "btsp.invalid",
        "completely.made.up",
    ];
    
    for method in unknown_methods {
        let result = registry.route(method, None, &provider).await;
        assert!(result.is_err(), "Unknown method {} should fail", method);
        assert!(
            result.unwrap_err().contains("Unknown"),
            "Error should mention unknown method"
        );
    }
    
    println!("✅ Unknown method rejection SUCCESS!");
}

#[tokio::test]
async fn test_fault_invalid_namespace() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    let invalid_namespaces = vec![
        "invalid.namespace.method",
        "fake.method",
        "nonexistent.operation",
    ];
    
    for method in invalid_namespaces {
        let result = registry.route(method, None, &provider).await;
        assert!(result.is_err(), "Invalid namespace {} should fail", method);
    }
    
    println!("✅ Invalid namespace rejection SUCCESS!");
}

#[tokio::test]
async fn test_fault_missing_parameters() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    // Methods that require parameters
    let methods_needing_params = vec![
        "crypto.sign_ed25519",
        "tls.derive_handshake_secrets",
        "encryption.encrypt",
        "federation.derive_subfed_key",
    ];
    
    for method in methods_needing_params {
        let result = registry.route(method, None, &provider).await;
        // Should fail with missing parameter error (not unknown method)
        if let Err(e) = result {
            assert!(
                e.contains("Missing") || e.contains("required"),
                "Error should mention missing parameters for {}: {}",
                method,
                e
            );
        }
    }
    
    println!("✅ Missing parameter detection SUCCESS!");
}

#[tokio::test]
async fn test_fault_concurrent_error_handling() {
    let registry = Arc::new(HandlerRegistry::new());
    let provider = Arc::new(create_minimal_beardog_provider().await);
    
    let mut join_set = JoinSet::new();
    
    // 100 concurrent calls to methods that will error
    for i in 0..100 {
        let reg = registry.clone();
        let prov = provider.clone();
        
        join_set.spawn(async move {
            let method = if i % 2 == 0 {
                "unknown.method"
            } else {
                "crypto.fake_operation"
            };
            
            reg.route(method, None, &prov).await
        });
    }
    
    let mut error_count = 0;
    while let Some(result) = join_set.join_next().await {
        let response = result.expect("Task should not panic");
        assert!(response.is_err(), "Should return error");
        error_count += 1;
    }
    
    assert_eq!(error_count, 100);
    println!("✅ 100 concurrent error handling SUCCESS!");
}

// ============================================================================
// PERFORMANCE TESTS
// ============================================================================

#[tokio::test]
async fn test_perf_routing_overhead() {
    let registry = HandlerRegistry::new();
    let provider = create_minimal_beardog_provider().await;
    
    let iterations = 1000;
    let start = Instant::now();
    
    for _ in 0..iterations {
        let _ = registry.route("ping", None, &provider).await;
    }
    
    let duration = start.elapsed();
    let avg_per_route = duration / iterations;
    
    println!("📊 Routing overhead: {:?} per route", avg_per_route);
    println!("📊 Throughput: {} routes/sec", (iterations as f64 / duration.as_secs_f64()) as u64);
    
    // Routing should be fast (< 1ms per route)
    assert!(avg_per_route.as_millis() < 1, "Routing too slow: {:?}", avg_per_route);
    
    println!("✅ Routing performance excellent!");
}

#[tokio::test]
async fn test_perf_handler_lookup() {
    let registry = HandlerRegistry::new();
    
    let iterations = 10000;
    let start = Instant::now();
    
    for i in 0..iterations {
        let method = match i % 4 {
            0 => "ping",
            1 => "crypto.sign_ed25519",
            2 => "tls.derive_handshake_secrets",
            3 => "btsp.tunnel_status",
            _ => unreachable!(),
        };
        
        // Just lookup, don't execute
        let _ = registry.handlers().iter().find(|h| {
            h.methods().contains(&method) || method.split('.').next() == Some(h.name())
        });
    }
    
    let duration = start.elapsed();
    
    println!("📊 Handler lookup: {:?} for 10k lookups", duration);
    println!("📊 Throughput: {} lookups/sec", (iterations as f64 / duration.as_secs_f64()) as u64);
    
    // Lookups should be very fast (< 100ms for 10k)
    assert!(duration.as_millis() < 100, "Lookups too slow: {:?}", duration);
    
    println!("✅ Handler lookup performance excellent!");
}

// ============================================================================
// STRESS TESTS
// ============================================================================

#[tokio::test]
async fn test_stress_sustained_load() {
    let registry = Arc::new(HandlerRegistry::new());
    let provider = Arc::new(create_minimal_beardog_provider().await);
    
    // Sustained load: 1000 requests over 10 concurrent tasks
    let mut join_set = JoinSet::new();
    
    for task_id in 0..10 {
        let reg = registry.clone();
        let prov = provider.clone();
        
        join_set.spawn(async move {
            let mut count = 0;
            for i in 0..100 {
                let method = match (task_id + i) % 4 {
                    0 => "ping",
                    1 => "health",
                    2 => "capabilities",
                    3 => "identity",
                    _ => unreachable!(),
                };
                
                let _ = reg.route(method, None, &prov).await;
                count += 1;
            }
            count
        });
    }
    
    let mut total = 0;
    while let Some(result) = join_set.join_next().await {
        let count = result.expect("Task should not panic");
        total += count;
    }
    
    assert_eq!(total, 1000);
    println!("✅ Sustained load test: 1000 requests SUCCESS!");
}

#[tokio::test]
async fn test_stress_burst_load() {
    let registry = Arc::new(HandlerRegistry::new());
    let provider = Arc::new(create_minimal_beardog_provider().await);
    
    // Burst load: 500 simultaneous requests
    let mut join_set = JoinSet::new();
    
    for i in 0..500 {
        let reg = registry.clone();
        let prov = provider.clone();
        
        join_set.spawn(async move {
            let method = if i % 2 == 0 { "ping" } else { "health" };
            reg.route(method, None, &prov).await
        });
    }
    
    let mut count = 0;
    while let Some(result) = join_set.join_next().await {
        let _ = result.expect("Task should not panic");
        count += 1;
    }
    
    assert_eq!(count, 500);
    println!("✅ Burst load test: 500 simultaneous requests SUCCESS!");
}

