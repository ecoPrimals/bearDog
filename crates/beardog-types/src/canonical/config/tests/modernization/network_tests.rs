// SPDX-License-Identifier: AGPL-3.0-only

//! Tests for Network Configuration
//!
//! Tests network-related configs including connection pools, timeouts,
//! circuit breakers, load balancers, and testing configurations.

use crate::canonical::config::domains::{network, testing};
use std::time::Duration;

// ============================================================================
// Network Config Tests
// ============================================================================

#[test]
fn test_connection_pool_config_with_defaults() {
    use network::connection::ConnectionPoolConfig;

    let config = ConnectionPoolConfig::with_defaults();

    assert_eq!(config.max_size, ConnectionPoolConfig::DEFAULT_MAX_SIZE);
    assert_eq!(
        config.idle_timeout,
        Duration::from_secs(ConnectionPoolConfig::DEFAULT_IDLE_TIMEOUT_SECS)
    );
    assert_eq!(
        config.acquire_timeout,
        Duration::from_secs(ConnectionPoolConfig::DEFAULT_ACQUIRE_TIMEOUT_SECS)
    );
    assert_eq!(
        config.maintenance_interval,
        Duration::from_secs(ConnectionPoolConfig::DEFAULT_MAINTENANCE_INTERVAL_SECS)
    );
}

#[test]
fn test_connection_pool_config_from_env() {
    use network::connection::ConnectionPoolConfig;

    let config = ConnectionPoolConfig::from_env_provider(|k| match k {
        "BEARDOG_CONNECTION_POOL_MAX_SIZE" => Some("200".to_string()),
        "BEARDOG_CONNECTION_IDLE_TIMEOUT_SECS" => Some("1200".to_string()),
        _ => None,
    });

    assert_eq!(config.max_size, 200);
    assert_eq!(config.idle_timeout, Duration::from_secs(1200));
}

#[test]
fn test_timeout_configuration_with_defaults() {
    use network::connection::TimeoutConfiguration;

    let config = TimeoutConfiguration::with_defaults();

    assert_eq!(
        config.keepalive_timeout_seconds,
        TimeoutConfiguration::DEFAULT_KEEPALIVE_TIMEOUT_SECS
    );
    assert_eq!(
        config.tls_handshake_timeout_seconds,
        TimeoutConfiguration::DEFAULT_TLS_HANDSHAKE_TIMEOUT_SECS
    );
    assert_eq!(
        config.read_timeout_seconds,
        TimeoutConfiguration::DEFAULT_READ_TIMEOUT_SECS
    );
    assert_eq!(
        config.write_timeout_seconds,
        TimeoutConfiguration::DEFAULT_WRITE_TIMEOUT_SECS
    );
    assert_eq!(
        config.shutdown_timeout_seconds,
        TimeoutConfiguration::DEFAULT_SHUTDOWN_TIMEOUT_SECS
    );
}

#[test]
fn test_timeout_configuration_from_env() {
    use network::connection::TimeoutConfiguration;

    let config = TimeoutConfiguration::from_env_provider(|k| match k {
        "BEARDOG_KEEPALIVE_TIMEOUT_SECS" => Some("120".to_string()),
        "BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS" => Some("60".to_string()),
        _ => None,
    });

    assert_eq!(config.keepalive_timeout_seconds, 120);
    assert_eq!(config.tls_handshake_timeout_seconds, 60);
}

#[test]
fn test_circuit_breaker_configuration_with_defaults() {
    use network::connection::CircuitBreakerConfiguration;

    let config = CircuitBreakerConfiguration::with_defaults();

    assert_eq!(
        config.recovery_timeout_seconds,
        CircuitBreakerConfiguration::DEFAULT_RECOVERY_TIMEOUT_SECS
    );
    assert_eq!(
        config.minimum_throughput,
        CircuitBreakerConfiguration::DEFAULT_MIN_THROUGHPUT
    );
    assert!(config.enabled);
}

#[test]
fn test_circuit_breaker_configuration_from_env() {
    use network::connection::CircuitBreakerConfiguration;

    let config = CircuitBreakerConfiguration::from_env_provider(|k| match k {
        "BEARDOG_CIRCUIT_BREAKER_RECOVERY_TIMEOUT_SECS" => Some("120".to_string()),
        "BEARDOG_CIRCUIT_BREAKER_MIN_THROUGHPUT" => Some("50".to_string()),
        _ => None,
    });

    assert_eq!(config.recovery_timeout_seconds, 120);
    assert_eq!(config.minimum_throughput, 50);
}

#[test]
fn test_load_balancer_health_check_config_with_defaults() {
    use network::connection::LoadBalancerHealthCheckConfiguration;

    let config = LoadBalancerHealthCheckConfiguration::with_defaults();

    assert_eq!(
        config.interval_seconds,
        LoadBalancerHealthCheckConfiguration::DEFAULT_INTERVAL_SECS
    );
    assert_eq!(
        config.unhealthy_threshold,
        LoadBalancerHealthCheckConfiguration::DEFAULT_UNHEALTHY_THRESHOLD
    );
    assert_eq!(
        config.healthy_threshold,
        LoadBalancerHealthCheckConfiguration::DEFAULT_HEALTHY_THRESHOLD
    );
}

#[test]
fn test_load_balancer_health_check_config_from_env() {
    use network::connection::LoadBalancerHealthCheckConfiguration;

    let config = LoadBalancerHealthCheckConfiguration::from_env_provider(|k| match k {
        "BEARDOG_LB_HEALTH_CHECK_INTERVAL_SECS" => Some("60".to_string()),
        "BEARDOG_LB_UNHEALTHY_THRESHOLD" => Some("5".to_string()),
        "BEARDOG_LB_HEALTHY_THRESHOLD" => Some("3".to_string()),
        _ => None,
    });

    assert_eq!(config.interval_seconds, 60);
    assert_eq!(config.unhealthy_threshold, 5);
    assert_eq!(config.healthy_threshold, 3);
}

// ============================================================================
// Testing Config Tests
// ============================================================================

#[test]
fn test_canonical_api_test_config_with_defaults() {
    use testing::CanonicalApiTestConfig;

    let config = CanonicalApiTestConfig::with_defaults();

    assert_eq!(
        config.timeout_seconds,
        CanonicalApiTestConfig::DEFAULT_TIMEOUT_SECS
    );
    assert_eq!(
        config.max_concurrent_requests,
        CanonicalApiTestConfig::DEFAULT_MAX_CONCURRENT
    );
    assert_eq!(
        config.max_retries,
        CanonicalApiTestConfig::DEFAULT_MAX_RETRIES
    );
    assert_eq!(
        config.retry_delay_ms,
        CanonicalApiTestConfig::DEFAULT_RETRY_DELAY_MS
    );
}

#[test]
fn test_canonical_api_test_config_from_env() {
    use testing::CanonicalApiTestConfig;

    let config = CanonicalApiTestConfig::from_env_provider(|k| match k {
        "BEARDOG_API_TEST_TIMEOUT_SECS" => Some("60".to_string()),
        "BEARDOG_API_TEST_MAX_CONCURRENT" => Some("20".to_string()),
        _ => None,
    });

    assert_eq!(config.timeout_seconds, 60);
    assert_eq!(config.max_concurrent_requests, 20);
}

#[test]
fn test_canonical_production_test_config_with_defaults() {
    use testing::CanonicalProductionTestConfig;

    let config = CanonicalProductionTestConfig::with_defaults();

    assert_eq!(
        config.health_check_timeout_seconds,
        CanonicalProductionTestConfig::DEFAULT_HEALTH_TIMEOUT_SECS
    );
    assert_eq!(
        config.canary_percentage,
        CanonicalProductionTestConfig::DEFAULT_CANARY_PERCENTAGE
    );
}

#[test]
fn test_canonical_production_test_config_from_env() {
    use testing::CanonicalProductionTestConfig;

    let config = CanonicalProductionTestConfig::from_env_provider(|k| match k {
        "BEARDOG_PROD_TEST_HEALTH_TIMEOUT_SECS" => Some("60".to_string()),
        "BEARDOG_CANARY_PERCENTAGE" => Some("10.0".to_string()),
        _ => None,
    });

    assert_eq!(config.health_check_timeout_seconds, 60);
    assert_eq!(config.canary_percentage, 10.0);
}
