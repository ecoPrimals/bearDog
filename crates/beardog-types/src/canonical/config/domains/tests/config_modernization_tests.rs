// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for Modernized Configuration Structs
//!
//! This module tests all configs that have been modernized to follow the pattern:
//! - `with_defaults()`: Deterministic, no env vars
//! - `from_env()`: Explicit env var reading
//! - `Default::default()`: Delegates to `with_defaults()`
//!
//! These tests ensure:
//! 1. Deterministic behavior in `with_defaults()`
//! 2. Environment variable reading in `from_env()`
//! 3. Correct default delegation
//! 4. Constant values match defaults

use super::super::*;
use std::time::Duration;

/// Helper to set env var for test scope
struct EnvGuard {
    key: String,
}

impl EnvGuard {
    fn set(key: &str, value: &str) -> Self {
        beardog_errors::process_env::set_var(key, value);
        Self {
            key: key.to_string(),
        }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        beardog_errors::process_env::remove_var(&self.key);
    }
}

// ============================================================================
// Bootstrap Config Tests
// ============================================================================

#[test]
fn test_infant_pattern_config_with_defaults() {
    let config = bootstrap::InfantPatternConfig::with_defaults();
    
    assert_eq!(config.min_observations, bootstrap::InfantPatternConfig::DEFAULT_MIN_OBSERVATIONS);
    assert_eq!(config.confidence_threshold, bootstrap::InfantPatternConfig::DEFAULT_CONFIDENCE_THRESHOLD);
    assert_eq!(config.pattern_max_age, Duration::from_secs(bootstrap::InfantPatternConfig::DEFAULT_PATTERN_MAX_AGE_SECS));
    assert_eq!(config.learning_rate, bootstrap::InfantPatternConfig::DEFAULT_LEARNING_RATE);
    assert!(config.enable_continuous_learning);
    assert_eq!(config.consolidation_interval, Duration::from_secs(bootstrap::InfantPatternConfig::DEFAULT_CONSOLIDATION_INTERVAL_SECS));
}

#[test]
fn test_infant_pattern_config_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_PATTERN_MAX_AGE_SECS", "7200");
    let _g2 = EnvGuard::set("BEARDOG_PATTERN_CONSOLIDATION_INTERVAL_SECS", "600");
    
    let config = bootstrap::InfantPatternConfig::from_env();
    
    assert_eq!(config.pattern_max_age, Duration::from_secs(7200));
    assert_eq!(config.consolidation_interval, Duration::from_secs(600));
}

#[test]
fn test_infant_pattern_config_default_uses_with_defaults() {
    let default_config = bootstrap::InfantPatternConfig::default();
    let with_defaults_config = bootstrap::InfantPatternConfig::with_defaults();
    
    assert_eq!(default_config, with_defaults_config);
}

// ============================================================================
// Database Config Tests
// ============================================================================

#[test]
fn test_database_connection_config_with_defaults() {
    let config = database::DatabaseConnectionConfig::with_defaults();
    
    assert_eq!(config.url, database::DatabaseConnectionConfig::DEFAULT_URL);
    assert_eq!(config.timeout, Duration::from_secs(database::DatabaseConnectionConfig::DEFAULT_TIMEOUT_SECS));
    assert_eq!(config.ssl, database::DatabaseConnectionConfig::DEFAULT_SSL);
}

#[test]
fn test_database_connection_config_from_env() {
    let _g1 = EnvGuard::set("DATABASE_URL", "postgres://localhost/test");
    let _g2 = EnvGuard::set("DATABASE_TIMEOUT_SECONDS", "60");
    let _g3 = EnvGuard::set("DATABASE_SSL", "true");
    
    let config = database::DatabaseConnectionConfig::from_env();
    
    assert_eq!(config.url, "postgres://localhost/test");
    assert_eq!(config.timeout, Duration::from_secs(60));
    assert_eq!(config.ssl, true);
}

#[test]
fn test_database_pool_config_with_defaults() {
    let config = database::DatabasePoolConfig::with_defaults();
    
    assert_eq!(config.min_idle, database::DatabasePoolConfig::DEFAULT_MIN_IDLE);
    assert_eq!(config.idle_timeout, Duration::from_secs(database::DatabasePoolConfig::DEFAULT_IDLE_TIMEOUT_SECS));
}

#[test]
fn test_database_pool_config_from_env() {
    let _g1 = EnvGuard::set("DATABASE_POOL_MIN_IDLE", "5");
    let _g2 = EnvGuard::set("DATABASE_POOL_IDLE_TIMEOUT_SECONDS", "1200");
    
    let config = database::DatabasePoolConfig::from_env();
    
    assert_eq!(config.min_idle, 5);
    assert_eq!(config.idle_timeout, Duration::from_secs(1200));
}

#[test]
fn test_migration_config_with_defaults() {
    let config = database::MigrationConfig::with_defaults();
    
    assert_eq!(config.auto_migrate, database::MigrationConfig::DEFAULT_AUTO_MIGRATE);
    assert_eq!(config.directory, database::MigrationConfig::DEFAULT_DIRECTORY);
}

#[test]
fn test_migration_config_from_env() {
    let _g1 = EnvGuard::set("DATABASE_AUTO_MIGRATE", "false");
    let _g2 = EnvGuard::set("DATABASE_MIGRATION_DIR", "db/migrations");
    
    let config = database::MigrationConfig::from_env();
    
    assert_eq!(config.auto_migrate, false);
    assert_eq!(config.directory, "db/migrations");
}

// ============================================================================
// System Config Tests
// ============================================================================

#[test]
fn test_log_rotation_config_with_defaults() {
    let config = system::LogRotationConfig::with_defaults();
    
    assert_eq!(config.max_size_mb, system::LogRotationConfig::DEFAULT_MAX_SIZE_MB);
    assert_eq!(config.max_files, system::LogRotationConfig::DEFAULT_MAX_FILES);
}

#[test]
fn test_log_rotation_config_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_SYSTEM_LOG_MAX_SIZE_MB", "200");
    let _g2 = EnvGuard::set("BEARDOG_SYSTEM_LOG_MAX_FILES", "20");
    
    let config = system::LogRotationConfig::from_env();
    
    assert_eq!(config.max_size_mb, 200);
    assert_eq!(config.max_files, 20);
}

#[test]
fn test_resource_config_with_defaults() {
    let config = system::ResourceConfig::with_defaults();
    
    assert_eq!(config.max_file_descriptors, Some(system::ResourceConfig::DEFAULT_MAX_FILE_DESCRIPTORS));
    assert_eq!(config.max_connections, system::ResourceConfig::DEFAULT_MAX_CONNECTIONS);
    assert_eq!(config.monitoring_interval, Duration::from_secs(system::ResourceConfig::DEFAULT_MONITORING_INTERVAL_SECS));
}

#[test]
fn test_resource_config_from_env() {
    let _g1 = EnvGuard::set("BEARDOG_MAX_FILE_DESCRIPTORS", "32768");
    let _g2 = EnvGuard::set("BEARDOG_SYSTEM_MAX_CONNECTIONS", "5000");
    let _g3 = EnvGuard::set("BEARDOG_SYSTEM_MONITORING_INTERVAL_SECS", "30");
    
    let config = system::ResourceConfig::from_env();
    
    assert_eq!(config.max_file_descriptors, Some(32768));
    assert_eq!(config.max_connections, 5000);
    assert_eq!(config.monitoring_interval, Duration::from_secs(30));
}

#[test]
fn test_environment_config_with_defaults() {
    let config = system::EnvironmentConfig::with_defaults();
    
    assert_eq!(config.environment_type, system::EnvironmentConfig::DEFAULT_ENVIRONMENT_TYPE);
    assert!(config.variables.is_empty());
    assert!(config.overrides.is_empty());
}

#[test]
fn test_environment_config_from_env() {
    let _g = EnvGuard::set("BEARDOG_ENVIRONMENT", "production");
    
    let config = system::EnvironmentConfig::from_env();
    
    assert_eq!(config.environment_type, "production");
}

// ============================================================================
// Network Config Tests
// ============================================================================

#[test]
fn test_connection_pool_config_with_defaults() {
    use network::connection::ConnectionPoolConfig;
    
    let config = ConnectionPoolConfig::with_defaults();
    
    assert_eq!(config.max_size, ConnectionPoolConfig::DEFAULT_MAX_SIZE);
    assert_eq!(config.idle_timeout, Duration::from_secs(ConnectionPoolConfig::DEFAULT_IDLE_TIMEOUT_SECS));
    assert_eq!(config.acquire_timeout, Duration::from_secs(ConnectionPoolConfig::DEFAULT_ACQUIRE_TIMEOUT_SECS));
    assert_eq!(config.maintenance_interval, Duration::from_secs(ConnectionPoolConfig::DEFAULT_MAINTENANCE_INTERVAL_SECS));
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
    
    assert_eq!(config.keepalive_timeout_seconds, TimeoutConfiguration::DEFAULT_KEEPALIVE_TIMEOUT_SECS);
    assert_eq!(config.tls_handshake_timeout_seconds, TimeoutConfiguration::DEFAULT_TLS_HANDSHAKE_TIMEOUT_SECS);
    assert_eq!(config.read_timeout_seconds, TimeoutConfiguration::DEFAULT_READ_TIMEOUT_SECS);
    assert_eq!(config.write_timeout_seconds, TimeoutConfiguration::DEFAULT_WRITE_TIMEOUT_SECS);
    assert_eq!(config.shutdown_timeout_seconds, TimeoutConfiguration::DEFAULT_SHUTDOWN_TIMEOUT_SECS);
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
    
    assert_eq!(config.recovery_timeout_seconds, CircuitBreakerConfiguration::DEFAULT_RECOVERY_TIMEOUT_SECS);
    assert_eq!(config.minimum_throughput, CircuitBreakerConfiguration::DEFAULT_MIN_THROUGHPUT);
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
    
    assert_eq!(config.interval_seconds, LoadBalancerHealthCheckConfiguration::DEFAULT_INTERVAL_SECS);
    assert_eq!(config.unhealthy_threshold, LoadBalancerHealthCheckConfiguration::DEFAULT_UNHEALTHY_THRESHOLD);
    assert_eq!(config.healthy_threshold, LoadBalancerHealthCheckConfiguration::DEFAULT_HEALTHY_THRESHOLD);
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
    
    assert_eq!(config.timeout_seconds, CanonicalApiTestConfig::DEFAULT_TIMEOUT_SECS);
    assert_eq!(config.max_concurrent_requests, CanonicalApiTestConfig::DEFAULT_MAX_CONCURRENT);
    assert_eq!(config.max_retries, CanonicalApiTestConfig::DEFAULT_MAX_RETRIES);
    assert_eq!(config.retry_delay_ms, CanonicalApiTestConfig::DEFAULT_RETRY_DELAY_MS);
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
    
    assert_eq!(config.health_check_timeout_seconds, CanonicalProductionTestConfig::DEFAULT_HEALTH_TIMEOUT_SECS);
    assert_eq!(config.canary_percentage, CanonicalProductionTestConfig::DEFAULT_CANARY_PERCENTAGE);
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

// ============================================================================
// Determinism Tests (Critical for Test Reliability)
// ============================================================================

#[test]
fn test_all_with_defaults_are_deterministic() {
    // Call with_defaults() multiple times and ensure same results
    let c1 = bootstrap::InfantPatternConfig::with_defaults();
    let c2 = bootstrap::InfantPatternConfig::with_defaults();
    assert_eq!(c1, c2, "InfantPatternConfig::with_defaults() is not deterministic");
    
    let c1 = database::DatabaseConnectionConfig::with_defaults();
    let c2 = database::DatabaseConnectionConfig::with_defaults();
    assert_eq!(c1.url, c2.url);
    assert_eq!(c1.timeout, c2.timeout);
    assert_eq!(c1.ssl, c2.ssl);
    
    let c1 = system::LogRotationConfig::with_defaults();
    let c2 = system::LogRotationConfig::with_defaults();
    assert_eq!(c1.max_size_mb, c2.max_size_mb);
    assert_eq!(c1.max_files, c2.max_files);
}

#[test]
fn test_constants_match_defaults() {
    // Ensure constants match what with_defaults() actually returns
    use bootstrap::InfantPatternConfig;
    let config = InfantPatternConfig::with_defaults();
    assert_eq!(config.min_observations, InfantPatternConfig::DEFAULT_MIN_OBSERVATIONS);
    assert_eq!(config.confidence_threshold, InfantPatternConfig::DEFAULT_CONFIDENCE_THRESHOLD);
    
    use database::DatabaseConnectionConfig;
    let config = DatabaseConnectionConfig::with_defaults();
    assert_eq!(config.url, DatabaseConnectionConfig::DEFAULT_URL);
    
    use system::ResourceConfig;
    let config = ResourceConfig::with_defaults();
    assert_eq!(config.max_connections, ResourceConfig::DEFAULT_MAX_CONNECTIONS);
}

#[test]
fn test_from_env_fallbacks_to_defaults() {
    // Ensure from_env() uses defaults when env vars not set
    // Clear any existing env vars first
    beardog_errors::process_env::remove_var("BEARDOG_PATTERN_MAX_AGE_SECS");
    beardog_errors::process_env::remove_var("BEARDOG_PATTERN_CONSOLIDATION_INTERVAL_SECS");
    
    let config = bootstrap::InfantPatternConfig::from_env();
    let defaults = bootstrap::InfantPatternConfig::with_defaults();
    
    assert_eq!(config, defaults, "from_env() should fallback to defaults when env vars not set");
}

#[test]
fn test_default_delegates_to_with_defaults() {
    // Verify Default trait delegates to with_defaults()
    let default_config = bootstrap::InfantPatternConfig::default();
    let with_defaults_config = bootstrap::InfantPatternConfig::with_defaults();
    
    assert_eq!(default_config, with_defaults_config);
}

