// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for Timeout Configuration
//!
//! Comprehensive test suite for all timeout functionality.

use super::*;
use std::time::Duration;

#[test]
fn test_default_timeouts() {
    let config = TimeoutConfig::default();
    assert_eq!(config.health_check_secs, 5);
    assert_eq!(config.hsm_operation_secs, 2);
    assert_eq!(config.hsm_probe_millis, 500);
    assert_eq!(config.discovery_operation_secs, 10);
    assert_eq!(config.ai_decision_secs, 30);
    assert_eq!(config.ai_request_secs, 30);
    assert_eq!(config.ai_batch_timeout_millis, 10);
    assert_eq!(config.pool_idle_secs, 300);
    assert_eq!(config.max_connection_age_secs, 3600);
}

#[test]
fn test_duration_conversions() {
    let config = TimeoutConfig::default();
    assert_eq!(config.health_check_duration(), Duration::from_secs(5));
    assert_eq!(config.hsm_operation_duration(), Duration::from_secs(2));
    assert_eq!(config.hsm_probe_duration(), Duration::from_millis(500));
    assert_eq!(
        config.discovery_operation_duration(),
        Duration::from_secs(10)
    );
    assert_eq!(config.ai_decision_duration(), Duration::from_secs(30));
    assert_eq!(config.ai_request_duration(), Duration::from_secs(30));
    assert_eq!(
        config.ai_batch_timeout_duration(),
        Duration::from_millis(10)
    );
    assert_eq!(config.pool_idle_duration(), Duration::from_secs(300));
    assert_eq!(
        config.max_connection_age_duration(),
        Duration::from_secs(3600)
    );
}

#[test]
fn test_builder_pattern() {
    let config = TimeoutConfig::builder()
        .health_check_secs(10)
        .hsm_operation_secs(5)
        .build();

    assert_eq!(config.health_check_secs, 10);
    assert_eq!(config.hsm_operation_secs, 5);
    // Others should be defaults
    assert_eq!(config.hsm_probe_millis, 500);
}

#[test]
fn test_validation_success() {
    let config = TimeoutConfig::default();
    assert!(config.validate().is_ok());
}

#[test]
fn test_validation_failure_health_check() {
    let config = TimeoutConfig::builder().health_check_secs(0).build();
    assert!(config.validate().is_err());

    let config = TimeoutConfig::builder().health_check_secs(61).build();
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_failure_hsm_operation() {
    let config = TimeoutConfig::builder().hsm_operation_secs(0).build();
    assert!(config.validate().is_err());

    let config = TimeoutConfig::builder().hsm_operation_secs(11).build();
    assert!(config.validate().is_err());
}

#[test]
fn test_validation_failure_hsm_probe() {
    let config = TimeoutConfig::builder().hsm_probe_millis(50).build();
    assert!(config.validate().is_err());

    let config = TimeoutConfig::builder().hsm_probe_millis(6000).build();
    assert!(config.validate().is_err());
}

#[test]
fn test_serialization() {
    let config = TimeoutConfig::default();
    let json = serde_json::to_string(&config).expect("serialization failed");
    let deserialized: TimeoutConfig = serde_json::from_str(&json).expect("deserialization failed");
    assert_eq!(config, deserialized);
}

#[test]
fn test_clone() {
    let config = TimeoutConfig::default();
    let cloned = config.clone();
    assert_eq!(config, cloned);
}

#[test]
fn test_debug() {
    let config = TimeoutConfig::default();
    let debug_str = format!("{config:?}");
    assert!(debug_str.contains("TimeoutConfig"));
}
