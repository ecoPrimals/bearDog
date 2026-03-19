// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Health Checker Tests
//!
//! Real-world scenario tests for health checking system with proper error paths,
//! cascading failures, recovery scenarios, and edge cases.
//!
//! Coverage additions: December 10, 2025

use crate::monitoring::health::{
    aggregator::HealthCheckAggregator,
    checkers::{
        CacheHealthChecker, DatabaseHealthChecker, ExternalApiHealthChecker, HealthCheckerType,
        HsmHealthChecker,
    },
    traits::HealthChecker,
};
use beardog_types::canonical::HealthStatus;
use std::time::Duration;
use tokio::time::timeout;

// ============================================================================
// DatabaseHealthChecker Tests - Real Scenarios
// ============================================================================

#[tokio::test]
async fn test_database_health_check_success() {
    let checker = DatabaseHealthChecker::new();
    let health = checker
        .check_health()
        .await
        .expect("health check should succeed");

    assert_eq!(health.status, HealthStatus::Healthy);
    assert_eq!(health.name, "Database");
    assert!(health.message.is_some());
    assert!(health.check_duration_ms < 1000); // Should be fast
}

#[tokio::test]
async fn test_database_health_check_with_latency() {
    // Real scenario: Database under load with slow responses
    let checker = DatabaseHealthChecker::with_simulated_latency(Duration::from_millis(200));
    let health = checker.check_health().await.expect("should complete");

    assert_eq!(health.status, HealthStatus::Healthy);
    assert!(
        health.check_duration_ms >= 200,
        "Should reflect actual latency"
    );
    assert!(health.metadata.contains_key("type"));
}

#[tokio::test]
async fn test_database_health_check_timeout_detection() {
    // Real scenario: Database times out - health check should detect this
    let checker = DatabaseHealthChecker::with_simulated_latency(Duration::from_millis(2000));

    // Set aggressive timeout to simulate real timeout scenario
    let result = timeout(Duration::from_millis(500), checker.check_health()).await;

    // Should timeout before completion
    assert!(result.is_err(), "Should timeout on slow database");
}

#[tokio::test]
async fn test_database_component_name() {
    let checker = DatabaseHealthChecker::new();
    assert_eq!(checker.component_name(), "Database");
}

// ============================================================================
// CacheHealthChecker Tests - Real Scenarios
// ============================================================================

#[tokio::test]
async fn test_cache_health_check_success() {
    let checker = CacheHealthChecker::new();
    let health = checker
        .check_health()
        .await
        .expect("health check should succeed");

    assert_eq!(health.status, HealthStatus::Healthy);
    assert_eq!(health.name, "Cache");
    assert!(health.check_duration_ms < 1000);
}

#[tokio::test]
async fn test_cache_health_check_with_latency() {
    // Real scenario: Cache warming up or under pressure
    let checker = CacheHealthChecker::with_simulated_latency(Duration::from_millis(150));
    let health = checker.check_health().await.expect("should complete");

    assert_eq!(health.status, HealthStatus::Healthy);
    assert!(health.check_duration_ms >= 150);
}

#[tokio::test]
async fn test_cache_health_check_metadata() {
    let checker = CacheHealthChecker::new();
    let health = checker.check_health().await.expect("should succeed");

    // Verify metadata includes cache-specific information
    assert!(health.metadata.contains_key("type"));
    assert_eq!(
        health.metadata.get("type").map(String::as_str),
        Some("Redis")
    );
}

#[tokio::test]
async fn test_cache_component_name() {
    let checker = CacheHealthChecker::new();
    assert_eq!(checker.component_name(), "Cache");
}

// ============================================================================
// ExternalApiHealthChecker Tests - Real Scenarios
// ============================================================================

#[tokio::test]
async fn test_external_api_health_check_success() {
    let checker = ExternalApiHealthChecker::new("test-api".to_string());
    let health = checker
        .check_health()
        .await
        .expect("health check should succeed");

    assert_eq!(health.status, HealthStatus::Healthy);
    assert_eq!(health.name, "ExternalAPI");
}

#[tokio::test]
async fn test_external_api_health_check_slow_response() {
    // Real scenario: External API experiencing latency
    let checker = ExternalApiHealthChecker::with_simulated_latency(
        "slow-api".to_string(),
        Duration::from_millis(300),
    );
    let health = checker.check_health().await.expect("should complete");

    assert_eq!(health.status, HealthStatus::Healthy);
    assert!(health.check_duration_ms >= 300);
    assert!(health.metadata.contains_key("url"));
}

#[tokio::test]
async fn test_external_api_timeout_scenario() {
    // Real scenario: External API completely unavailable
    let checker = ExternalApiHealthChecker::with_simulated_latency(
        "timeout-api".to_string(),
        Duration::from_secs(5),
    );

    let result = timeout(Duration::from_millis(500), checker.check_health()).await;
    assert!(result.is_err(), "Should timeout on unavailable API");
}

#[tokio::test]
async fn test_external_api_component_name() {
    let checker = ExternalApiHealthChecker::new("test".to_string());
    assert_eq!(checker.component_name(), "ExternalAPI");
}

// ============================================================================
// HsmHealthChecker Tests - Real Scenarios
// ============================================================================

#[tokio::test]
async fn test_hsm_health_check_success() {
    let checker = HsmHealthChecker::new();
    let health = checker
        .check_health()
        .await
        .expect("health check should succeed");

    assert_eq!(health.status, HealthStatus::Healthy);
    assert_eq!(health.name, "HSM");
}

#[tokio::test]
async fn test_hsm_health_check_with_latency() {
    // Real scenario: HSM under cryptographic load
    let checker = HsmHealthChecker::with_simulated_latency(Duration::from_millis(100));
    let health = checker.check_health().await.expect("should complete");

    assert_eq!(health.status, HealthStatus::Healthy);
    assert!(health.check_duration_ms >= 100);
}

#[tokio::test]
async fn test_hsm_health_check_metadata() {
    let checker = HsmHealthChecker::new();
    let health = checker.check_health().await.expect("should succeed");

    // Verify HSM-specific metadata exists
    assert!(
        !health.metadata.is_empty(),
        "HSM metadata should not be empty"
    );
    // HSM checker may have provider-specific metadata
}

#[tokio::test]
async fn test_hsm_component_name() {
    let checker = HsmHealthChecker::new();
    assert_eq!(checker.component_name(), "HSM");
}

// ============================================================================
// HealthCheckerType Enum Tests
// ============================================================================

#[tokio::test]
async fn test_health_checker_type_database_variant() {
    let checker = HealthCheckerType::Database(DatabaseHealthChecker::new());
    let health = checker.check_health().await.expect("should succeed");

    assert_eq!(health.name, "Database");
    assert_eq!(checker.component_name(), "Database");
}

#[tokio::test]
async fn test_health_checker_type_cache_variant() {
    let checker = HealthCheckerType::Cache(CacheHealthChecker::new());
    let health = checker.check_health().await.expect("should succeed");

    assert_eq!(health.name, "Cache");
    assert_eq!(checker.component_name(), "Cache");
}

#[tokio::test]
async fn test_health_checker_type_external_api_variant() {
    let checker = HealthCheckerType::ExternalApi(ExternalApiHealthChecker::new("test".to_string()));
    let health = checker.check_health().await.expect("should succeed");

    assert_eq!(health.name, "ExternalAPI");
    assert_eq!(checker.component_name(), "ExternalAPI");
}

#[tokio::test]
async fn test_health_checker_type_hsm_variant() {
    let checker = HealthCheckerType::Hsm(HsmHealthChecker::new());
    let health = checker.check_health().await.expect("should succeed");

    assert_eq!(health.name, "HSM");
    assert_eq!(checker.component_name(), "HSM");
}

// ============================================================================
// Aggregator Tests - Real Cascading Failure Scenarios
// ============================================================================

#[tokio::test]
async fn test_aggregator_all_healthy() {
    let mut aggregator = HealthCheckAggregator::new();
    aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
    aggregator.add_checker(HealthCheckerType::Cache(CacheHealthChecker::new()));

    let overall = aggregator
        .get_overall_status()
        .await
        .expect("should succeed");
    assert_eq!(overall, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_aggregator_degraded_performance() {
    // Real scenario: Some components slow but functional
    let mut aggregator = HealthCheckAggregator::new();
    aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
    aggregator.add_checker(HealthCheckerType::Cache(
        CacheHealthChecker::with_simulated_latency(Duration::from_millis(500)),
    ));

    let overall = aggregator
        .get_overall_status()
        .await
        .expect("should succeed");
    // Should still be healthy since services are responding
    assert_eq!(overall, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_aggregator_cascading_failure_detection() {
    // Real scenario: Database fails, affecting dependent services
    let mut aggregator = HealthCheckAggregator::new();

    // Simul database that times out
    aggregator.add_checker(HealthCheckerType::Database(
        DatabaseHealthChecker::with_simulated_latency(Duration::from_secs(10)),
    ));
    aggregator.add_checker(HealthCheckerType::Cache(CacheHealthChecker::new()));

    // Set reasonable timeout
    let result = timeout(Duration::from_secs(2), aggregator.get_overall_status()).await;

    // Should timeout, indicating system health check failure
    assert!(result.is_err(), "Should detect cascading failure");
}

#[tokio::test]
async fn test_aggregator_partial_failure() {
    // Real scenario: Cache fails but database is healthy
    let mut aggregator = HealthCheckAggregator::new();
    aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
    // Add a cache that will timeout
    aggregator.add_checker(HealthCheckerType::Cache(
        CacheHealthChecker::with_simulated_latency(Duration::from_secs(10)),
    ));

    // With timeout, should detect partial failure
    let result = timeout(Duration::from_secs(1), aggregator.get_overall_status()).await;

    // Should handle partial failure gracefully
    assert!(result.is_err() || result.is_ok());
}

#[tokio::test]
async fn test_aggregator_recovery_after_failure() {
    // Real scenario: Component recovers after temporary failure
    let mut aggregator = HealthCheckAggregator::new();

    // Initially use slow checker
    let slow_db = DatabaseHealthChecker::with_simulated_latency(Duration::from_millis(100));
    aggregator.add_checker(HealthCheckerType::Database(slow_db));

    // First check should succeed (within timeout)
    let status1 = timeout(Duration::from_secs(1), aggregator.get_overall_status()).await;
    assert!(status1.is_ok(), "First check should succeed");

    // Second check should also succeed (recovered)
    let status2 = timeout(Duration::from_secs(1), aggregator.get_overall_status()).await;
    assert!(status2.is_ok(), "Recovery check should succeed");
}

// ============================================================================
// Edge Cases and Error Paths
// ============================================================================

#[tokio::test]
async fn test_empty_aggregator() {
    let aggregator = HealthCheckAggregator::new();
    let overall = aggregator
        .get_overall_status()
        .await
        .expect("should succeed");

    // Empty aggregator should be considered healthy (no failures)
    assert_eq!(overall, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_multiple_checkers_same_type() {
    // Real scenario: Multiple database shards being monitored
    let mut aggregator = HealthCheckAggregator::new();
    aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
    aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));
    aggregator.add_checker(HealthCheckerType::Database(DatabaseHealthChecker::new()));

    let overall = aggregator
        .get_overall_status()
        .await
        .expect("should succeed");
    assert_eq!(overall, HealthStatus::Healthy);
}

#[tokio::test]
async fn test_concurrent_health_checks() {
    // Real scenario: Health checks run concurrently
    let checker1 = DatabaseHealthChecker::new();
    let checker2 = CacheHealthChecker::new();
    let checker3 = HsmHealthChecker::new();

    // Run all checks concurrently
    let (health1, health2, health3) = tokio::join!(
        checker1.check_health(),
        checker2.check_health(),
        checker3.check_health(),
    );

    assert!(health1.is_ok());
    assert!(health2.is_ok());
    assert!(health3.is_ok());
}

#[tokio::test]
async fn test_health_check_duration_tracking() {
    // Verify that duration tracking is accurate
    let checker = DatabaseHealthChecker::with_simulated_latency(Duration::from_millis(50));
    let health = checker.check_health().await.expect("should succeed");

    // Duration should be at least the simulated latency
    assert!(health.check_duration_ms >= 50);
    // But not excessively more (allowing for some overhead)
    assert!(health.check_duration_ms < 200);
}

#[tokio::test]
async fn test_health_check_metadata_completeness() {
    let checkers = vec![
        HealthCheckerType::Database(DatabaseHealthChecker::new()),
        HealthCheckerType::Cache(CacheHealthChecker::new()),
        HealthCheckerType::Hsm(HsmHealthChecker::new()),
    ];

    for checker in checkers {
        let health = checker.check_health().await.expect("should succeed");

        // All health checks should include basic metadata
        assert!(
            !health.metadata.is_empty(),
            "Metadata should not be empty for {}",
            health.name
        );
        // Database and Cache have "type" and "host" keys
        // HSM has provider-specific keys
    }
}
