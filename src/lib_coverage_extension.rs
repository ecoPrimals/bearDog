// SPDX-License-Identifier: AGPL-3.0-or-later
//! Coverage extension tests for root lib.rs
//!
//! Added December 8, 2025 to increase coverage from 75.17% to 90%+
//! Targets: error paths, edge cases, `ServiceInfo`, all `BearDogError` variants

#[cfg(test)]
mod root_lib_coverage_extension_tests {
    use super::super::*;
    use std::time::Duration;

    fn assert_f64_approx_eq(a: f64, b: f64) {
        const EPS: f64 = 1e-9;
        assert!((a - b).abs() < EPS, "expected {b}, got {a}");
    }

    // ============================================================================
    // FrameworkConfig Tests
    // ============================================================================

    #[test]
    fn test_framework_config_default() {
        let config = FrameworkConfig::default();

        assert_f64_approx_eq(config.confidence_level, 0.95);
        assert_eq!(config.sample_size, 1000);
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_framework_config_custom_values() {
        let config = FrameworkConfig {
            confidence_level: 0.99,
            sample_size: 5000,
            timeout: Duration::from_secs(120),
            compute_endpoint: None,
            storage_endpoint: None,
        };

        assert_f64_approx_eq(config.confidence_level, 0.99);
        assert_eq!(config.sample_size, 5000);
        assert_eq!(config.timeout, Duration::from_secs(120));
    }

    #[test]
    fn test_framework_config_clone() {
        let config1 = FrameworkConfig::default();
        let config2 = config1.clone();

        assert_f64_approx_eq(config1.confidence_level, config2.confidence_level);
        assert_eq!(config1.sample_size, config2.sample_size);
        assert_eq!(config1.timeout, config2.timeout);
    }

    #[test]
    fn test_framework_config_debug() {
        let config = FrameworkConfig::default();
        let debug_str = format!("{config:?}");

        assert!(debug_str.contains("FrameworkConfig"));
        assert!(debug_str.contains("confidence_level"));
    }

    // ============================================================================
    // FrameworkStats Tests
    // ============================================================================

    #[test]
    fn test_framework_stats_default() {
        let stats = FrameworkStats::default();

        assert_eq!(stats.services_discovered, 0);
        assert_eq!(stats.zero_copy_operations, 0);
        assert_eq!(stats.memory_ops_avoided, 0);
        assert_f64_approx_eq(stats.cache_hit_ratio, 0.0);
    }

    #[test]
    fn test_framework_stats_clone() {
        let stats1 = FrameworkStats {
            services_discovered: 5,
            zero_copy_operations: 1000,
            memory_ops_avoided: 2000,
            cache_hit_ratio: 0.95,
        };
        let stats2 = stats1.clone();

        assert_eq!(stats1.services_discovered, stats2.services_discovered);
        assert_eq!(stats1.zero_copy_operations, stats2.zero_copy_operations);
        assert_eq!(stats1.memory_ops_avoided, stats2.memory_ops_avoided);
        assert_f64_approx_eq(stats1.cache_hit_ratio, stats2.cache_hit_ratio);
    }

    #[test]
    fn test_framework_stats_debug() {
        let stats = FrameworkStats::default();
        let debug_str = format!("{stats:?}");

        assert!(debug_str.contains("FrameworkStats"));
    }

    // ============================================================================
    // ServiceInfo Tests
    // ============================================================================

    #[test]
    fn test_service_info_creation() {
        let mut metadata = HashMap::new();
        metadata.insert("version".to_string(), "1.0".to_string());

        let service = ServiceInfo {
            name: "test-service".to_string(),
            capabilities: vec!["capability1".to_string(), "capability2".to_string()],
            endpoint: "http://localhost:8080".to_string(),
            metadata,
        };

        assert_eq!(service.name, "test-service");
        assert_eq!(service.capabilities.len(), 2);
        assert_eq!(service.endpoint, "http://localhost:8080");
        assert_eq!(service.metadata.get("version"), Some(&"1.0".to_string()));
    }

    #[test]
    fn test_service_info_clone() {
        let service1 = ServiceInfo {
            name: "test".to_string(),
            capabilities: vec!["cap1".to_string()],
            endpoint: "http://test:8080".to_string(),
            metadata: HashMap::new(),
        };
        let service2 = service1.clone();

        assert_eq!(service1.name, service2.name);
        assert_eq!(service1.capabilities, service2.capabilities);
        assert_eq!(service1.endpoint, service2.endpoint);
    }

    #[test]
    fn test_service_info_debug() {
        let service = ServiceInfo {
            name: "test".to_string(),
            capabilities: vec![],
            endpoint: "http://test:8080".to_string(),
            metadata: HashMap::new(),
        };
        let debug_str = format!("{service:?}");

        assert!(debug_str.contains("ServiceInfo"));
        assert!(debug_str.contains("test"));
    }

    // ============================================================================
    // BearDogError Tests
    // ============================================================================

    #[test]
    fn test_beardog_error_configuration() {
        let error = BearDogError::Configuration("test config error".to_string());
        let error_str = format!("{error}");

        assert!(error_str.contains("Configuration error"));
        assert!(error_str.contains("test config error"));
    }

    #[test]
    fn test_beardog_error_discovery() {
        let error = BearDogError::Discovery("test discovery error".to_string());
        let error_str = format!("{error}");

        assert!(error_str.contains("Discovery error"));
        assert!(error_str.contains("test discovery error"));
    }

    #[test]
    fn test_beardog_error_performance() {
        let error = BearDogError::Performance("test performance error".to_string());
        let error_str = format!("{error}");

        assert!(error_str.contains("Performance error"));
        assert!(error_str.contains("test performance error"));
    }

    #[test]
    fn test_beardog_error_general() {
        let error = BearDogError::General("test general error".to_string());
        let error_str = format!("{error}");

        assert!(error_str.contains("BearDog error"));
        assert!(error_str.contains("test general error"));
    }

    #[test]
    fn test_beardog_error_debug() {
        let error = BearDogError::Configuration("test".to_string());
        let debug_str = format!("{error:?}");

        assert!(debug_str.contains("Configuration"));
    }

    // ============================================================================
    // BearDogFramework Tests
    // ============================================================================

    #[tokio::test]
    async fn test_framework_new() {
        let framework = BearDogFramework::new();

        assert!(framework.is_ok());
        let framework = framework.unwrap();
        assert_f64_approx_eq(framework.config.confidence_level, 0.95);
        assert_eq!(framework.stats.services_discovered, 0);
    }

    #[tokio::test]
    async fn test_framework_with_config() {
        let config = FrameworkConfig {
            confidence_level: 0.99,
            sample_size: 2000,
            timeout: Duration::from_secs(60),
            compute_endpoint: None,
            storage_endpoint: None,
        };

        let framework = BearDogFramework::with_config(config);

        assert!(framework.is_ok());
        let framework = framework.unwrap();
        assert_f64_approx_eq(framework.config.confidence_level, 0.99);
        assert_eq!(framework.config.sample_size, 2000);
    }

    #[tokio::test]
    async fn test_discover_services_missing_compute_endpoint() {
        let config = FrameworkConfig {
            confidence_level: 0.95,
            sample_size: 1000,
            timeout: Duration::from_secs(30),
            compute_endpoint: None, // Missing!
            storage_endpoint: Some("http://storage:8081".to_string()),
        };

        let mut framework = BearDogFramework::with_config(config).unwrap();
        let result = framework.discover_services();

        assert!(result.is_err());
        match result {
            Err(BearDogError::Configuration(msg)) => {
                assert!(msg.contains("BEARDOG_COMPUTE_ENDPOINT"));
            }
            _ => panic!("Expected Configuration error"),
        }
    }

    #[tokio::test]
    async fn test_discover_services_missing_storage_endpoint() {
        let config = FrameworkConfig {
            confidence_level: 0.95,
            sample_size: 1000,
            timeout: Duration::from_secs(30),
            compute_endpoint: Some("http://test:8080".to_string()),
            storage_endpoint: None, // Missing!
        };

        let mut framework = BearDogFramework::with_config(config).unwrap();
        let result = framework.discover_services();

        // Should error due to missing storage endpoint
        assert!(result.is_err());
        if let Err(BearDogError::Configuration(msg)) = result {
            assert!(msg.contains("BEARDOG_STORAGE_ENDPOINT") || msg.contains("storage"));
        } else {
            panic!("Expected Configuration error about storage endpoint");
        }
    }

    #[tokio::test]
    async fn test_discover_services_success() {
        let config = FrameworkConfig {
            confidence_level: 0.95,
            sample_size: 1000,
            timeout: Duration::from_secs(30),
            compute_endpoint: Some("http://compute-test:8080".to_string()),
            storage_endpoint: Some("http://storage-test:8081".to_string()),
        };

        let mut framework = BearDogFramework::with_config(config).unwrap();
        let result = framework.discover_services();

        assert!(result.is_ok());
        let services = result.unwrap();
        assert_eq!(services.len(), 2);
        assert_eq!(framework.stats.services_discovered, 2);

        // Check first service (compute)
        assert_eq!(services[0].name, "compute-service");
        assert_eq!(services[0].endpoint, "http://compute-test:8080");
        assert_eq!(services[0].capabilities.len(), 2);
        assert!(
            services[0]
                .capabilities
                .contains(&"ai-processing".to_string())
        );

        // Check second service (storage)
        assert_eq!(services[1].name, "storage-service");
        assert_eq!(services[1].endpoint, "http://storage-test:8081");
        assert_eq!(services[1].capabilities.len(), 2);
        assert!(
            services[1]
                .capabilities
                .contains(&"high-throughput".to_string())
        );
    }

    #[tokio::test]
    async fn test_demonstrate_zero_copy_performance() {
        let mut framework = BearDogFramework::new().unwrap();

        assert_eq!(framework.stats.zero_copy_operations, 0);
        assert_eq!(framework.stats.memory_ops_avoided, 0);
        assert_f64_approx_eq(framework.stats.cache_hit_ratio, 0.0);

        let result = framework.demonstrate_zero_copy_performance();

        assert!(result.is_ok());
        assert_eq!(framework.stats.zero_copy_operations, 1000);
        assert_eq!(framework.stats.memory_ops_avoided, 1000);
        assert_f64_approx_eq(framework.stats.cache_hit_ratio, 0.95);
    }

    #[tokio::test]
    async fn test_get_stats() {
        let mut framework = BearDogFramework::new().unwrap();

        let stats = framework.get_stats();
        assert_eq!(stats.services_discovered, 0);

        framework.stats.services_discovered = 5;
        let stats = framework.get_stats();
        assert_eq!(stats.services_discovered, 5);
    }

    #[tokio::test]
    async fn test_config_accessor() {
        let framework = BearDogFramework::new().unwrap();

        let config = framework.config();
        assert_f64_approx_eq(config.confidence_level, 0.95);
        assert_eq!(config.sample_size, 1000);
    }

    #[tokio::test]
    async fn test_reset_stats() {
        let mut framework = BearDogFramework::new().unwrap();

        // Set some stats
        framework.stats.services_discovered = 10;
        framework.stats.zero_copy_operations = 5000;
        framework.stats.memory_ops_avoided = 3000;
        framework.stats.cache_hit_ratio = 0.85;

        assert_eq!(framework.stats.services_discovered, 10);

        framework.reset_stats();

        assert_eq!(framework.stats.services_discovered, 0);
        assert_eq!(framework.stats.zero_copy_operations, 0);
        assert_eq!(framework.stats.memory_ops_avoided, 0);
        assert_f64_approx_eq(framework.stats.cache_hit_ratio, 0.0);
    }

    #[tokio::test]
    async fn test_multiple_operations() {
        let config = FrameworkConfig {
            confidence_level: 0.95,
            sample_size: 1000,
            timeout: Duration::from_secs(30),
            compute_endpoint: Some("http://test1:8080".to_string()),
            storage_endpoint: Some("http://test2:8081".to_string()),
        };

        let mut framework = BearDogFramework::with_config(config).unwrap();

        // Discover services
        let services = framework.discover_services().unwrap();
        assert_eq!(services.len(), 2);
        assert_eq!(framework.stats.services_discovered, 2);

        // Demonstrate performance
        framework.demonstrate_zero_copy_performance().unwrap();
        assert_eq!(framework.stats.zero_copy_operations, 1000);

        // Check combined stats
        assert_eq!(framework.stats.services_discovered, 2);
        assert_eq!(framework.stats.zero_copy_operations, 1000);
        assert_f64_approx_eq(framework.stats.cache_hit_ratio, 0.95);
    }

    #[tokio::test]
    async fn test_framework_with_high_confidence() {
        let config = FrameworkConfig {
            confidence_level: 0.999,
            sample_size: 10000,
            timeout: Duration::from_secs(300),
            compute_endpoint: None,
            storage_endpoint: None,
        };

        let framework = BearDogFramework::with_config(config).unwrap();

        assert_f64_approx_eq(framework.config.confidence_level, 0.999);
        assert_eq!(framework.config.sample_size, 10000);
        assert_eq!(framework.config.timeout, Duration::from_secs(300));
    }

    #[tokio::test]
    async fn test_framework_with_low_confidence() {
        let config = FrameworkConfig {
            confidence_level: 0.5,
            sample_size: 100,
            timeout: Duration::from_secs(5),
            compute_endpoint: None,
            storage_endpoint: None,
        };

        let framework = BearDogFramework::with_config(config).unwrap();

        assert_f64_approx_eq(framework.config.confidence_level, 0.5);
        assert_eq!(framework.config.sample_size, 100);
        assert_eq!(framework.config.timeout, Duration::from_secs(5));
    }

    #[tokio::test]
    async fn test_service_metadata() {
        let config = FrameworkConfig {
            confidence_level: 0.95,
            sample_size: 1000,
            timeout: Duration::from_secs(30),
            compute_endpoint: Some("http://compute:9090".to_string()),
            storage_endpoint: Some("http://storage:9091".to_string()),
        };

        let mut framework = BearDogFramework::with_config(config).unwrap();
        let services = framework.discover_services().unwrap();

        // Check compute service metadata
        assert_eq!(
            services[0].metadata.get("type"),
            Some(&"compute".to_string())
        );

        // Check storage service metadata
        assert_eq!(
            services[1].metadata.get("type"),
            Some(&"storage".to_string())
        );
    }

    #[tokio::test]
    async fn test_cumulative_zero_copy_operations() {
        let mut framework = BearDogFramework::new().unwrap();

        // Perform operations multiple times
        framework.demonstrate_zero_copy_performance().unwrap();
        assert_eq!(framework.stats.zero_copy_operations, 1000);

        framework.demonstrate_zero_copy_performance().unwrap();
        assert_eq!(framework.stats.zero_copy_operations, 2000);

        framework.demonstrate_zero_copy_performance().unwrap();
        assert_eq!(framework.stats.zero_copy_operations, 3000);
    }

    #[tokio::test]
    async fn test_reset_after_operations() {
        let config = FrameworkConfig {
            confidence_level: 0.95,
            sample_size: 1000,
            timeout: Duration::from_secs(30),
            compute_endpoint: Some("http://test:8080".to_string()),
            storage_endpoint: Some("http://test:8081".to_string()),
        };

        let mut framework = BearDogFramework::with_config(config).unwrap();

        // Perform operations
        framework.discover_services().unwrap();
        framework.demonstrate_zero_copy_performance().unwrap();

        assert!(framework.stats.services_discovered > 0);
        assert!(framework.stats.zero_copy_operations > 0);

        // Reset
        framework.reset_stats();

        assert_eq!(framework.stats.services_discovered, 0);
        assert_eq!(framework.stats.zero_copy_operations, 0);
    }

    #[test]
    fn test_service_info_preserves_multiple_metadata_keys() {
        let mut metadata = HashMap::new();
        metadata.insert("region".to_string(), "us-west".to_string());
        metadata.insert("tier".to_string(), "prod".to_string());

        let service = ServiceInfo {
            name: "multi-meta".to_string(),
            capabilities: vec!["read".to_string()],
            endpoint: "https://svc.local:8443".to_string(),
            metadata,
        };

        assert_eq!(service.metadata.get("region"), Some(&"us-west".to_string()));
        assert_eq!(service.metadata.get("tier"), Some(&"prod".to_string()));
    }

    #[test]
    fn test_framework_stats_equality_for_zero_defaults() {
        let a = FrameworkStats::default();
        let b = FrameworkStats::default();
        assert_eq!(a.services_discovered, b.services_discovered);
        assert_f64_approx_eq(a.cache_hit_ratio, b.cache_hit_ratio);
    }
}
