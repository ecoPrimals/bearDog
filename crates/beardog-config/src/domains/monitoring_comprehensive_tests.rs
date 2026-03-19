// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive tests for MonitoringConfig
//!
//! Added December 8, 2025 to increase coverage toward 90% target
//! Targets: All builder methods, validation, environment loading, edge cases

#[cfg(test)]
mod tests {
    use crate::domains::monitoring::{MonitoringConfig, MonitoringConfigBuilder};

    // ============================================================================
    // MonitoringConfig Default and Construction Tests
    // ============================================================================

    #[test]
    fn test_const_defaults() {
        let config = MonitoringConfig::const_defaults();

        assert_eq!(config.log_level, "info");
        assert_eq!(config.log_format, "text");
        assert!(config.structured_logging);
        assert!(config.enable_metrics);
        assert_eq!(config.metrics_port, 9092);
        assert!(config.enable_health_check);
        assert_eq!(config.health_check_port, 9093);
        assert!(!config.enable_performance_tracking);
        assert_eq!(config.tracing_sample_rate, 0.1);
    }

    #[test]
    fn test_default_equals_const_defaults() {
        let default_config = MonitoringConfig::default();
        let const_config = MonitoringConfig::const_defaults();

        assert_eq!(default_config.log_level, const_config.log_level);
        assert_eq!(default_config.log_format, const_config.log_format);
    }

    #[test]
    fn test_from_env_respects_current_env() {
        // ✅ CONCURRENT-SAFE: Only READ environment, never WRITE
        // This test is safe to run in parallel because it doesn't mutate global state

        let config = MonitoringConfig::from_env();

        // Verify config is valid (will use either env vars or defaults)
        assert!(!config.log_level.is_empty());
        assert!(!config.log_format.is_empty());
        assert!(config.metrics_port > 0);
        assert!(config.health_check_port > 0);
        assert!(config.tracing_sample_rate >= 0.0 && config.tracing_sample_rate <= 1.0);
    }

    #[test]
    fn test_builder_with_log_level() {
        // ✅ CONCURRENT-SAFE: Use builder pattern, no env vars
        let config = MonitoringConfig::builder()
            .log_level("debug".to_string())
            .build();

        assert_eq!(config.log_level, "debug");
    }

    #[test]
    fn test_builder_with_log_format() {
        // ✅ CONCURRENT-SAFE: Use builder pattern, no env vars
        let config = MonitoringConfig::builder()
            .log_format("json".to_string())
            .build();

        assert_eq!(config.log_format, "json");
    }

    #[test]
    fn test_builder_with_metrics_port() {
        // ✅ CONCURRENT-SAFE: Use builder pattern, no env vars
        let config = MonitoringConfig::builder().metrics_port(8080).build();

        assert_eq!(config.metrics_port, 8080);
    }

    #[test]
    fn test_builder_with_health_port() {
        // ✅ CONCURRENT-SAFE: Use builder pattern, no env vars
        let config = MonitoringConfig::builder().health_check_port(8081).build();

        assert_eq!(config.health_check_port, 8081);
    }

    #[test]
    fn test_builder_with_tracing_rate() {
        // ✅ CONCURRENT-SAFE: Use builder pattern, no env vars
        let config = MonitoringConfig::builder().tracing_sample_rate(0.5).build();

        assert_eq!(config.tracing_sample_rate, 0.5);
    }

    #[test]
    fn test_builder_with_multiple_values() {
        // ✅ CONCURRENT-SAFE: Test builder with multiple values
        let config = MonitoringConfig::builder()
            .metrics_port(7070)
            .health_check_port(7071)
            .log_level("trace".to_string())
            .tracing_sample_rate(0.25)
            .build();

        assert_eq!(config.metrics_port, 7070);
        assert_eq!(config.health_check_port, 7071);
        assert_eq!(config.log_level, "trace");
        assert_eq!(config.tracing_sample_rate, 0.25);
    }

    // ============================================================================
    // Builder Pattern Comprehensive Tests
    // ============================================================================

    #[test]
    fn test_builder_new() {
        let builder = MonitoringConfigBuilder::new();
        let config = builder.build();

        assert_eq!(config, MonitoringConfig::default());
    }

    #[test]
    fn test_builder_default() {
        let builder = MonitoringConfigBuilder::default();
        let config = builder.build();

        assert_eq!(config, MonitoringConfig::default());
    }

    #[test]
    fn test_builder_log_level_trace() {
        let config = MonitoringConfig::builder()
            .log_level("trace".to_string())
            .build();

        assert_eq!(config.log_level, "trace");
    }

    #[test]
    fn test_builder_log_level_debug() {
        let config = MonitoringConfig::builder()
            .log_level("debug".to_string())
            .build();

        assert_eq!(config.log_level, "debug");
    }

    #[test]
    fn test_builder_log_level_info() {
        let config = MonitoringConfig::builder()
            .log_level("info".to_string())
            .build();

        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_builder_log_level_warn() {
        let config = MonitoringConfig::builder()
            .log_level("warn".to_string())
            .build();

        assert_eq!(config.log_level, "warn");
    }

    #[test]
    fn test_builder_log_level_error() {
        let config = MonitoringConfig::builder()
            .log_level("error".to_string())
            .build();

        assert_eq!(config.log_level, "error");
    }

    #[test]
    fn test_builder_log_format_json() {
        let config = MonitoringConfig::builder()
            .log_format("json".to_string())
            .build();

        assert_eq!(config.log_format, "json");
    }

    #[test]
    fn test_builder_log_format_text() {
        let config = MonitoringConfig::builder()
            .log_format("text".to_string())
            .build();

        assert_eq!(config.log_format, "text");
    }

    #[test]
    fn test_builder_structured_logging_enabled() {
        let config = MonitoringConfig::builder().structured_logging(true).build();

        assert!(config.structured_logging);
    }

    #[test]
    fn test_builder_structured_logging_disabled() {
        let config = MonitoringConfig::builder()
            .structured_logging(false)
            .build();

        assert!(!config.structured_logging);
    }

    #[test]
    fn test_builder_enable_metrics_enabled() {
        let config = MonitoringConfig::builder().enable_metrics(true).build();

        assert!(config.enable_metrics);
    }

    #[test]
    fn test_builder_enable_metrics_disabled() {
        let config = MonitoringConfig::builder().enable_metrics(false).build();

        assert!(!config.enable_metrics);
    }

    #[test]
    fn test_builder_metrics_port() {
        let config = MonitoringConfig::builder().metrics_port(9999).build();

        assert_eq!(config.metrics_port, 9999);
    }

    #[test]
    fn test_builder_enable_health_check_enabled() {
        let config = MonitoringConfig::builder()
            .enable_health_check(true)
            .build();

        assert!(config.enable_health_check);
    }

    #[test]
    fn test_builder_enable_health_check_disabled() {
        let config = MonitoringConfig::builder()
            .enable_health_check(false)
            .build();

        assert!(!config.enable_health_check);
    }

    #[test]
    fn test_builder_health_check_port() {
        let config = MonitoringConfig::builder().health_check_port(8888).build();

        assert_eq!(config.health_check_port, 8888);
    }

    #[test]
    fn test_builder_enable_performance_tracking_enabled() {
        let config = MonitoringConfig::builder()
            .enable_performance_tracking(true)
            .build();

        assert!(config.enable_performance_tracking);
    }

    #[test]
    fn test_builder_enable_performance_tracking_disabled() {
        let config = MonitoringConfig::builder()
            .enable_performance_tracking(false)
            .build();

        assert!(!config.enable_performance_tracking);
    }

    #[test]
    fn test_builder_tracing_sample_rate_zero() {
        let config = MonitoringConfig::builder().tracing_sample_rate(0.0).build();

        assert_eq!(config.tracing_sample_rate, 0.0);
    }

    #[test]
    fn test_builder_tracing_sample_rate_full() {
        let config = MonitoringConfig::builder().tracing_sample_rate(1.0).build();

        assert_eq!(config.tracing_sample_rate, 1.0);
    }

    #[test]
    fn test_builder_all_fields() {
        let config = MonitoringConfig::builder()
            .log_level("debug".to_string())
            .log_format("json".to_string())
            .structured_logging(true)
            .enable_metrics(true)
            .metrics_port(8080)
            .enable_health_check(true)
            .health_check_port(8081)
            .enable_performance_tracking(true)
            .tracing_sample_rate(0.5)
            .build();

        assert_eq!(config.log_level, "debug");
        assert_eq!(config.log_format, "json");
        assert!(config.structured_logging);
        assert!(config.enable_metrics);
        assert_eq!(config.metrics_port, 8080);
        assert!(config.enable_health_check);
        assert_eq!(config.health_check_port, 8081);
        assert!(config.enable_performance_tracking);
        assert_eq!(config.tracing_sample_rate, 0.5);
    }

    // ============================================================================
    // Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_default_config() {
        let config = MonitoringConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_all_log_levels() {
        let levels = vec!["trace", "debug", "info", "warn", "error"];

        for level in levels {
            let config = MonitoringConfig::builder()
                .log_level(level.to_string())
                .build();
            assert!(config.validate().is_ok(), "Level {} should be valid", level);
        }
    }

    #[test]
    fn test_validate_invalid_log_level_empty() {
        let config = MonitoringConfig::builder()
            .log_level("".to_string())
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_invalid_log_level_typo() {
        let config = MonitoringConfig::builder()
            .log_level("debg".to_string())
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_all_log_formats() {
        let formats = vec!["json", "text"];

        for format in formats {
            let config = MonitoringConfig::builder()
                .log_format(format.to_string())
                .build();
            assert!(
                config.validate().is_ok(),
                "Format {} should be valid",
                format
            );
        }
    }

    #[test]
    fn test_validate_invalid_log_format() {
        let config = MonitoringConfig::builder()
            .log_format("xml".to_string())
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_tracing_rate_valid_boundaries() {
        let config_zero = MonitoringConfig::builder().tracing_sample_rate(0.0).build();
        assert!(config_zero.validate().is_ok());

        let config_one = MonitoringConfig::builder().tracing_sample_rate(1.0).build();
        assert!(config_one.validate().is_ok());
    }

    #[test]
    fn test_validate_tracing_rate_negative() {
        let config = MonitoringConfig::builder()
            .tracing_sample_rate(-0.1)
            .build();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validate_tracing_rate_too_high() {
        let config = MonitoringConfig::builder().tracing_sample_rate(1.1).build();
        assert!(config.validate().is_err());
    }

    // ============================================================================
    // Configuration Scenarios
    // ============================================================================

    #[test]
    fn test_development_config() {
        let config = MonitoringConfig::builder()
            .log_level("debug".to_string())
            .log_format("text".to_string())
            .structured_logging(false)
            .enable_metrics(false)
            .enable_performance_tracking(false)
            .tracing_sample_rate(1.0) // Full tracing in dev
            .build();

        assert_eq!(config.log_level, "debug");
        assert!(!config.structured_logging);
        assert!(!config.enable_metrics);
        assert_eq!(config.tracing_sample_rate, 1.0);
    }

    #[test]
    fn test_minimal_monitoring_config() {
        let config = MonitoringConfig::builder()
            .log_level("error".to_string())
            .enable_metrics(false)
            .enable_health_check(false)
            .enable_performance_tracking(false)
            .tracing_sample_rate(0.0)
            .build();

        assert_eq!(config.log_level, "error");
        assert!(!config.enable_metrics);
        assert!(!config.enable_health_check);
        assert_eq!(config.tracing_sample_rate, 0.0);
    }

    // ============================================================================
    // Serialization Tests
    // ============================================================================

    #[test]
    fn test_serialization() {
        let config = MonitoringConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: MonitoringConfig =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_serialization_with_custom_values() {
        let config = MonitoringConfig::builder()
            .log_level("debug".to_string())
            .log_format("json".to_string())
            .metrics_port(9999)
            .build();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: MonitoringConfig =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.log_level, deserialized.log_level);
        assert_eq!(config.metrics_port, deserialized.metrics_port);
    }

    // ============================================================================
    // Trait Implementation Tests
    // ============================================================================

    #[test]
    fn test_clone() {
        let config1 = MonitoringConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[test]
    fn test_debug() {
        let config = MonitoringConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("MonitoringConfig"));
    }

    #[test]
    fn test_partial_eq() {
        let config1 = MonitoringConfig::default();
        let config2 = MonitoringConfig::default();
        let config3 = MonitoringConfig::builder()
            .log_level("debug".to_string())
            .build();

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    // ============================================================================
    // Builder Chaining Tests
    // ============================================================================

    #[test]
    fn test_builder_chaining() {
        let config = MonitoringConfig::builder()
            .log_level("warn".to_string())
            .log_format("json".to_string())
            .enable_metrics(true)
            .enable_performance_tracking(true)
            .build();

        assert_eq!(config.log_level, "warn");
        assert_eq!(config.log_format, "json");
        assert!(config.enable_metrics);
        assert!(config.enable_performance_tracking);
    }

    #[test]
    fn test_builder_partial_configuration() {
        let config = MonitoringConfig::builder()
            .log_level("error".to_string())
            .build();

        // Unset fields should use defaults
        assert_eq!(config.log_level, "error");
        assert_eq!(config.log_format, "text"); // default
        assert_eq!(config.metrics_port, 9092); // default
    }
}
