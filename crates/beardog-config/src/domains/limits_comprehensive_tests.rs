//! Comprehensive tests for LimitsConfig
//!
//! Added December 8, 2025 to increase coverage toward 90% target
//! Targets: Validation, builder patterns, configuration scenarios

#[cfg(test)]
mod tests {
    use crate::domains::limits::{
        LimitsConfig, DEFAULT_BACKOFF_MS, DEFAULT_BUFFER_SIZE, DEFAULT_MAX_CONNECTIONS,
        DEFAULT_MAX_MESSAGE_SIZE, DEFAULT_MAX_RETRIES, DEFAULT_MAX_THREADS, DEFAULT_MIN_THREADS,
        DEFAULT_OPERATION_TIMEOUT_SECS, DEFAULT_QUEUE_SIZE,
    };

    // ============================================================================
    // Constant Verification Tests
    // ============================================================================

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_BUFFER_SIZE, 8192);
        assert_eq!(DEFAULT_MAX_CONNECTIONS, 100);
        assert_eq!(DEFAULT_MAX_RETRIES, 3);
        assert_eq!(DEFAULT_BACKOFF_MS, 100);
        assert_eq!(DEFAULT_MAX_MESSAGE_SIZE, 1_048_576);
        assert_eq!(DEFAULT_QUEUE_SIZE, 1000);
        assert_eq!(DEFAULT_MIN_THREADS, 2);
        assert_eq!(DEFAULT_MAX_THREADS, 128);
        assert_eq!(DEFAULT_OPERATION_TIMEOUT_SECS, 30);
    }

    // ============================================================================
    // Configuration Validity Tests
    // ============================================================================

    #[test]
    fn test_validate_default_config() {
        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");

        let config = LimitsConfig::default();
        assert!(config.validate().is_ok());

        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");
    }

    #[test]
    fn test_validate_zero_buffer_size_fails() {
        let config = LimitsConfig {
            buffer_size: 0,
            ..Default::default()
        };

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("Buffer size"));
    }

    #[test]
    fn test_validate_zero_max_connections_fails() {
        let config = LimitsConfig {
            max_connections: 0,
            ..Default::default()
        };

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("Maximum connections"));
    }

    #[test]
    fn test_validate_zero_queue_size_fails() {
        let config = LimitsConfig {
            queue_size: 0,
            ..Default::default()
        };

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("Queue size"));
    }

    #[test]
    fn test_validate_high_max_connections() {
        let config = LimitsConfig {
            max_connections: 100_000,
            ..Default::default()
        };

        // No max limit in actual validation
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_high_max_retries() {
        let config = LimitsConfig {
            max_retries: 1000,
            ..Default::default()
        };

        // No max limit in actual validation
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_nonzero_buffer_size() {
        let config = LimitsConfig {
            buffer_size: 1024,
            ..Default::default()
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_various_buffer_sizes() {
        // Test 512 bytes
        let config_512 = LimitsConfig {
            buffer_size: 512,
            ..Default::default()
        };
        assert!(config_512.validate().is_ok()); // No minimum in actual validation

        // Test 1 MB
        let config_1mb = LimitsConfig {
            buffer_size: 1024 * 1024,
            ..Default::default()
        };
        assert!(config_1mb.validate().is_ok());

        // Test 2 MB
        let config_2mb = LimitsConfig {
            buffer_size: 2 * 1024 * 1024,
            ..Default::default()
        };
        assert!(config_2mb.validate().is_ok()); // No maximum in actual validation
    }

    // ============================================================================
    // Thread Pool Validation Tests
    // ============================================================================

    #[test]
    fn test_validate_thread_pool_auto() {
        let config = LimitsConfig {
            thread_pool_size: 0, // Auto-detect
            ..Default::default()
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_thread_pool_min() {
        let config = LimitsConfig {
            thread_pool_size: DEFAULT_MIN_THREADS,
            ..Default::default()
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_thread_pool_max() {
        let config = LimitsConfig {
            thread_pool_size: DEFAULT_MAX_THREADS,
            ..Default::default()
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_thread_pool_small() {
        let config = LimitsConfig {
            thread_pool_size: 1, // Small but allowed
            ..Default::default()
        };

        // No minimum check in actual validation (0 is auto-detect, >0 is explicit)
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validate_thread_pool_too_large_fails() {
        let config = LimitsConfig {
            thread_pool_size: 256, // More than DEFAULT_MAX_THREADS (128)
            ..Default::default()
        };

        assert!(config.validate().is_err());
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("Thread pool size"));
        assert!(err.to_string().contains("128"));
    }

    // ============================================================================
    // Configuration Scenarios
    // ============================================================================

    #[test]
    fn test_low_resource_config() {
        let mut config = LimitsConfig::default();
        config.buffer_size = 2048;
        config.max_connections = 10;
        config.queue_size = 100;
        config.thread_pool_size = 2;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_high_throughput_config() {
        let mut config = LimitsConfig::default();
        config.buffer_size = 65536;
        config.max_connections = 1000;
        config.queue_size = 10000;
        config.thread_pool_size = 32;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_embedded_config() {
        let mut config = LimitsConfig::default();
        config.buffer_size = 1024;
        config.max_connections = 5;
        config.max_retries = 1;
        config.queue_size = 50;
        config.thread_pool_size = 2;

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_production_config() {
        let mut config = LimitsConfig::default();
        config.buffer_size = 32768;
        config.max_connections = 500;
        config.max_retries = 5;
        config.backoff_ms = 200;
        config.max_message_size = 10 * 1024 * 1024;
        config.queue_size = 5000;
        config.thread_pool_size = 16;
        config.operation_timeout_secs = 60;

        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Trait Implementation Tests
    // ============================================================================

    #[test]
    fn test_clone() {
        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        let config1 = LimitsConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.buffer_size, config2.buffer_size);
        assert_eq!(config1.max_connections, config2.max_connections);

        std::env::remove_var("BEARDOG_BUFFER_SIZE");
    }

    #[test]
    fn test_debug() {
        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        let config = LimitsConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("LimitsConfig"));

        std::env::remove_var("BEARDOG_BUFFER_SIZE");
    }

    #[test]
    fn test_partial_eq() {
        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");

        let config1 = LimitsConfig::default();
        let config2 = LimitsConfig::default();
        let mut config3 = LimitsConfig::default();
        config3.buffer_size = 16384;

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);

        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");
    }

    // ============================================================================
    // Serialization Tests
    // ============================================================================

    #[test]
    fn test_serialization() {
        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        let config = LimitsConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: LimitsConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);

        std::env::remove_var("BEARDOG_BUFFER_SIZE");
    }

    #[test]
    fn test_serialization_with_custom_values() {
        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        let mut config = LimitsConfig::default();
        config.buffer_size = 16384;
        config.max_connections = 500;

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: LimitsConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.buffer_size, deserialized.buffer_size);
        assert_eq!(config.max_connections, deserialized.max_connections);

        std::env::remove_var("BEARDOG_BUFFER_SIZE");
    }

    // ============================================================================
    // Edge Cases and Boundary Tests
    // ============================================================================

    #[test]
    fn test_boundary_max_connections() {
        let mut config = LimitsConfig::default();
        config.max_connections = 10000;

        // No max connections limit in actual validation
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_boundary_max_retries() {
        let mut config = LimitsConfig::default();
        config.max_retries = 100;

        // No max retries limit in actual validation
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_minimal_valid_config() {
        let mut config = LimitsConfig::default();
        config.buffer_size = 1024; // Min
        config.max_connections = 1;
        config.max_retries = 0;
        config.queue_size = 1;
        config.thread_pool_size = 2; // Min

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_maximal_valid_config() {
        let mut config = LimitsConfig::default();
        config.buffer_size = 1024 * 1024; // Max
        config.max_connections = 10000; // Max
        config.max_retries = 100; // Max
        config.queue_size = 100000;
        config.thread_pool_size = 128; // Max

        assert!(config.validate().is_ok());
    }

    // ============================================================================
    // Validation Message Tests
    // ============================================================================

    #[test]
    fn test_validation_error_messages() {
        let mut config = LimitsConfig::default();

        // Test buffer size error message
        config.buffer_size = 0;
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("Buffer size"));
        assert!(err.to_string().contains("greater than 0"));
    }

    #[test]
    fn test_from_env_method() {
        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");

        let config = LimitsConfig::from_env();

        assert_eq!(config.buffer_size, DEFAULT_BUFFER_SIZE);
        assert_eq!(config.max_connections, DEFAULT_MAX_CONNECTIONS);

        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");
    }

    #[test]
    fn test_from_env_with_overrides() {
        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");

        std::env::set_var("BEARDOG_BUFFER_SIZE", "16384");
        std::env::set_var("BEARDOG_MAX_CONNECTIONS", "200");

        let config = LimitsConfig::from_env();

        assert_eq!(config.buffer_size, 16384);
        assert_eq!(config.max_connections, 200);

        std::env::remove_var("BEARDOG_BUFFER_SIZE");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");
    }
}
