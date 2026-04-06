// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(
    unused_imports,
    clippy::float_cmp,
    clippy::useless_vec,
    clippy::needless_range_loop,
    clippy::uninlined_format_args,
    clippy::field_reassign_with_default,
    clippy::manual_range_contains,
    unused_variables,
    dead_code
)]

// Comprehensive tests for health monitoring module

use crate::universal_discovery::health::*;
use beardog_types::canonical::HealthStatus;
use chrono::Utc;

#[test]
fn test_health_check_config_default() {
    let config = HealthCheckConfig::default();

    assert_eq!(config.check_interval_secs, 30);
    assert_eq!(config.check_timeout_ms, 5000);
    assert_eq!(config.failure_threshold, 3);
    assert_eq!(config.success_threshold, 2);
    assert!(config.enable_detailed_metrics);
    assert_eq!(config.check_methods.len(), 2);
}

#[test]
fn test_health_check_config_creation() {
    let config = HealthCheckConfig {
        check_interval_secs: 60,
        check_timeout_ms: 10000,
        failure_threshold: 5,
        success_threshold: 3,
        enable_detailed_metrics: false,
        check_methods: vec![HealthCheckMethod::Http],
    };

    assert_eq!(config.check_interval_secs, 60);
    assert_eq!(config.failure_threshold, 5);
    assert!(!config.enable_detailed_metrics);
}

#[test]
fn test_health_check_method_variants() {
    let methods = [
        HealthCheckMethod::Http,
        HealthCheckMethod::Https,
        HealthCheckMethod::Tcp,
        HealthCheckMethod::Udp,
        HealthCheckMethod::Ping,
        HealthCheckMethod::Custom("grpc".to_string()),
    ];

    assert_eq!(methods.len(), 6);
}

#[test]
fn test_health_check_method_serialization() {
    let method = HealthCheckMethod::Http;
    let serialized = serde_json::to_string(&method).unwrap();
    let _deserialized: HealthCheckMethod = serde_json::from_str(&serialized).unwrap();
    // Both serialize/deserialize successfully
    assert!(serialized.contains("Http"));
}

#[test]
fn test_service_health_config_creation() {
    let config = ServiceHealthConfig {
        check_method: HealthCheckMethod::Https,
        check_interval_secs: 45,
        failure_threshold: 4,
        success_threshold: 2,
    };

    assert_eq!(config.check_interval_secs, 45);
    assert_eq!(config.failure_threshold, 4);
}

#[test]
fn test_health_statistics_default() {
    let stats = HealthStatistics::default();

    assert_eq!(stats.total_services, 0);
    assert_eq!(stats.healthy_services, 0);
    assert_eq!(stats.unhealthy_services, 0);
    assert!((stats.average_response_time_ms - 0.0).abs() < f64::EPSILON);
    assert_eq!(stats.total_health_checks, 0);
    assert_eq!(stats.total_failures, 0);
}

#[test]
fn test_health_statistics_creation() {
    let stats = HealthStatistics {
        total_services: 10,
        healthy_services: 8,
        unhealthy_services: 2,
        average_response_time_ms: 125.5,
        total_health_checks: 1000,
        total_failures: 25,
    };

    assert_eq!(stats.total_services, 10);
    assert_eq!(stats.healthy_services, 8);
    assert_eq!(stats.total_health_checks, 1000);
}

#[test]
fn test_service_health_state_creation() {
    let state = ServiceHealthState {
        service_id: "service-123".to_string(),
        current_status: HealthStatus::Healthy,
        consecutive_failures: 0,
        consecutive_successes: 5,
        last_check: Utc::now(),
        total_checks: 100,
        total_failures: 3,
    };

    assert_eq!(state.service_id, "service-123");
    assert_eq!(state.consecutive_successes, 5);
    assert_eq!(state.total_checks, 100);
}

#[test]
fn test_service_health_state_unhealthy() {
    let state = ServiceHealthState {
        service_id: "service-456".to_string(),
        current_status: HealthStatus::Unhealthy,
        consecutive_failures: 5,
        consecutive_successes: 0,
        last_check: Utc::now(),
        total_checks: 50,
        total_failures: 15,
    };

    assert_eq!(state.consecutive_failures, 5);
    assert_eq!(state.total_failures, 15);
}

#[test]
fn test_health_monitor_creation() {
    let config = HealthCheckConfig::default();
    let monitor = HealthMonitor::new(&config);

    assert!(monitor.is_ok());
}

#[test]
fn test_health_monitor_with_custom_config() {
    let config = HealthCheckConfig {
        check_interval_secs: 120,
        check_timeout_ms: 15000,
        failure_threshold: 10,
        success_threshold: 5,
        enable_detailed_metrics: true,
        check_methods: vec![HealthCheckMethod::Tcp, HealthCheckMethod::Ping],
    };

    let monitor = HealthMonitor::new(&config);
    assert!(monitor.is_ok());
}

#[test]
fn test_health_statistics_serialization() {
    let stats = HealthStatistics {
        total_services: 5,
        healthy_services: 4,
        unhealthy_services: 1,
        average_response_time_ms: 50.0,
        total_health_checks: 500,
        total_failures: 10,
    };

    let serialized = serde_json::to_string(&stats).unwrap();
    let deserialized: HealthStatistics = serde_json::from_str(&serialized).unwrap();

    assert_eq!(stats.total_services, deserialized.total_services);
    assert_eq!(stats.healthy_services, deserialized.healthy_services);
}

#[test]
fn test_health_check_config_serialization() {
    let config = HealthCheckConfig {
        check_interval_secs: 90,
        check_timeout_ms: 8000,
        failure_threshold: 6,
        success_threshold: 4,
        enable_detailed_metrics: true,
        check_methods: vec![HealthCheckMethod::Http, HealthCheckMethod::Tcp],
    };

    let serialized = serde_json::to_string(&config).unwrap();
    let deserialized: HealthCheckConfig = serde_json::from_str(&serialized).unwrap();

    assert_eq!(config.check_interval_secs, deserialized.check_interval_secs);
    assert_eq!(config.failure_threshold, deserialized.failure_threshold);
}

#[test]
fn test_multiple_health_check_methods() {
    let config = HealthCheckConfig {
        check_interval_secs: 30,
        check_timeout_ms: 5000,
        failure_threshold: 3,
        success_threshold: 2,
        enable_detailed_metrics: true,
        check_methods: vec![
            HealthCheckMethod::Http,
            HealthCheckMethod::Https,
            HealthCheckMethod::Tcp,
            HealthCheckMethod::Udp,
        ],
    };

    assert_eq!(config.check_methods.len(), 4);
}

#[test]
fn test_health_state_progression() {
    let mut state = ServiceHealthState {
        service_id: "service-789".to_string(),
        current_status: HealthStatus::Healthy,
        consecutive_failures: 0,
        consecutive_successes: 10,
        last_check: Utc::now(),
        total_checks: 50,
        total_failures: 0,
    };

    // Simulate a failure
    state.consecutive_failures = 1;
    state.consecutive_successes = 0;
    state.total_failures = 1;
    state.total_checks = 51;

    assert_eq!(state.consecutive_failures, 1);
    assert_eq!(state.total_failures, 1);
}

#[test]
fn test_custom_health_check_method() {
    let method = HealthCheckMethod::Custom("websocket".to_string());

    let serialized = serde_json::to_string(&method).unwrap();
    assert!(serialized.contains("Custom"));
    assert!(serialized.contains("websocket"));
}
