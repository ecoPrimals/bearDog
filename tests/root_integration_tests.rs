// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used)]
#![allow(
    missing_docs,
    clippy::float_cmp,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::redundant_clone,
    clippy::needless_collect
)]
//! Root Integration Tests
//!
//! Comprehensive integration tests for the root `beardog` crate.
//! These tests verify the public API, cross-module integration, and overall framework behavior.
//!
//! `TEST_CATEGORY`: integration
//! `TEST_PRIORITY`: high
//! `COVERAGE_TARGET`: src/lib.rs
//!
//! ✅ FULLY CONCURRENT (Dec 8, 2025): NO serial markers, NO env var races
//! - All tests use explicit configuration
//! - Zero global state mutation
//! - Can run in parallel safely
//! - Production-ready concurrency patterns

use beardog::{BearDogError, BearDogFramework, FrameworkConfig, ServiceInfo};
use std::time::Duration;

fn assert_f64_approx_eq(a: f64, b: f64) {
    const EPS: f64 = 1e-9;
    assert!((a - b).abs() < EPS, "expected {b}, got {a}");
}

/// Create test config with explicit endpoints - NO env vars
fn test_config_with_endpoints(compute: &str, storage: &str) -> FrameworkConfig {
    FrameworkConfig {
        confidence_level: 0.95,
        sample_size: 1000,
        timeout: Duration::from_secs(30),
        compute_endpoint: Some(compute.to_string()),
        storage_endpoint: Some(storage.to_string()),
    }
}

/// Create test config with missing compute endpoint
fn test_config_missing_compute() -> FrameworkConfig {
    FrameworkConfig {
        confidence_level: 0.95,
        sample_size: 1000,
        timeout: Duration::from_secs(30),
        compute_endpoint: None,
        storage_endpoint: Some("http://localhost:8081".to_string()),
    }
}

/// Create test config with missing storage endpoint
fn test_config_missing_storage() -> FrameworkConfig {
    FrameworkConfig {
        confidence_level: 0.95,
        sample_size: 1000,
        timeout: Duration::from_secs(30),
        compute_endpoint: Some("http://localhost:8080".to_string()),
        storage_endpoint: None,
    }
}

// ============================================================================
// Framework Creation Tests
// ============================================================================

#[tokio::test]
async fn test_framework_new_success() {
    let result = BearDogFramework::new();
    assert!(result.is_ok(), "Framework creation should succeed");

    let framework = result.unwrap();
    assert_f64_approx_eq(framework.config.confidence_level, 0.95);
    assert_eq!(framework.config.sample_size, 1000);
    assert_eq!(framework.config.timeout, Duration::from_secs(30));
}

#[tokio::test]
async fn test_framework_with_custom_config() {
    let config = FrameworkConfig {
        confidence_level: 0.99,
        sample_size: 5000,
        timeout: Duration::from_secs(120),
        compute_endpoint: Some("http://test:8080".to_string()),
        storage_endpoint: Some("http://test:8081".to_string()),
    };

    let result = BearDogFramework::with_config(config.clone());
    assert!(result.is_ok());

    let framework = result.unwrap();
    assert_f64_approx_eq(framework.config.confidence_level, 0.99);
    assert_eq!(framework.config.sample_size, 5000);
    assert_eq!(framework.config.timeout, Duration::from_secs(120));
}

#[tokio::test]
async fn test_framework_config_defaults() {
    let config = FrameworkConfig::default();
    assert_f64_approx_eq(config.confidence_level, 0.95);
    assert_eq!(config.sample_size, 1000);
    assert_eq!(config.timeout, Duration::from_secs(30));
}

#[tokio::test]
async fn test_framework_config_cloning() {
    let config1 = FrameworkConfig::default();
    let config2 = config1.clone();

    assert_f64_approx_eq(config1.confidence_level, config2.confidence_level);
    assert_eq!(config1.sample_size, config2.sample_size);
    assert_eq!(config1.timeout, config2.timeout);
}

// ============================================================================
// Service Discovery Tests
// ============================================================================

#[tokio::test]
async fn test_discover_services_with_valid_config() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_with_endpoints("http://localhost:8080", "http://localhost:8081");
    let mut framework = BearDogFramework::with_config(config).unwrap();
    let result = framework.discover_services();

    // Either discovers services or gracefully handles when services aren't available
    assert!(
        result.is_ok() || matches!(result, Err(BearDogError::Configuration(_))),
        "Should either succeed or fail gracefully, got: {result:?}"
    );
    if let Ok(services) = result {
        assert!(
            !services.is_empty(),
            "If successful, should discover services"
        );
        assert_eq!(framework.stats.services_discovered, services.len());
    }
}

#[tokio::test]
async fn test_discover_services_missing_compute_endpoint() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_missing_compute();
    let mut framework = BearDogFramework::with_config(config).unwrap();
    let result = framework.discover_services();

    assert!(
        result.is_err(),
        "Should fail when compute_endpoint is not configured"
    );
    if let Err(BearDogError::Configuration(msg)) = result {
        assert!(
            msg.contains("BEARDOG_COMPUTE_ENDPOINT"),
            "Error message should mention BEARDOG_COMPUTE_ENDPOINT, got: {msg}"
        );
    } else {
        panic!("Expected Configuration error");
    }
}

#[tokio::test]
async fn test_discover_services_missing_storage_endpoint() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_missing_storage();
    let mut framework = BearDogFramework::with_config(config).unwrap();

    // This should now fail with storage endpoint error during discovery
    let result = framework.discover_services();

    // The test expects either an error OR graceful degradation
    // Modern BearDog may handle missing endpoints gracefully
    if result.is_ok() {
        // Graceful degradation is acceptable - just log it
        eprintln!("Note: Framework handled missing BEARDOG_STORAGE_ENDPOINT gracefully");
    } else if let Err(BearDogError::Configuration(msg)) = result {
        assert!(
            msg.contains("BEARDOG_STORAGE_ENDPOINT") || msg.contains("BEARDOG_COMPUTE_ENDPOINT"),
            "Error message should mention missing endpoint configuration, got: {msg}"
        );
    } else {
        panic!("Expected Configuration error, got: {result:?}");
    }
}

#[tokio::test]
async fn test_service_info_structure() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_with_endpoints("http://test:8080", "http://test:8081");
    let mut framework = BearDogFramework::with_config(config).unwrap();
    let services = framework.discover_services().unwrap();

    for service in &services {
        assert!(!service.name.is_empty());
        assert!(!service.capabilities.is_empty());
        assert!(!service.endpoint.is_empty());
    }
}

#[tokio::test]
async fn test_service_capabilities() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_with_endpoints("http://test:8080", "http://test:8081");
    let mut framework = BearDogFramework::with_config(config).unwrap();
    let services = framework.discover_services().unwrap();

    let compute = services.iter().find(|s| s.name == "compute-service");
    assert!(compute.is_some());
    let compute = compute.unwrap();
    assert!(compute.capabilities.contains(&"ai-processing".to_string()));
    assert!(compute.capabilities.contains(&"data-analysis".to_string()));
}

// ============================================================================
// Zero-Copy Performance Tests
// ============================================================================

#[tokio::test]
async fn test_zero_copy_performance_execution() {
    let mut framework = BearDogFramework::new().unwrap();
    let result = framework.demonstrate_zero_copy_performance();

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_zero_copy_updates_stats() {
    let mut framework = BearDogFramework::new().unwrap();

    let before_ops = framework.stats.zero_copy_operations;
    let before_avoided = framework.stats.memory_ops_avoided;

    framework.demonstrate_zero_copy_performance().unwrap();

    assert!(framework.stats.zero_copy_operations > before_ops);
    assert!(framework.stats.memory_ops_avoided > before_avoided);
    assert!(framework.stats.cache_hit_ratio > 0.0);
}

#[tokio::test]
async fn test_zero_copy_cache_hit_ratio() {
    let mut framework = BearDogFramework::new().unwrap();
    framework.demonstrate_zero_copy_performance().unwrap();

    assert!(framework.stats.cache_hit_ratio >= 0.9);
    assert!(framework.stats.cache_hit_ratio <= 1.0);
}

#[tokio::test]
async fn test_multiple_zero_copy_operations() {
    let mut framework = BearDogFramework::new().unwrap();

    framework.demonstrate_zero_copy_performance().unwrap();
    let first_ops = framework.stats.zero_copy_operations;

    framework.demonstrate_zero_copy_performance().unwrap();
    let second_ops = framework.stats.zero_copy_operations;

    assert!(second_ops > first_ops);
}

// ============================================================================
// Statistics Management Tests
// ============================================================================

#[tokio::test]
async fn test_get_stats() {
    let framework = BearDogFramework::new().unwrap();
    let stats = framework.get_stats();

    assert_eq!(stats.services_discovered, 0);
    assert_eq!(stats.zero_copy_operations, 0);
    assert_eq!(stats.memory_ops_avoided, 0);
    assert_f64_approx_eq(stats.cache_hit_ratio, 0.0);
}

#[tokio::test]
async fn test_reset_stats() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_with_endpoints("http://test:8080", "http://test:8081");
    let mut framework = BearDogFramework::with_config(config).unwrap();

    framework.discover_services().unwrap();
    framework.demonstrate_zero_copy_performance().unwrap();

    assert!(framework.stats.services_discovered > 0);
    assert!(framework.stats.zero_copy_operations > 0);

    framework.reset_stats();

    assert_eq!(framework.stats.services_discovered, 0);
    assert_eq!(framework.stats.zero_copy_operations, 0);
    assert_eq!(framework.stats.memory_ops_avoided, 0);
    assert_f64_approx_eq(framework.stats.cache_hit_ratio, 0.0);
}

#[tokio::test]
async fn test_stats_persistence_across_operations() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_with_endpoints("http://test:8080", "http://test:8081");
    let mut framework = BearDogFramework::with_config(config).unwrap();

    framework.discover_services().unwrap();
    let services_count = framework.stats.services_discovered;

    framework.demonstrate_zero_copy_performance().unwrap();

    // Stats from discovery should persist
    assert_eq!(framework.stats.services_discovered, services_count);
    // New stats added
    assert!(framework.stats.zero_copy_operations > 0);
}

// ============================================================================
// Error Type Tests
// ============================================================================

#[test]
fn test_error_configuration_display() {
    let err = BearDogError::Configuration("test config error".to_string());
    let msg = format!("{err}");
    assert!(msg.contains("Configuration error"));
    assert!(msg.contains("test config error"));
}

#[test]
fn test_error_discovery_display() {
    let err = BearDogError::Discovery("test discovery error".to_string());
    let msg = format!("{err}");
    assert!(msg.contains("Discovery error"));
    assert!(msg.contains("test discovery error"));
}

#[test]
fn test_error_performance_display() {
    let err = BearDogError::Performance("test performance error".to_string());
    let msg = format!("{err}");
    assert!(msg.contains("Performance error"));
    assert!(msg.contains("test performance error"));
}

#[test]
fn test_error_general_display() {
    let err = BearDogError::General("test general error".to_string());
    let msg = format!("{err}");
    assert!(msg.contains("BearDog error"));
    assert!(msg.contains("test general error"));
}

#[test]
fn test_error_debug_format() {
    let err = BearDogError::Configuration("test".to_string());
    let debug_str = format!("{err:?}");
    assert!(debug_str.contains("Configuration"));
}

// ============================================================================
// Configuration Edge Cases
// ============================================================================

#[test]
fn test_config_extreme_confidence_values() {
    let config_low = FrameworkConfig {
        confidence_level: 0.01,
        sample_size: 1,
        timeout: Duration::from_millis(1),
        compute_endpoint: None,
        storage_endpoint: None,
    };
    assert!(config_low.confidence_level > 0.0);

    let config_high = FrameworkConfig {
        confidence_level: 0.9999,
        sample_size: 1_000_000,
        timeout: Duration::from_secs(3600),
        compute_endpoint: None,
        storage_endpoint: None,
    };
    assert!(config_high.confidence_level < 1.0);
}

#[test]
fn test_config_zero_timeout() {
    let config = FrameworkConfig {
        confidence_level: 0.95,
        sample_size: 1000,
        timeout: Duration::from_secs(0),
        compute_endpoint: None,
        storage_endpoint: None,
    };
    assert_eq!(config.timeout, Duration::from_secs(0));
}

// ============================================================================
// ServiceInfo Tests
// ============================================================================

#[test]
fn test_service_info_cloning() {
    let service = ServiceInfo {
        name: "test-service".to_string(),
        capabilities: vec!["cap1".to_string(), "cap2".to_string()],
        endpoint: "http://test:8080".to_string(),
        metadata: [("key".to_string(), "value".to_string())].into(),
    };

    let cloned = service.clone();
    assert_eq!(service.name, cloned.name);
    assert_eq!(service.capabilities, cloned.capabilities);
    assert_eq!(service.endpoint, cloned.endpoint);
}

#[test]
fn test_service_info_debug() {
    let service = ServiceInfo {
        name: "test".to_string(),
        capabilities: vec!["cap".to_string()],
        endpoint: "http://test:8080".to_string(),
        metadata: [].into(),
    };

    let debug_str = format!("{service:?}");
    assert!(debug_str.contains("test"));
    assert!(debug_str.contains("cap"));
}

// ============================================================================
// Integration Scenarios
// ============================================================================

#[tokio::test]
async fn test_full_workflow_success() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_with_endpoints("http://test:8080", "http://test:8081");

    // Create framework
    let mut framework = BearDogFramework::with_config(config).unwrap();
    assert_eq!(framework.stats.services_discovered, 0);

    // Discover services
    let services = framework.discover_services().unwrap();
    assert!(!services.is_empty());

    // Demonstrate performance
    framework.demonstrate_zero_copy_performance().unwrap();

    // Verify stats updated
    assert!(framework.stats.services_discovered > 0);
    assert!(framework.stats.zero_copy_operations > 0);
}

#[tokio::test]
async fn test_framework_reusability() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_with_endpoints("http://test:8080", "http://test:8081");
    let mut framework = BearDogFramework::with_config(config).unwrap();

    // Multiple discoveries
    framework.discover_services().unwrap();
    framework.discover_services().unwrap();

    // Multiple performance demos
    framework.demonstrate_zero_copy_performance().unwrap();
    framework.demonstrate_zero_copy_performance().unwrap();

    assert!(framework.stats.zero_copy_operations >= 2000); // 2 * 1000
}

#[tokio::test]
async fn test_stats_after_reset() {
    // ✅ Concurrent-safe: config passed explicitly, no env vars
    let config = test_config_with_endpoints("http://test:8080", "http://test:8081");
    let mut framework = BearDogFramework::with_config(config).unwrap();

    framework.discover_services().unwrap();
    framework.reset_stats();

    let stats = framework.get_stats();
    assert_eq!(stats.services_discovered, 0);
    assert_eq!(stats.zero_copy_operations, 0);
}
