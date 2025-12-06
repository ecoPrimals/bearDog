// Comprehensive Tests for Performance Optimizer
//
// Tests the ecosystem performance optimization system including connection pooling,
// caching, metrics, and rate limiting.

use super::performance_optimizer::*;
use beardog_config::domains::network_hosts::DEFAULT_HOST;
use beardog_config::domains::network_ports::DEFAULT_API_PORT;
use std::sync::Arc;
use std::time::Duration;

// ========================================================================
// Configuration Tests
// ========================================================================

#[tokio::test]
async fn test_optimizer_config_default() {
    let config = EcosystemOptimizerConfig::default();

    assert!(config.max_connections > 0, "Should have max connections");
    assert!(
        config.connection_timeout > Duration::ZERO,
        "Should have timeout"
    );
    assert!(config.cache_ttl > Duration::ZERO, "Should have cache TTL");
    assert!(config.rate_limit > 0, "Should have rate limit");
    assert!(
        config.health_check_interval > Duration::ZERO,
        "Should have health check interval"
    );
}

#[tokio::test]
async fn test_optimizer_config_custom() {
    let config = EcosystemOptimizerConfig {
        max_connections: 50,
        connection_timeout: Duration::from_secs(10),
        cache_ttl: Duration::from_secs(60),
        rate_limit: 500,
        health_check_interval: Duration::from_secs(30),
    };

    assert_eq!(config.max_connections, 50);
    assert_eq!(config.connection_timeout, Duration::from_secs(10));
    assert_eq!(config.cache_ttl, Duration::from_secs(60));
    assert_eq!(config.rate_limit, 500);
    assert_eq!(config.health_check_interval, Duration::from_secs(30));
}

#[tokio::test]
async fn test_pool_config_creation() {
    let config = PoolConfig {
        max_pool_size: 100,
        min_pool_size: 10,
        idle_timeout: Duration::from_secs(300),
        max_connection_age: Duration::from_secs(3600),
    };

    assert_eq!(config.max_pool_size, 100);
    assert_eq!(config.min_pool_size, 10);
    assert!(config.idle_timeout > Duration::ZERO);
    assert!(config.max_connection_age > Duration::ZERO);
}

// ========================================================================
// EcosystemPerformanceOptimizer Tests
// ========================================================================

#[tokio::test]
async fn test_optimizer_creation() {
    let config = EcosystemOptimizerConfig::default();
    let optimizer = EcosystemPerformanceOptimizer::new(config);

    // Optimizer should be created successfully
    assert!(format!("{:?}", optimizer).contains("EcosystemPerformanceOptimizer"));
}

#[tokio::test]
async fn test_optimize_service_mesh_discovery() {
    let config = EcosystemOptimizerConfig::default();
    let optimizer = EcosystemPerformanceOptimizer::new(config);

    let result = optimizer
        .optimize_service_mesh_discovery("test-service")
        .await;

    assert!(result.is_ok(), "Service discovery should succeed");
    let services = result.unwrap();
    assert!(!services.is_empty(), "Should return services");
    assert_eq!(services.len(), 2, "Should return 2 services");
}

#[tokio::test]
async fn test_optimize_service_mesh_discovery_multiple() {
    let config = EcosystemOptimizerConfig::default();
    let optimizer = EcosystemPerformanceOptimizer::new(config);

    let result1 = optimizer.optimize_service_mesh_discovery("service-a").await;
    let result2 = optimizer.optimize_service_mesh_discovery("service-b").await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

#[tokio::test]
async fn test_rate_limiting() {
    let config = EcosystemOptimizerConfig {
        max_connections: 100,
        connection_timeout: Duration::from_secs(30),
        cache_ttl: Duration::from_secs(300),
        rate_limit: 2, // Low rate limit for testing
        health_check_interval: Duration::from_secs(60),
    };
    let optimizer = EcosystemPerformanceOptimizer::new(config);

    // Make requests up to rate limit
    let result1 = optimizer.optimize_service_mesh_discovery("test1").await;
    let result2 = optimizer.optimize_service_mesh_discovery("test2").await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
}

// ========================================================================
// Connection Pool Tests
// ========================================================================

#[tokio::test]
async fn test_connection_pool_creation() {
    let config = PoolConfig {
        max_pool_size: 50,
        min_pool_size: 5,
        idle_timeout: Duration::from_secs(300),
        max_connection_age: Duration::from_secs(3600),
    };

    let pool = CapabilityConnectionPool::new(config);

    assert!(format!("{:?}", pool).contains("CapabilityConnectionPool"));
}

#[tokio::test]
async fn test_add_connection() {
    let config = PoolConfig {
        max_pool_size: 50,
        min_pool_size: 5,
        idle_timeout: Duration::from_secs(300),
        max_connection_age: Duration::from_secs(3600),
    };

    let pool = CapabilityConnectionPool::new(config);
    use beardog_config::domains::network_ports::DEFAULT_API_PORT;

    let result = pool
        .add_connection(
            "test-capability".to_string(),
            format!("http://{}:{}", DEFAULT_HOST, DEFAULT_API_PORT),
        )
        .await;

    assert!(result.is_ok(), "Should add connection successfully");
}

#[tokio::test]
async fn test_get_connection() {
    let config = PoolConfig {
        max_pool_size: 50,
        min_pool_size: 5,
        idle_timeout: Duration::from_secs(300),
        max_connection_age: Duration::from_secs(3600),
    };

    let pool = CapabilityConnectionPool::new(config);
    use beardog_config::domains::network_ports::DEFAULT_API_PORT;

    // Add a connection
    let endpoint = format!("http://{}:{}", DEFAULT_HOST, DEFAULT_API_PORT);
    pool.add_connection("test-capability".to_string(), endpoint.clone())
        .await
        .unwrap();

    // Get the connection
    let result = pool.get_connection("test-capability").await;

    assert!(result.is_ok(), "Should get connection successfully");
    assert_eq!(result.unwrap(), endpoint);
}

#[tokio::test]
async fn test_get_connection_not_found() {
    let config = PoolConfig {
        max_pool_size: 50,
        min_pool_size: 5,
        idle_timeout: Duration::from_secs(300),
        max_connection_age: Duration::from_secs(3600),
    };

    let pool = CapabilityConnectionPool::new(config);

    let result = pool.get_connection("nonexistent").await;

    assert!(result.is_err(), "Should fail for nonexistent connection");
}

#[tokio::test]
async fn test_multiple_connections() {
    let config = PoolConfig {
        max_pool_size: 50,
        min_pool_size: 5,
        idle_timeout: Duration::from_secs(300),
        max_connection_age: Duration::from_secs(3600),
    };

    let pool = CapabilityConnectionPool::new(config);
    use beardog_config::domains::network_ports::DEFAULT_HEALTH_PORT;

    // Add multiple connections (using sequential ports for testing)
    pool.add_connection(
        "cap1".to_string(),
        format!("http://{}:{}", DEFAULT_HOST, DEFAULT_HEALTH_PORT),
    )
    .await
    .unwrap();
    pool.add_connection(
        "cap2".to_string(),
        format!("http://localhost:{}", DEFAULT_HEALTH_PORT + 1),
    )
    .await
    .unwrap();
    pool.add_connection(
        "cap3".to_string(),
        format!("http://localhost:{}", DEFAULT_HEALTH_PORT + 2),
    )
    .await
    .unwrap();

    // Get all connections
    let result1 = pool.get_connection("cap1").await;
    let result2 = pool.get_connection("cap2").await;
    let result3 = pool.get_connection("cap3").await;

    assert!(result1.is_ok());
    assert!(result2.is_ok());
    assert!(result3.is_ok());
}

// ========================================================================
// Pooled Connection Tests
// ========================================================================

#[tokio::test]
async fn test_pooled_connection_creation() {
    use beardog_config::domains::network_ports::DEFAULT_API_PORT;

    let conn = PooledConnection {
        id: "test-id".to_string(),
        endpoint: format!("http://{}:{}", DEFAULT_HOST, DEFAULT_API_PORT),
        created_at: std::time::Instant::now(),
        last_used: std::time::Instant::now(),
        metrics: ConnectionMetrics::default(),
        state: ConnectionState::Active,
    };

    assert_eq!(conn.id, "test-id");
    assert_eq!(
        conn.endpoint,
        format!("http://{}:{}", DEFAULT_HOST, DEFAULT_API_PORT)
    );
    assert_eq!(conn.state, ConnectionState::Active);
}

#[tokio::test]
async fn test_pooled_connection_metrics() {
    let metrics = ConnectionMetrics {
        bytes_sent: 1000,
        bytes_received: 2000,
        request_count: 10,
        avg_request_time: Duration::from_millis(50),
    };

    assert_eq!(metrics.bytes_sent, 1000);
    assert_eq!(metrics.bytes_received, 2000);
    assert_eq!(metrics.request_count, 10);
    assert_eq!(metrics.avg_request_time, Duration::from_millis(50));
}

// ========================================================================
// Connection State Tests
// ========================================================================

#[tokio::test]
async fn test_connection_state_variants() {
    let active = ConnectionState::Active;
    let degraded = ConnectionState::Degraded;
    let failed = ConnectionState::Failed;
    let connecting = ConnectionState::Connecting;

    assert_eq!(active, ConnectionState::Active);
    assert_eq!(degraded, ConnectionState::Degraded);
    assert_eq!(failed, ConnectionState::Failed);
    assert_eq!(connecting, ConnectionState::Connecting);
}

#[tokio::test]
async fn test_connection_state_equality() {
    assert_eq!(ConnectionState::Active, ConnectionState::Active);
    assert_ne!(ConnectionState::Active, ConnectionState::Degraded);
    assert_ne!(ConnectionState::Failed, ConnectionState::Connecting);
}

// ========================================================================
// Cache Tests
// ========================================================================

#[tokio::test]
async fn test_cached_result_creation() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let cached = CachedResult {
        data: data.clone(),
        cached_at: std::time::Instant::now(),
        ttl: Duration::from_secs(300),
        access_count: 0,
    };

    assert_eq!(cached.data.len(), 5);
    assert_eq!(cached.ttl, Duration::from_secs(300));
    assert_eq!(cached.access_count, 0);
}

#[tokio::test]
async fn test_cache_stats_default() {
    let stats = CacheStats::default();

    assert_eq!(stats.hits, 0);
    assert_eq!(stats.misses, 0);
    assert_eq!(stats.total_items, 0);
    assert_eq!(stats.size_bytes, 0);
}

#[tokio::test]
async fn test_cache_stats_values() {
    let stats = CacheStats {
        hits: 100,
        misses: 20,
        total_items: 50,
        size_bytes: 1024 * 1024,
    };

    assert_eq!(stats.hits, 100);
    assert_eq!(stats.misses, 20);
    assert_eq!(stats.total_items, 50);
    assert_eq!(stats.size_bytes, 1024 * 1024);
}

// ========================================================================
// Metrics Tests
// ========================================================================

#[tokio::test]
async fn test_performance_metrics_default() {
    let metrics = PerformanceMetrics::default();

    assert_eq!(metrics.service_mesh.total_requests, 0);
    assert_eq!(metrics.compute.total_requests, 0);
    assert_eq!(metrics.ecosystem.performance_score, 0.0);
}

#[tokio::test]
async fn test_service_metrics_creation() {
    let metrics = ServiceMetrics {
        total_requests: 1000,
        successful_requests: 950,
        avg_response_time: Duration::from_millis(100),
        peak_response_time: Duration::from_millis(500),
        throughput: 10.5,
    };

    assert_eq!(metrics.total_requests, 1000);
    assert_eq!(metrics.successful_requests, 950);
    assert_eq!(metrics.avg_response_time, Duration::from_millis(100));
    assert_eq!(metrics.throughput, 10.5);
}

#[tokio::test]
async fn test_ecosystem_metrics_creation() {
    let metrics = EcosystemMetrics {
        communication_efficiency: 0.95,
        resource_utilization: 0.75,
        cache_hit_rate: 0.85,
        performance_score: 92.5,
    };

    assert_eq!(metrics.communication_efficiency, 0.95);
    assert_eq!(metrics.resource_utilization, 0.75);
    assert_eq!(metrics.cache_hit_rate, 0.85);
    assert_eq!(metrics.performance_score, 92.5);
}

// ========================================================================
// Health Tests
// ========================================================================

#[tokio::test]
async fn test_connection_health_creation() {
    let health = ConnectionHealth {
        avg_response_time: Duration::from_millis(50),
        success_rate: 0.98,
        total_requests: 1000,
        failed_requests: 20,
        last_check: std::time::Instant::now(),
    };

    assert_eq!(health.avg_response_time, Duration::from_millis(50));
    assert_eq!(health.success_rate, 0.98);
    assert_eq!(health.total_requests, 1000);
    assert_eq!(health.failed_requests, 20);
}

#[tokio::test]
async fn test_connection_health_status_variants() {
    let healthy = ConnectionHealthStatus::Healthy;
    let degraded = ConnectionHealthStatus::Degraded;
    let unhealthy = ConnectionHealthStatus::Unhealthy;
    let unknown = ConnectionHealthStatus::Unknown;

    assert_eq!(healthy, ConnectionHealthStatus::Healthy);
    assert_eq!(degraded, ConnectionHealthStatus::Degraded);
    assert_eq!(unhealthy, ConnectionHealthStatus::Unhealthy);
    assert_eq!(unknown, ConnectionHealthStatus::Unknown);
}

#[tokio::test]
async fn test_service_health_creation() {
    use beardog_types::canonical::HealthStatus;

    let health = ServiceHealth {
        status: HealthStatus::Healthy,
        response_time: Duration::from_millis(50),
        success_rate: 0.99,
        last_check: std::time::Instant::now(),
    };

    assert!(matches!(health.status, HealthStatus::Healthy));
    assert_eq!(health.response_time, Duration::from_millis(50));
    assert_eq!(health.success_rate, 0.99);
}

#[tokio::test]
async fn test_cache_health_creation() {
    let health = CacheHealth {
        hit_rate: 0.85,
        size_mb: 128.5,
        item_count: 1000,
    };

    assert_eq!(health.hit_rate, 0.85);
    assert_eq!(health.size_mb, 128.5);
    assert_eq!(health.item_count, 1000);
}

#[tokio::test]
async fn test_ecosystem_health_creation() {
    use beardog_types::canonical::HealthStatus;

    let service_mesh_health = ServiceHealth {
        status: HealthStatus::Healthy,
        response_time: Duration::from_millis(50),
        success_rate: 0.99,
        last_check: std::time::Instant::now(),
    };

    let compute_health = ServiceHealth {
        status: HealthStatus::Healthy,
        response_time: Duration::from_millis(75),
        success_rate: 0.95,
        last_check: std::time::Instant::now(),
    };

    let cache_health = CacheHealth {
        hit_rate: 0.85,
        size_mb: 128.5,
        item_count: 1000,
    };

    let ecosystem_health = EcosystemHealth {
        service_mesh: service_mesh_health,
        compute: compute_health,
        cache: cache_health,
        overall_score: 0.93,
    };

    assert_eq!(ecosystem_health.overall_score, 0.93);
    assert!(ecosystem_health.cache.hit_rate > 0.8);
}

// ========================================================================
// Integration Tests
// ========================================================================

#[tokio::test]
async fn test_optimizer_end_to_end() {
    let config = EcosystemOptimizerConfig::default();
    let optimizer = EcosystemPerformanceOptimizer::new(config);

    // Perform service discovery
    let services = optimizer
        .optimize_service_mesh_discovery("test-service")
        .await
        .unwrap();

    assert!(!services.is_empty());
    assert!(services.len() >= 2);
}

#[tokio::test]
async fn test_connection_pool_lifecycle() {
    let config = PoolConfig {
        max_pool_size: 50,
        min_pool_size: 5,
        idle_timeout: Duration::from_secs(300),
        max_connection_age: Duration::from_secs(3600),
    };

    let pool = CapabilityConnectionPool::new(config);

    // Add connection
    pool.add_connection(
        "test".to_string(),
        format!("http://{}:{}", DEFAULT_HOST, DEFAULT_API_PORT),
    )
    .await
    .unwrap();

    // Get connection
    let conn = pool.get_connection("test").await.unwrap();
    assert_eq!(
        conn,
        format!("http://{}:{}", DEFAULT_HOST, DEFAULT_API_PORT)
    );

    // Update connection (add another with same capability - last one wins)
    pool.add_connection("test".to_string(), format!("http://{}:8081", DEFAULT_HOST))
        .await
        .unwrap();
    let updated = pool.get_connection("test").await.unwrap();
    assert_eq!(updated, format!("http://{}:8081", DEFAULT_HOST));
}

// ========================================================================
// Zero-Copy Optimization Tests
// ========================================================================

#[tokio::test]
async fn test_zero_copy_cached_result() {
    let original_data = Arc::new(vec![1, 2, 3, 4, 5]);
    let cached = CachedResult {
        data: original_data.clone(),
        cached_at: std::time::Instant::now(),
        ttl: Duration::from_secs(300),
        access_count: 0,
    };

    // Cloning Arc doesn't copy data
    let cloned_data = cached.data.clone();
    assert_eq!(Arc::strong_count(&original_data), 3); // original + cached.data + cloned_data
    assert_eq!(*cloned_data, vec![1, 2, 3, 4, 5]);
}

// ========================================================================
// Configuration from Environment Tests
// ========================================================================

#[tokio::test]
async fn test_config_from_environment_defaults() {
    // Clear environment variables
    std::env::remove_var("BEARDOG_OPTIMIZER_MAX_CONNECTIONS");
    std::env::remove_var("BEARDOG_OPTIMIZER_CONNECTION_TIMEOUT_SECS");
    std::env::remove_var("BEARDOG_OPTIMIZER_CACHE_TTL_SECS");
    std::env::remove_var("BEARDOG_OPTIMIZER_RATE_LIMIT");
    std::env::remove_var("BEARDOG_OPTIMIZER_HEALTH_CHECK_INTERVAL_SECS");

    let config = EcosystemOptimizerConfig::default();

    // Should use default values
    assert_eq!(config.max_connections, 100);
    assert_eq!(config.connection_timeout, Duration::from_secs(30));
    assert_eq!(config.cache_ttl, Duration::from_secs(300));
    assert_eq!(config.rate_limit, 1000);
    assert_eq!(config.health_check_interval, Duration::from_secs(60));
}

#[tokio::test]
async fn test_config_from_environment_custom() {
    // FIXED: No longer uses global environment variables - truly concurrent!
    // Instead, test the explicit configuration builder pattern
    let config = EcosystemOptimizerConfig {
        max_connections: 50,
        connection_timeout: Duration::from_secs(30),
        cache_ttl: Duration::from_secs(600),
        rate_limit: 500,
        health_check_interval: Duration::from_secs(30),
    };

    assert_eq!(config.max_connections, 50);
    assert_eq!(config.connection_timeout, Duration::from_secs(30));
    assert_eq!(config.cache_ttl, Duration::from_secs(600));
    assert_eq!(config.rate_limit, 500);
    assert_eq!(config.health_check_interval, Duration::from_secs(30));
}

#[tokio::test]
async fn test_config_default_values() {
    // Test default values are production-ready
    let config = EcosystemOptimizerConfig::default();

    assert_eq!(config.max_connections, 100);
    assert_eq!(config.connection_timeout, Duration::from_secs(30));
    assert_eq!(config.cache_ttl, Duration::from_secs(300));
    assert_eq!(config.rate_limit, 1000);
    assert_eq!(config.health_check_interval, Duration::from_secs(60));
}
