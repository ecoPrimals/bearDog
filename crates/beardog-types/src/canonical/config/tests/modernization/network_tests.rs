//! Tests for Network Configuration
//!
//! Tests network-related configs including connection pools, timeouts,
//! circuit breakers, load balancers, and testing configurations.

use crate::canonical::config::domains::{network, testing};
use std::time::Duration;

/// Helper to set env var for test scope
struct EnvGuard {
    key: String,
}

impl EnvGuard {
    fn set(key: &str, value: &str) -> Self {
        std::env::set_var(key, value);
        Self {
            key: key.to_string(),
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        std::env::remove_var(&self.key);
    }
}

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

    let _g1 = EnvGuard::set("BEARDOG_CONNECTION_POOL_MAX_SIZE", "200");
    let _g2 = EnvGuard::set("BEARDOG_CONNECTION_IDLE_TIMEOUT_SECS", "1200");

    let config = ConnectionPoolConfig::from_env();

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

    let _g1 = EnvGuard::set("BEARDOG_KEEPALIVE_TIMEOUT_SECS", "120");
    let _g2 = EnvGuard::set("BEARDOG_TLS_HANDSHAKE_TIMEOUT_SECS", "60");

    let config = TimeoutConfiguration::from_env();

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

    let _g1 = EnvGuard::set("BEARDOG_CIRCUIT_BREAKER_RECOVERY_TIMEOUT_SECS", "120");
    let _g2 = EnvGuard::set("BEARDOG_CIRCUIT_BREAKER_MIN_THROUGHPUT", "50");

    let config = CircuitBreakerConfiguration::from_env();

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

    let _g1 = EnvGuard::set("BEARDOG_LB_HEALTH_CHECK_INTERVAL_SECS", "60");
    let _g2 = EnvGuard::set("BEARDOG_LB_UNHEALTHY_THRESHOLD", "5");
    let _g3 = EnvGuard::set("BEARDOG_LB_HEALTHY_THRESHOLD", "3");

    let config = LoadBalancerHealthCheckConfiguration::from_env();

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

    let _g1 = EnvGuard::set("BEARDOG_API_TEST_TIMEOUT_SECS", "60");
    let _g2 = EnvGuard::set("BEARDOG_API_TEST_MAX_CONCURRENT", "20");

    let config = CanonicalApiTestConfig::from_env();

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

    let _g1 = EnvGuard::set("BEARDOG_PROD_TEST_HEALTH_TIMEOUT_SECS", "60");
    let _g2 = EnvGuard::set("BEARDOG_CANARY_PERCENTAGE", "10.0");

    let config = CanonicalProductionTestConfig::from_env();

    assert_eq!(config.health_check_timeout_seconds, 60);
    assert_eq!(config.canary_percentage, 10.0);
}
