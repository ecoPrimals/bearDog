//! Comprehensive tests for SecuritySentinel
//!
//! These tests cover security monitoring lifecycle, configuration,
//! and state management.

#![cfg(test)]
#![allow(clippy::unwrap_used, clippy::expect_used)]

use super::*;
use std::sync::Arc;

#[test]
fn test_security_sentinel_creation() {
    let config = SecuritySentinelConfig::default();
    let sentinel = SecuritySentinel::new(config);
    assert_eq!(sentinel.get_event_count(), 0);
    assert!(!sentinel.is_monitoring_active());
}

#[test]
fn test_security_sentinel_with_custom_config() {
    let config = SecuritySentinelConfig {
        enabled: true,
        max_events: 5000,
        retention_hours: 12,
        log_events: false,
        alert_threshold: 5,
        real_time_monitoring: true,
        monitoring_interval_seconds: 30,
    };

    let sentinel = SecuritySentinel::new(config);
    assert_eq!(sentinel.get_event_count(), 0);
}

#[test]
fn test_security_sentinel_default_config() {
    let config = SecuritySentinelConfig::default();

    assert!(config.enabled);
    assert_eq!(config.max_events, 10000);
    assert_eq!(config.retention_hours, 24);
    assert!(config.log_events);
    assert_eq!(config.alert_threshold, 10);
    assert!(config.real_time_monitoring);
    assert_eq!(config.monitoring_interval_seconds, 60);
}

#[test]
fn test_security_sentinel_start_monitoring() {
    let sentinel = SecuritySentinel::default();
    assert!(!sentinel.is_monitoring_active());

    let result = sentinel.start_monitoring();
    assert!(result.is_ok());
    assert!(sentinel.is_monitoring_active());
}

#[test]
fn test_security_sentinel_stop_monitoring() {
    let sentinel = SecuritySentinel::default();

    // Start then stop
    sentinel.start_monitoring().unwrap();
    assert!(sentinel.is_monitoring_active());

    let result = sentinel.stop_monitoring();
    assert!(result.is_ok());
    assert!(!sentinel.is_monitoring_active());
}

#[test]
fn test_security_sentinel_stats_default() {
    let stats = SecuritySentinelStats::default();

    assert_eq!(stats.total_events, 0);
    assert_eq!(stats.auth_failures, 0);
    assert_eq!(stats.access_violations, 0);
    assert_eq!(stats.compliance_violations, 0);
    assert_eq!(stats.suspicious_activities, 0);
    assert_eq!(stats.blocked_requests, 0);
    assert!(stats.last_event_time.is_none());
}

#[test]
fn test_config_enabled_flag() {
    let enabled_config = SecuritySentinelConfig {
        enabled: true,
        ..Default::default()
    };

    let disabled_config = SecuritySentinelConfig {
        enabled: false,
        ..Default::default()
    };

    let enabled_sentinel = SecuritySentinel::new(enabled_config);
    let disabled_sentinel = SecuritySentinel::new(disabled_config);

    // Both should be created successfully
    assert_eq!(enabled_sentinel.get_event_count(), 0);
    assert_eq!(disabled_sentinel.get_event_count(), 0);
}

#[test]
fn test_max_events_configuration() {
    let configs = vec![10, 100, 1000, 10000];

    for max_events in configs {
        let config = SecuritySentinelConfig {
            max_events,
            ..Default::default()
        };

        let sentinel = SecuritySentinel::new(config);
        assert_eq!(sentinel.get_event_count(), 0);
    }
}

#[test]
fn test_retention_hours_configuration() {
    let configs = vec![1, 24, 48, 168]; // 1 hour to 1 week

    for retention_hours in configs {
        let config = SecuritySentinelConfig {
            retention_hours,
            ..Default::default()
        };

        let sentinel = SecuritySentinel::new(config);
        assert!(!sentinel.is_monitoring_active());
    }
}

#[test]
fn test_alert_threshold_configuration() {
    let thresholds = vec![1, 5, 10, 50, 100];

    for alert_threshold in thresholds {
        let config = SecuritySentinelConfig {
            alert_threshold,
            ..Default::default()
        };

        let sentinel = SecuritySentinel::new(config);
        assert_eq!(sentinel.get_event_count(), 0);
    }
}

#[test]
fn test_monitoring_interval_configuration() {
    let intervals = vec![10, 30, 60, 300]; // 10s to 5 min

    for monitoring_interval_seconds in intervals {
        let config = SecuritySentinelConfig {
            monitoring_interval_seconds,
            ..Default::default()
        };

        let sentinel = SecuritySentinel::new(config);
        assert!(!sentinel.is_monitoring_active());
    }
}

#[test]
fn test_log_events_flag() {
    let log_enabled = SecuritySentinelConfig {
        log_events: true,
        ..Default::default()
    };

    let log_disabled = SecuritySentinelConfig {
        log_events: false,
        ..Default::default()
    };

    let sentinel1 = SecuritySentinel::new(log_enabled);
    let sentinel2 = SecuritySentinel::new(log_disabled);

    assert_eq!(sentinel1.get_event_count(), 0);
    assert_eq!(sentinel2.get_event_count(), 0);
}

#[test]
fn test_real_time_monitoring_flag() {
    let realtime_config = SecuritySentinelConfig {
        real_time_monitoring: true,
        ..Default::default()
    };

    let batch_config = SecuritySentinelConfig {
        real_time_monitoring: false,
        ..Default::default()
    };

    let sentinel1 = SecuritySentinel::new(realtime_config);
    let sentinel2 = SecuritySentinel::new(batch_config);

    assert!(!sentinel1.is_monitoring_active());
    assert!(!sentinel2.is_monitoring_active());
}

#[test]
fn test_monitoring_state_transitions() {
    let sentinel = SecuritySentinel::default();

    // Initial state
    assert!(!sentinel.is_monitoring_active());

    // Start monitoring
    sentinel.start_monitoring().unwrap();
    assert!(sentinel.is_monitoring_active());

    // Stop monitoring
    sentinel.stop_monitoring().unwrap();
    assert!(!sentinel.is_monitoring_active());
}

#[test]
fn test_multiple_start_stop_cycles() {
    let sentinel = SecuritySentinel::default();

    for _ in 0..5 {
        assert!(sentinel.start_monitoring().is_ok());
        assert!(sentinel.is_monitoring_active());

        assert!(sentinel.stop_monitoring().is_ok());
        assert!(!sentinel.is_monitoring_active());
    }
}

#[test]
fn test_event_count_initialization() {
    let sentinel = SecuritySentinel::default();
    assert_eq!(sentinel.get_event_count(), 0);
}

#[test]
fn test_security_config_cloning() {
    let config1 = SecuritySentinelConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.enabled, config2.enabled);
    assert_eq!(config1.max_events, config2.max_events);
    assert_eq!(config1.retention_hours, config2.retention_hours);
    assert_eq!(config1.log_events, config2.log_events);
    assert_eq!(config1.alert_threshold, config2.alert_threshold);
    assert_eq!(config1.real_time_monitoring, config2.real_time_monitoring);
    assert_eq!(
        config1.monitoring_interval_seconds,
        config2.monitoring_interval_seconds
    );
}

#[test]
fn test_security_stats_cloning() {
    let stats1 = SecuritySentinelStats::default();
    let stats2 = stats1.clone();

    assert_eq!(stats1.total_events, stats2.total_events);
    assert_eq!(stats1.auth_failures, stats2.auth_failures);
    assert_eq!(stats1.access_violations, stats2.access_violations);
    assert_eq!(stats1.compliance_violations, stats2.compliance_violations);
    assert_eq!(stats1.suspicious_activities, stats2.suspicious_activities);
    assert_eq!(stats1.blocked_requests, stats2.blocked_requests);
}

#[test]
fn test_default_trait_implementation() {
    let sentinel1 = SecuritySentinel::default();
    let sentinel2 = SecuritySentinel::new(SecuritySentinelConfig::default());

    // Both should start with same state
    assert_eq!(sentinel1.get_event_count(), sentinel2.get_event_count());
    assert_eq!(
        sentinel1.is_monitoring_active(),
        sentinel2.is_monitoring_active()
    );
}

#[test]
fn test_concurrent_sentinel_instances() {
    let sentinel1 = SecuritySentinel::default();
    let sentinel2 = SecuritySentinel::default();

    // Start only sentinel1
    sentinel1.start_monitoring().unwrap();

    assert!(sentinel1.is_monitoring_active());
    assert!(!sentinel2.is_monitoring_active());
}

#[test]
fn test_minimal_config() {
    let config = SecuritySentinelConfig {
        enabled: false,
        max_events: 0,
        retention_hours: 0,
        log_events: false,
        alert_threshold: 0,
        real_time_monitoring: false,
        monitoring_interval_seconds: 0,
    };

    // Should still be able to create sentinel with minimal config
    let sentinel = SecuritySentinel::new(config);
    assert_eq!(sentinel.get_event_count(), 0);
    assert!(!sentinel.is_monitoring_active());
}

#[test]
fn test_maximal_config() {
    let config = SecuritySentinelConfig {
        enabled: true,
        max_events: 1_000_000,
        retention_hours: 8760, // 1 year
        log_events: true,
        alert_threshold: 1000,
        real_time_monitoring: true,
        monitoring_interval_seconds: 1,
    };

    let sentinel = SecuritySentinel::new(config);
    assert_eq!(sentinel.get_event_count(), 0);
}

#[test]
fn test_sentinel_thread_safe() {
    let sentinel = Arc::new(SecuritySentinel::default());
    let sentinel_clone = Arc::clone(&sentinel);

    std::thread::spawn(move || {
        sentinel_clone.start_monitoring().unwrap();
    })
    .join()
    .unwrap();

    // Should be able to check state from original reference
    assert!(sentinel.is_monitoring_active());
}

#[test]
fn test_start_monitoring_idempotent() {
    let sentinel = SecuritySentinel::default();

    // Multiple starts should all succeed
    assert!(sentinel.start_monitoring().is_ok());
    assert!(sentinel.start_monitoring().is_ok());
    assert!(sentinel.start_monitoring().is_ok());

    assert!(sentinel.is_monitoring_active());
}

#[test]
fn test_stop_monitoring_idempotent() {
    let sentinel = SecuritySentinel::default();

    // Stop without starting should succeed
    assert!(sentinel.stop_monitoring().is_ok());
    assert!(sentinel.stop_monitoring().is_ok());

    assert!(!sentinel.is_monitoring_active());
}

#[test]
fn test_stats_timestamp_initialization() {
    let stats = SecuritySentinelStats::default();

    // monitoring_start_time should be set to current time (approximately)
    let now = Utc::now();
    let diff = now.signed_duration_since(stats.monitoring_start_time);

    // Should be within a few seconds
    assert!(diff.num_seconds().abs() < 5);
}

#[test]
fn test_config_serialization() {
    let config = SecuritySentinelConfig::default();

    // Test that config can be serialized (required trait bounds)
    let json = serde_json::to_string(&config);
    assert!(json.is_ok());
}

#[test]
fn test_stats_serialization() {
    let stats = SecuritySentinelStats::default();

    // Test that stats can be serialized
    let json = serde_json::to_string(&stats);
    assert!(json.is_ok());
}

#[test]
fn test_config_deserialization() {
    let config = SecuritySentinelConfig::default();
    let json = serde_json::to_string(&config).unwrap();

    // Test that config can be deserialized
    let deserialized: Result<SecuritySentinelConfig, _> = serde_json::from_str(&json);
    assert!(deserialized.is_ok());
}

#[test]
fn test_stats_deserialization() {
    let stats = SecuritySentinelStats::default();
    let json = serde_json::to_string(&stats).unwrap();

    // Test that stats can be deserialized
    let deserialized: Result<SecuritySentinelStats, _> = serde_json::from_str(&json);
    assert!(deserialized.is_ok());
}
