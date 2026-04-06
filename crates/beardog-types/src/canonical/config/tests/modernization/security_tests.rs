// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for Security and Advanced System Configuration
//!
//! Tests security-related configs including rate limiting, authentication,
//! failover, threading, and network resources.

use crate::canonical::config::domains::{network, system};
use crate::canonical::config::production::resources::NetworkResourceConfig;
use crate::canonical::config::security::RateLimitingConfig;
use crate::canonical::config::security::authentication::CanonicalAuthenticationConfig;
use std::time::Duration;

// ============================================================================
// Failover Configuration Tests
// ============================================================================

/// Test FailoverConfiguration::with_defaults()
#[test]
fn test_failover_configuration_with_defaults() {
    let config = network::connection::FailoverConfiguration::with_defaults();
    assert!(config.enabled);
    assert_eq!(
        config.detection_timeout_seconds,
        network::connection::FailoverConfiguration::DEFAULT_DETECTION_TIMEOUT_SECS
    );
    assert_eq!(
        config.max_attempts,
        network::connection::FailoverConfiguration::DEFAULT_MAX_ATTEMPTS
    );
}

/// Test FailoverConfiguration::from_env()
#[test]
fn test_failover_configuration_from_env() {
    let config = network::connection::FailoverConfiguration::from_env_provider(|k| match k {
        "BEARDOG_FAILOVER_DETECTION_TIMEOUT_SECS" => Some("60".to_string()),
        "BEARDOG_FAILOVER_MAX_ATTEMPTS" => Some("5".to_string()),
        _ => None,
    });
    assert_eq!(config.detection_timeout_seconds, 60);
    assert_eq!(config.max_attempts, 5);
}

// ============================================================================
// Rate Limiting Configuration Tests
// ============================================================================

/// Test RateLimitingConfig::with_defaults()
#[test]
fn test_rate_limiting_config_with_defaults() {
    let config = RateLimitingConfig::with_defaults();
    assert!(config.enabled);
    assert_eq!(
        config.max_requests_per_minute,
        RateLimitingConfig::DEFAULT_MAX_REQUESTS_PER_MINUTE
    );
    assert_eq!(
        config.burst_capacity,
        RateLimitingConfig::DEFAULT_BURST_CAPACITY
    );
    assert_eq!(
        config.window_seconds,
        RateLimitingConfig::DEFAULT_WINDOW_SECS
    );
}

/// Test RateLimitingConfig::from_env()
#[test]
fn test_rate_limiting_config_from_env() {
    let config = RateLimitingConfig::from_env_provider(|k| match k {
        "BEARDOG_RATE_LIMIT_MAX_REQUESTS_PER_MIN" => Some("200".to_string()),
        "BEARDOG_RATE_LIMIT_BURST_CAPACITY" => Some("20".to_string()),
        "BEARDOG_RATE_LIMIT_WINDOW_SECS" => Some("120".to_string()),
        _ => None,
    });
    assert_eq!(config.max_requests_per_minute, 200);
    assert_eq!(config.burst_capacity, 20);
    assert_eq!(config.window_seconds, 120);
}

// ============================================================================
// Threading Configuration Tests
// ============================================================================

/// Test ThreadingConfig::with_defaults()
#[test]
fn test_threading_config_with_defaults() {
    let config = system::ThreadingConfig::with_defaults();
    assert_eq!(
        config.worker_threads,
        system::ThreadingConfig::DEFAULT_WORKER_THREADS
    );
    assert_eq!(
        config.blocking_threads,
        system::ThreadingConfig::DEFAULT_BLOCKING_THREADS
    );
    assert_eq!(config.stack_size, None);
    assert!(config.enable_tls_optimization);
}

/// Test ThreadingConfig::from_env()
#[test]
fn test_threading_config_from_env() {
    let config = system::ThreadingConfig::from_env_provider(|k| match k {
        "BEARDOG_WORKER_THREADS" => Some("8".to_string()),
        "BEARDOG_BLOCKING_THREADS" => Some("16".to_string()),
        _ => None,
    });
    assert_eq!(config.worker_threads, 8);
    assert_eq!(config.blocking_threads, 16);
}

// ============================================================================
// Network Resource Configuration Tests
// ============================================================================

/// Test NetworkResourceConfig::with_defaults()
#[test]
fn test_network_resource_config_with_defaults() {
    let config = NetworkResourceConfig::with_defaults();
    assert_eq!(
        config.max_connections,
        NetworkResourceConfig::DEFAULT_MAX_CONNECTIONS
    );
    assert_eq!(
        config.connection_timeout,
        Duration::from_secs(NetworkResourceConfig::DEFAULT_CONNECTION_TIMEOUT_SECS)
    );
    assert_eq!(
        config.read_timeout,
        Duration::from_secs(NetworkResourceConfig::DEFAULT_READ_TIMEOUT_SECS)
    );
    assert_eq!(
        config.write_timeout,
        Duration::from_secs(NetworkResourceConfig::DEFAULT_WRITE_TIMEOUT_SECS)
    );
    assert!(config.keep_alive);
    assert!(config.tcp_nodelay);
}

/// Test NetworkResourceConfig::from_env()
#[test]
fn test_network_resource_config_from_env() {
    let config = NetworkResourceConfig::from_env_provider(|k| match k {
        "BEARDOG_PROD_MAX_CONNECTIONS" => Some("2000".to_string()),
        "BEARDOG_PROD_CONNECTION_TIMEOUT_SECS" => Some("60".to_string()),
        "BEARDOG_PROD_READ_TIMEOUT_SECS" => Some("45".to_string()),
        "BEARDOG_PROD_WRITE_TIMEOUT_SECS" => Some("45".to_string()),
        _ => None,
    });
    assert_eq!(config.max_connections, 2000);
    assert_eq!(config.connection_timeout, Duration::from_secs(60));
    assert_eq!(config.read_timeout, Duration::from_secs(45));
    assert_eq!(config.write_timeout, Duration::from_secs(45));
}

// ============================================================================
// Authentication Configuration Tests
// ============================================================================

/// Test CanonicalAuthenticationConfig::with_defaults()
#[test]
fn test_authentication_config_with_defaults() {
    let config = CanonicalAuthenticationConfig::with_defaults();
    assert_eq!(config.jwt_secret.as_ref(), "CHANGE_ME_IN_PRODUCTION");
    assert_eq!(
        config.jwt_expiration_seconds,
        CanonicalAuthenticationConfig::DEFAULT_JWT_EXPIRATION_SECS
    );
    assert_eq!(
        config.jwt_refresh_expiration_seconds,
        CanonicalAuthenticationConfig::DEFAULT_JWT_REFRESH_EXPIRATION_SECS
    );
    assert_eq!(
        config.api_key_min_length,
        CanonicalAuthenticationConfig::DEFAULT_API_KEY_MIN_LENGTH
    );
    assert_eq!(
        config.password_min_length,
        CanonicalAuthenticationConfig::DEFAULT_PASSWORD_MIN_LENGTH
    );
    assert_eq!(
        config.max_auth_attempts,
        CanonicalAuthenticationConfig::DEFAULT_MAX_AUTH_ATTEMPTS
    );
    assert_eq!(
        config.lockout_duration_seconds,
        CanonicalAuthenticationConfig::DEFAULT_LOCKOUT_DURATION_SECS
    );
}

/// Test CanonicalAuthenticationConfig::from_env()
#[test]
fn test_authentication_config_from_env() {
    let config = CanonicalAuthenticationConfig::from_env_provider(|k| match k {
        "BEARDOG_JWT_EXPIRATION_SECS" => Some("7200".to_string()),
        "BEARDOG_JWT_REFRESH_EXPIRATION_SECS" => Some("172800".to_string()),
        "BEARDOG_API_KEY_MIN_LENGTH" => Some("64".to_string()),
        "BEARDOG_PASSWORD_MIN_LENGTH" => Some("16".to_string()),
        "BEARDOG_MAX_AUTH_ATTEMPTS" => Some("5".to_string()),
        "BEARDOG_LOCKOUT_DURATION_SECS" => Some("1800".to_string()),
        _ => None,
    });
    assert_eq!(config.jwt_expiration_seconds, 7200);
    assert_eq!(config.jwt_refresh_expiration_seconds, 172_800);
    assert_eq!(config.api_key_min_length, 64);
    assert_eq!(config.password_min_length, 16);
    assert_eq!(config.max_auth_attempts, 5);
    assert_eq!(config.lockout_duration_seconds, 1800);
}
