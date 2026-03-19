// SPDX-License-Identifier: AGPL-3.0-only

// Comprehensive Metrics System Tests
//
// Extensive test coverage for production metrics collection and reporting

use super::metrics::*;
use std::collections::HashMap;

#[test]
fn test_metrics_config_default() {
    let config = MetricsConfig::default();
    assert_eq!(config.collection_interval_seconds, 30);
    assert_eq!(config.retention_count, 1000);
}

#[test]
fn test_metrics_config_custom() {
    let config = MetricsConfig {
        collection_interval_seconds: 60,
        retention_count: 5000,
        enable_streaming: true,
        batch_size: 200,
        labels: HashMap::new(),
    };

    assert!(config.enable_streaming);
    assert_eq!(config.batch_size, 200);
    assert_eq!(config.retention_count, 5000);
}

#[test]
fn test_metrics_config_with_labels() {
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let mut labels = HashMap::new();
    labels.insert("environment".to_string(), "production".to_string());
    labels.insert("region".to_string(), "us-west-2".to_string());

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let config = MetricsConfig {
        collection_interval_seconds: 30,
        retention_count: 1000,
        enable_streaming: true,
        batch_size: 100,
        labels,
    };

    assert_eq!(config.labels.len(), 2);
    assert_eq!(config.labels.get("environment").unwrap(), "production");
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_current_metrics_creation() {
    let metrics = CurrentMetrics {
        timestamp: chrono::Utc::now(),
        cpu_usage_percent: 45.5,
        memory_usage_percent: 60.0,
        disk_usage_percent: 70.0,
        network_throughput_bps: 1_000_000,
        active_connections: 150,
        latency_ms: 25.5,
        error_rate_percent: 0.01,
        custom_metrics: HashMap::new(),
    };

    assert_eq!(metrics.cpu_usage_percent, 45.5);
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    assert_eq!(metrics.active_connections, 150);
}

#[test]
fn test_current_metrics_with_custom() {
    let mut custom_metrics = HashMap::new();
    custom_metrics.insert("cache_hit_rate".to_string(), 0.95);
    custom_metrics.insert("queue_depth".to_string(), 42.0);

    let metrics = CurrentMetrics {
        timestamp: chrono::Utc::now(),
        cpu_usage_percent: 30.0,
        memory_usage_percent: 50.0,
        disk_usage_percent: 40.0,
        network_throughput_bps: 500_000,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        active_connections: 100,
        latency_ms: 15.0,
        error_rate_percent: 0.0,
        custom_metrics,
    };

    assert_eq!(metrics.custom_metrics.len(), 2);
    assert_eq!(metrics.custom_metrics.get("cache_hit_rate").unwrap(), &0.95);
}

#[test]
fn test_production_metrics_collector_new() {
    let config = MetricsConfig::default();
    let _collector = ProductionMetricsCollector::new(config);
    // Collector is created successfully
    // Note: No public method to check empty state directly
    // Collector creation itself is the test - if we got here it succeeded
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_metrics_collector_lifecycle() {
    let config = MetricsConfig {
        collection_interval_seconds: 60,
        retention_count: 2000,
        enable_streaming: false,
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        batch_size: 50,
        labels: HashMap::new(),
    };

    let mut collector = ProductionMetricsCollector::new(config);
    assert!(collector.start_collection().is_ok());
    assert!(collector.stop_collection().is_ok());
}

#[test]
fn test_metrics_config_serialization() {
    let config = MetricsConfig::default();
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    let json = serde_json::to_string(&config).unwrap();
    let deserialized: MetricsConfig = serde_json::from_str(&json).unwrap();

    assert_eq!(
        config.collection_interval_seconds,
        deserialized.collection_interval_seconds
    );
    assert_eq!(config.retention_count, deserialized.retention_count);
}

// TEST_CATEGORY: unit
// TEST_DOMAIN: types
// TEST_PRIORITY: normal
#[test]
fn test_current_metrics_serialization() {
    let metrics = CurrentMetrics {
        timestamp: chrono::Utc::now(),
        cpu_usage_percent: 50.0,
        memory_usage_percent: 60.0,
        disk_usage_percent: 70.0,
        network_throughput_bps: 1_000_000,
        active_connections: 200,
        latency_ms: 30.0,
        error_rate_percent: 0.1,
        custom_metrics: HashMap::new(),
    };

    let json = serde_json::to_string(&metrics).unwrap();
    assert!(json.contains("cpu_usage_percent"));
}
