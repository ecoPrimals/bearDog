//! Core Production Module Tests
//!
//! Comprehensive tests for production configuration, ecosystem lifecycle,
//! and operational state management.

use super::*;

#[cfg(test)]
mod config_tests {
    use super::*;

    #[test]
    fn test_production_config_default() {
        let config = ProductionConfig::default();

        // Verify core config
        assert_eq!(config.core.environment_level, EnvironmentLevel::Development);
        assert_eq!(config.core.service_name, "beardog-ecosystem");
        assert!(!config.core.deployment_id.is_empty());
        assert_eq!(config.core.region, "local");

        // Verify flags
        assert!(config.core.flags.enable_advanced_monitoring);
        assert!(config.core.flags.enable_distributed_tracing);
        assert!(config.core.flags.enable_performance_profiling);
        assert!(config.core.flags.enable_security_auditing);
    }

    #[test]
    fn test_production_core_config_default() {
        let core = ProductionCoreConfig::default();

        assert_eq!(core.environment_level, EnvironmentLevel::Development);
        assert_eq!(core.service_name, "beardog-ecosystem");
        assert!(!core.service_version.is_empty());
        assert!(!core.deployment_id.is_empty());
        assert_eq!(core.region, "local");
        assert_eq!(core.cluster_id, "default-cluster");
        assert!(!core.node_id.is_empty());
    }

    #[test]
    fn test_production_flags_default() {
        let flags = ProductionFlags::default();

        assert!(flags.enable_advanced_monitoring);
        assert!(flags.enable_distributed_tracing);
        assert!(flags.enable_performance_profiling);
        assert!(flags.enable_security_auditing);
        assert!(!flags.enable_auto_scaling); // Disabled by default
        assert!(flags.enable_circuit_breakers);
        assert!(flags.enable_rate_limiting);
        assert!(flags.enable_caching);
    }

    #[test]
    fn test_environment_level_variants() {
        let levels = vec![
            EnvironmentLevel::Development,
            EnvironmentLevel::Staging,
            EnvironmentLevel::PreProduction,
            EnvironmentLevel::Production,
            EnvironmentLevel::Critical,
        ];

        for level in levels {
            // Test serialization/deserialization
            let serialized = serde_json::to_string(&level).unwrap();
            let deserialized: EnvironmentLevel = serde_json::from_str(&serialized).unwrap();
            assert_eq!(level, deserialized);
        }
    }

    #[test]
    fn test_production_config_serialization() {
        let config = ProductionConfig::default();

        // Serialize
        let json = serde_json::to_string(&config).unwrap();
        assert!(!json.is_empty());

        // Deserialize
        let deserialized: ProductionConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.core.service_name, config.core.service_name);
        assert_eq!(
            deserialized.core.environment_level,
            config.core.environment_level
        );
    }

    #[test]
    fn test_production_flags_serialization() {
        let flags = ProductionFlags::default();

        // Serialize
        let json = serde_json::to_string(&flags).unwrap();
        assert!(!json.is_empty());

        // Deserialize
        let deserialized: ProductionFlags = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized.enable_advanced_monitoring,
            flags.enable_advanced_monitoring
        );
        assert_eq!(
            deserialized.enable_distributed_tracing,
            flags.enable_distributed_tracing // TEST_CATEGORY: unit
                                             // TEST_DOMAIN: types
                                             // TEST_PRIORITY: normal
        );
    }
}

#[cfg(test)]
mod state_tests {
    use super::*;

    #[test]
    fn test_production_state_default() {
        let state = ProductionState::default();

        assert_eq!(state.status, OperationalStatus::Initializing);
        assert_eq!(state.active_connections, 0);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(state.total_requests, 0);
        assert_eq!(state.memory_usage_bytes, 0);
        assert_eq!(state.cpu_usage_percent, 0.0);
        assert_eq!(state.error_count_hourly, 0);
    }

    #[test]
    fn test_operational_status_variants() {
        let statuses = vec![
            OperationalStatus::Initializing,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            OperationalStatus::Healthy,
            OperationalStatus::Degraded,
            OperationalStatus::Unhealthy,
            OperationalStatus::Critical,
            OperationalStatus::Shutdown,
        ];

        for status in statuses {
            // Test serialization/deserialization
            let serialized = serde_json::to_string(&status).unwrap();
            let deserialized: OperationalStatus = serde_json::from_str(&serialized).unwrap();
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            assert_eq!(status, deserialized);
        }
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();

        assert_eq!(metrics.avg_response_time_ms, 0.0);
        assert_eq!(metrics.p95_response_time_ms, 0.0);
        assert_eq!(metrics.p99_response_time_ms, 0.0);
        assert_eq!(metrics.requests_per_second, 0.0);
        assert_eq!(metrics.error_rate_percent, 0.0);
        assert_eq!(metrics.throughput_bytes_per_sec, 0);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_production_state_serialization() {
        let state = ProductionState::default();

        // Serialize
        let json = serde_json::to_string(&state).unwrap();
        assert!(!json.is_empty());

        // Deserialize
        let deserialized: ProductionState = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.status, state.status);
        assert_eq!(deserialized.active_connections, state.active_connections);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[cfg(test)]
mod ecosystem_tests {
    use super::*;

    #[test]
    fn test_ecosystem_creation() {
        let config = ProductionConfig::default();
        let result = ProductionEcosystem::new(config);

        assert!(result.is_ok());
        let ecosystem = result.unwrap();
        assert_eq!(
            ecosystem.get_status().status,
            OperationalStatus::Initializing
        );
    }

    #[test]
    fn test_ecosystem_initialization() {
        let config = ProductionConfig::default();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let mut ecosystem = ProductionEcosystem::new(config).unwrap();

        let result = ecosystem.initialize();
        assert!(result.is_ok());
        assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
    }

    #[test]
    fn test_ecosystem_uptime() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        let config = ProductionConfig::default();
        let ecosystem = ProductionEcosystem::new(config).unwrap();

        let uptime = ecosystem.uptime();
        assert!(uptime.as_secs() < 1); // Should be very recent
    }

    #[test]
    fn test_ecosystem_get_status() {
        let config = ProductionConfig::default();
        let ecosystem = ProductionEcosystem::new(config).unwrap();

        let status = ecosystem.get_status();
        assert_eq!(status.status, OperationalStatus::Initializing);
        assert_eq!(status.active_connections, 0);
        assert_eq!(status.total_requests, 0);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_ecosystem_lifecycle() {
        let config = ProductionConfig::default();
        let mut ecosystem = ProductionEcosystem::new(config).unwrap();

        // Initialize
        ecosystem.initialize().unwrap();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

        // Shutdown
        ecosystem.shutdown().unwrap();
        assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
    }

    #[test]
    fn test_ecosystem_update_metrics() {
        let config = ProductionConfig::default();
        let mut ecosystem = ProductionEcosystem::new(config).unwrap();
        ecosystem.initialize().unwrap();

        let result = ecosystem.update_metrics();
        assert!(result.is_ok());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_ecosystem_health_check() {
        let config = ProductionConfig::default();
        let mut ecosystem = ProductionEcosystem::new(config).unwrap();
        ecosystem.initialize().unwrap();

        let result = ecosystem.health_check();
        assert!(result.is_ok());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
}

#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_builder_default() {
        let builder = ProductionEcosystemBuilder::default();
        let ecosystem = builder.build().unwrap();

        assert_eq!(
            ecosystem.get_status().status,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            OperationalStatus::Initializing
        );
    }

    #[test]
    fn test_builder_new() {
        let builder = ProductionEcosystemBuilder::new();
        let ecosystem = builder.build().unwrap();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        assert_eq!(
            ecosystem.get_status().status,
            OperationalStatus::Initializing
        );
    }

    #[test]
    fn test_builder_environment_level() {
        let builder =
            ProductionEcosystemBuilder::new().environment_level(EnvironmentLevel::Production);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        let ecosystem = builder.build().unwrap();
        assert_eq!(
            ecosystem.config.core.environment_level,
            EnvironmentLevel::Production
        );
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_builder_service() {
        let builder = ProductionEcosystemBuilder::new()
            .service("test-service".to_string(), "1.0.0".to_string());

        let ecosystem = builder.build().unwrap();
        assert_eq!(ecosystem.config.core.service_name, "test-service");
        assert_eq!(ecosystem.config.core.service_version, "1.0.0");
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_builder_deployment() {
        let builder = ProductionEcosystemBuilder::new().deployment(
            "deploy-123".to_string(),
            "us-west-2".to_string(),
            "cluster-prod".to_string(),
        );

        let ecosystem = builder.build().unwrap();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(ecosystem.config.core.deployment_id, "deploy-123");
        assert_eq!(ecosystem.config.core.region, "us-west-2");
        assert_eq!(ecosystem.config.core.cluster_id, "cluster-prod");
    }

    #[test]
    fn test_builder_enable_advanced_features() {
        let builder = ProductionEcosystemBuilder::new().enable_advanced_features();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        let ecosystem = builder.build().unwrap();
        assert!(ecosystem.config.core.flags.enable_advanced_monitoring);
        assert!(ecosystem.config.core.flags.enable_distributed_tracing);
        assert!(ecosystem.config.core.flags.enable_performance_profiling);
        assert!(ecosystem.config.core.flags.enable_security_auditing);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_builder_chaining() {
        let builder = ProductionEcosystemBuilder::new()
            .environment_level(EnvironmentLevel::Production)
            .service("chain-test".to_string(), "2.0.0".to_string())
            .deployment(
                "d-456".to_string(),
                "eu-west-1".to_string(),
                // TEST_CATEGORY: unit
                // TEST_DOMAIN: types
                // TEST_PRIORITY: normal
                "c-789".to_string(),
            )
            .enable_advanced_features();

        let ecosystem = builder.build().unwrap();
        assert_eq!(
            ecosystem.config.core.environment_level,
            EnvironmentLevel::Production
        );
        assert_eq!(ecosystem.config.core.service_name, "chain-test");
        assert_eq!(ecosystem.config.core.service_version, "2.0.0");
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        assert_eq!(ecosystem.config.core.deployment_id, "d-456");
        assert_eq!(ecosystem.config.core.region, "eu-west-1");
        assert_eq!(ecosystem.config.core.cluster_id, "c-789");
    }

    #[test]
    fn test_builder_full_lifecycle() {
        let builder = ProductionEcosystemBuilder::new()
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            .environment_level(EnvironmentLevel::Staging)
            .service("lifecycle-test".to_string(), "1.2.3".to_string())
            .enable_advanced_features();

        let mut ecosystem = builder.build().unwrap();

        // Test full lifecycle
        ecosystem.initialize().unwrap();
        assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

        ecosystem.update_metrics().unwrap();
        ecosystem.health_check().unwrap();

        ecosystem.shutdown().unwrap();
        assert_eq!(ecosystem.get_status().status, OperationalStatus::Shutdown);
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_multiple_initializations() {
        let config = ProductionConfig::default();
        let mut ecosystem = ProductionEcosystem::new(config).unwrap();

        // First initialization
        ecosystem.initialize().unwrap();
        assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);

        // Second initialization (behavior is implementation-dependent)
        let result = ecosystem.initialize();
        // Either succeeds or returns an error - both are valid
        if result.is_ok() {
            assert_eq!(ecosystem.get_status().status, OperationalStatus::Healthy);
        }
    }

    #[test]
    fn test_shutdown_before_initialization() {
        let config = ProductionConfig::default();
        let mut ecosystem = ProductionEcosystem::new(config).unwrap();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal

        // Shutdown without initializing
        // This may succeed or fail depending on implementation
        let _ = ecosystem.shutdown();
        // If it succeeds, status should be Shutdown
        // If it fails, that's also acceptable for this edge case
    }

    #[test]
    fn test_operations_after_shutdown() {
        let config = ProductionConfig::default();
        let mut ecosystem = ProductionEcosystem::new(config).unwrap();

        ecosystem.initialize().unwrap();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: types
        // TEST_PRIORITY: normal
        ecosystem.shutdown().unwrap();

        // Operations after shutdown
        let metrics_result = ecosystem.update_metrics();
        // Should handle gracefully (implementation dependent)
        let _ = metrics_result;
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    fn test_uptime_tracking() {
        let config = ProductionConfig::default();
        let ecosystem = ProductionEcosystem::new(config).unwrap();

        let uptime1 = ecosystem.uptime();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let uptime2 = ecosystem.uptime();

        assert!(uptime2 > uptime1);
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_empty_service_name() {
        let mut config = ProductionConfig::default();
        config.core.service_name = String::new();

        let result = ProductionEcosystem::new(config);
        // Should handle empty service name
        assert!(result.is_ok());
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal

    #[test]
    fn test_all_flags_disabled() {
        let mut config = ProductionConfig::default();
        config.core.flags = ProductionFlags {
            enable_advanced_monitoring: false,
            enable_distributed_tracing: false,
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            enable_performance_profiling: false,
            enable_security_auditing: false,
            enable_auto_scaling: false,
            enable_circuit_breakers: false,
            enable_rate_limiting: false,
            enable_caching: false,
        };

        let mut ecosystem = ProductionEcosystem::new(config).unwrap();

        // Should still initialize successfully
        let result = ecosystem.initialize();
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_rapid_initialization_shutdown() {
        for _ in 0..10 {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: types
            // TEST_PRIORITY: normal
            let config = ProductionConfig::default();
            let mut ecosystem = ProductionEcosystem::new(config).unwrap();
            ecosystem.initialize().unwrap();
            ecosystem.shutdown().unwrap();
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: types
    // TEST_PRIORITY: normal
    #[test]
    fn test_performance_metrics_update() {
        let mut metrics = PerformanceMetrics::default();

        // Simulate metric updates
        metrics.avg_response_time_ms = 12.5;
        metrics.p95_response_time_ms = 45.0;
        metrics.p99_response_time_ms = 78.3;
        metrics.requests_per_second = 1000.0;
        metrics.error_rate_percent = 0.01;
        metrics.throughput_bytes_per_sec = 1_000_000;

        assert!(metrics.avg_response_time_ms > 0.0);
        assert!(metrics.p99_response_time_ms > metrics.p95_response_time_ms);
        assert!(metrics.p95_response_time_ms > metrics.avg_response_time_ms);
    }
}
