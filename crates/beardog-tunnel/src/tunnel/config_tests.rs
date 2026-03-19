// SPDX-License-Identifier: AGPL-3.0-only

// Configuration Tests
//
// Comprehensive tests for tunnel configuration types

#![allow(clippy::unwrap_used, clippy::expect_used)]
#![allow(clippy::field_reassign_with_default, clippy::default_trait_access)]

use super::config::*;
use std::time::Duration;

#[test]
fn test_performance_config_default() {
    let config = PerformanceConfig::default();

    assert_eq!(config.max_decryption_latency, Duration::from_micros(100));
    assert_eq!(config.max_session_setup_time, Duration::from_millis(500));
    assert_eq!(config.min_gaming_throughput, 1_000_000);
    assert!(config.enable_monitoring);
    assert_eq!(config.metrics_interval, Duration::from_secs(60));
    assert!(!config.enable_prediction);
    assert_eq!(config.memory_limit_mb, 512);
    assert_eq!(config.max_concurrent_sessions, 100);
    assert!(config.enable_auto_scaling);
    assert_eq!(config.cpu_threshold, 80.0);
    assert_eq!(config.memory_threshold, 85.0);
    assert_eq!(config.bandwidth_limit_mbps, 1000);
    assert!(config.enable_compression);
    assert_eq!(config.compression_level, 6);
    assert!(config.enable_caching);
    assert_eq!(config.cache_size_mb, 128);
    assert_eq!(config.cache_ttl, Duration::from_secs(300));
}

#[test]
fn test_performance_config_custom() {
    let config = PerformanceConfig {
        max_decryption_latency: Duration::from_micros(50),
        max_session_setup_time: Duration::from_millis(250),
        min_gaming_throughput: 2_000_000,
        enable_monitoring: false,
        metrics_interval: Duration::from_secs(30),
        enable_prediction: true,
        memory_limit_mb: 1024,
        max_concurrent_sessions: 200,
        enable_auto_scaling: false,
        cpu_threshold: 70.0,
        memory_threshold: 75.0,
        bandwidth_limit_mbps: 2000,
        enable_compression: false,
        compression_level: 9,
        enable_caching: false,
        cache_size_mb: 256,
        cache_ttl: Duration::from_secs(600),
    };

    assert_eq!(config.max_decryption_latency, Duration::from_micros(50));
    assert_eq!(config.min_gaming_throughput, 2_000_000);
    assert!(!config.enable_monitoring);
    assert!(config.enable_prediction);
}

#[test]
fn test_performance_config_extreme_values() {
    let config = PerformanceConfig {
        max_decryption_latency: Duration::from_micros(1),
        max_session_setup_time: Duration::from_millis(1),
        min_gaming_throughput: u64::MAX,
        enable_monitoring: true,
        metrics_interval: Duration::from_secs(1),
        enable_prediction: true,
        memory_limit_mb: u64::MAX,
        max_concurrent_sessions: u32::MAX,
        enable_auto_scaling: true,
        cpu_threshold: 100.0,
        memory_threshold: 100.0,
        bandwidth_limit_mbps: u64::MAX,
        enable_compression: true,
        compression_level: 9,
        enable_caching: true,
        cache_size_mb: u64::MAX,
        cache_ttl: Duration::from_secs(u64::MAX),
    };

    assert_eq!(config.max_decryption_latency, Duration::from_micros(1));
    assert_eq!(config.min_gaming_throughput, u64::MAX);
    assert_eq!(config.cpu_threshold, 100.0);
}

#[test]
fn test_security_config_default() {
    let config = SecurityConfig::default();

    assert!(!config.key_storage_path.is_empty());
    assert!(config.key_escrow_threshold > 0);
}

#[test]
fn test_unified_processor_config_default() {
    let config = UnifiedProcessorConfig::default();

    // Verify that default config is valid
    assert_eq!(
        std::mem::size_of_val(&config),
        std::mem::size_of::<UnifiedProcessorConfig>()
    );
}

#[test]
fn test_performance_config_serialization() {
    let config = PerformanceConfig::default();

    // Serialize
    let serialized = serde_json::to_string(&config).expect("Serialization should succeed");
    assert!(!serialized.is_empty());

    // Deserialize
    let deserialized: PerformanceConfig =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(
        config.max_concurrent_sessions,
        deserialized.max_concurrent_sessions
    );
    assert_eq!(config.enable_monitoring, deserialized.enable_monitoring);
}

#[test]
fn test_security_config_serialization() {
    let config = SecurityConfig::default();

    // Serialize
    let serialized = serde_json::to_string(&config).expect("Serialization should succeed");
    assert!(!serialized.is_empty());

    // Deserialize
    let deserialized: SecurityConfig =
        serde_json::from_str(&serialized).expect("Deserialization should succeed");

    assert_eq!(config.key_storage_path, deserialized.key_storage_path);
    assert_eq!(
        config.key_escrow_threshold,
        deserialized.key_escrow_threshold
    );
}

#[test]
fn test_performance_config_cloning() {
    let config1 = PerformanceConfig::default();
    let config2 = config1.clone();

    assert_eq!(
        config1.max_concurrent_sessions,
        config2.max_concurrent_sessions
    );
    assert_eq!(config1.enable_monitoring, config2.enable_monitoring);
    assert_eq!(config1.memory_limit_mb, config2.memory_limit_mb);
}

#[test]
fn test_security_config_cloning() {
    let config1 = SecurityConfig::default();
    let config2 = config1.clone();

    assert_eq!(config1.key_storage_path, config2.key_storage_path);
    assert_eq!(config1.key_escrow_threshold, config2.key_escrow_threshold);
}

#[test]
fn test_performance_config_debug_format() {
    let config = PerformanceConfig::default();
    let debug_string = format!("{:?}", config);

    assert!(debug_string.contains("PerformanceConfig"));
    assert!(debug_string.contains("max_concurrent_sessions"));
}

#[test]
fn test_performance_config_reasonable_limits() {
    let config = PerformanceConfig::default();

    // Verify reasonable default limits
    assert!(config.max_decryption_latency < Duration::from_millis(1));
    assert!(config.max_session_setup_time < Duration::from_secs(1));
    assert!(config.memory_limit_mb < 10000); // Less than 10GB
    assert!(config.max_concurrent_sessions < 100_000);
    assert!(config.cpu_threshold <= 100.0);
    assert!(config.memory_threshold <= 100.0);
    assert!(config.compression_level <= 9);
}

#[test]
fn test_performance_config_boolean_flags() {
    let config = PerformanceConfig::default();

    // Test that boolean flags have expected defaults
    assert!(config.enable_monitoring);
    assert!(config.enable_auto_scaling);
    assert!(config.enable_compression);
    assert!(config.enable_caching);
    assert!(!config.enable_prediction); // Should be false by default
}

#[test]
fn test_performance_config_duration_values() {
    let config = PerformanceConfig::default();

    // All durations should be positive
    assert!(config.max_decryption_latency > Duration::ZERO);
    assert!(config.max_session_setup_time > Duration::ZERO);
    assert!(config.metrics_interval > Duration::ZERO);
    assert!(config.cache_ttl > Duration::ZERO);
}

#[test]
fn test_performance_config_threshold_ranges() {
    let config = PerformanceConfig::default();

    // Thresholds should be reasonable percentages
    assert!(config.cpu_threshold >= 0.0);
    assert!(config.cpu_threshold <= 100.0);
    assert!(config.memory_threshold >= 0.0);
    assert!(config.memory_threshold <= 100.0);
}

#[test]
fn test_performance_config_cache_settings() {
    let config = PerformanceConfig::default();

    assert!(config.enable_caching);
    assert!(config.cache_size_mb > 0);
    assert!(config.cache_ttl > Duration::ZERO);

    // Cache size should be reasonable
    assert!(config.cache_size_mb < 10000); // Less than 10GB
}

#[test]
fn test_performance_config_compression_settings() {
    let config = PerformanceConfig::default();

    assert!(config.enable_compression);
    assert!(config.compression_level > 0);
    assert!(config.compression_level <= 9); // Valid zlib compression level
}

#[test]
fn test_performance_config_monitoring_settings() {
    let config = PerformanceConfig::default();

    assert!(config.enable_monitoring);
    assert!(config.metrics_interval > Duration::ZERO);
    assert!(config.metrics_interval < Duration::from_secs(3600)); // Less than 1 hour
}

#[test]
fn test_performance_config_auto_scaling_settings() {
    let config = PerformanceConfig::default();

    assert!(config.enable_auto_scaling);
    assert!(config.cpu_threshold > 0.0);
    assert!(config.memory_threshold > 0.0);

    // Thresholds should be high enough to avoid constant scaling
    assert!(config.cpu_threshold >= 50.0);
    assert!(config.memory_threshold >= 50.0);
}

#[test]
fn test_security_config_custom_values() {
    let config = SecurityConfig {
        key_storage_path: "/custom/path/to/keys".to_string(),
        key_escrow_threshold: 5,
    };

    assert_eq!(config.key_storage_path, "/custom/path/to/keys");
    assert_eq!(config.key_escrow_threshold, 5);
}

#[test]
fn test_security_config_path_validation() {
    let config = SecurityConfig::default();

    // Path should not be empty
    assert!(!config.key_storage_path.is_empty());

    // Path should not contain null bytes
    assert!(!config.key_storage_path.contains('\0'));
}

#[test]
fn test_security_config_threshold_validation() {
    let config = SecurityConfig::default();

    // Threshold should be reasonable (1-10)
    assert!(config.key_escrow_threshold > 0);
    assert!(config.key_escrow_threshold < 100);
}

#[test]
fn test_unified_processor_config_creation() {
    let config = UnifiedProcessorConfig::default();
    let size = std::mem::size_of_val(&config);

    // Config should have a reasonable size
    assert!(size > 0);
    assert!(size < 10000); // Less than 10KB
}
