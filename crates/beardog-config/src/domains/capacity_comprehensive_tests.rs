//! Comprehensive tests for CapacityConfig
//!
//! Added December 8, 2025 to increase coverage toward 90% target
//! Targets: from_env_provider, all environment variables, validation, edge cases

#[cfg(test)]
mod tests {
    use crate::domains::capacity::CapacityConfig;
    use std::collections::HashMap;

    // ============================================================================
    // Default and Construction Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_const_defaults() {
        let config = CapacityConfig::const_defaults();

        assert_eq!(config.default_channel_buffer, 1000);
        assert_eq!(config.discovery_queue_size, 100);
        assert_eq!(config.event_bus_capacity, 10000);
        assert_eq!(config.max_connections, 100);
        assert_eq!(config.min_idle_connections, 10);
        assert_eq!(config.connection_pool_timeout_secs, 30);
        assert_eq!(config.max_message_size_bytes, 10 * 1024 * 1024);
        assert_eq!(config.buffer_pool_size, 1024);
        assert_eq!(config.cache_max_entries, 10000);
    }

    #[serial_test::serial]
    #[test]
    fn test_default_equals_const_defaults() {
        let default_config = CapacityConfig::default();
        let const_config = CapacityConfig::const_defaults();

        assert_eq!(default_config, const_config);
    }

    // ============================================================================
    // from_env_provider Tests (Concurrent-Safe Testing Pattern)
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_no_overrides() {
        let config = CapacityConfig::from_env_provider(|_| None);
        let defaults = CapacityConfig::default();

        assert_eq!(config, defaults);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_channel_buffer() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "2000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 2000);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_discovery_queue() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_DISCOVERY_QUEUE_SIZE", "500");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.discovery_queue_size, 500);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_event_bus() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_EVENT_BUS_CAPACITY", "50000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.event_bus_capacity, 50000);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_max_connections() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_MAX_CONNECTIONS", "500");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.max_connections, 500);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_min_idle_connections() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_MIN_IDLE_CONNECTIONS", "50");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.min_idle_connections, 50);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_pool_timeout() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CONNECTION_POOL_TIMEOUT_SECS", "60");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.connection_pool_timeout_secs, 60);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_max_message_size() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_MAX_MESSAGE_SIZE_BYTES", "52428800"); // 50 MB

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.max_message_size_bytes, 52428800);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_buffer_pool() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_BUFFER_POOL_SIZE", "4096");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.buffer_pool_size, 4096);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_cache_max() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "50000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.cache_max_entries, 50000);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_all_fields() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "3000");
        env.insert("BEARDOG_DISCOVERY_QUEUE_SIZE", "300");
        env.insert("BEARDOG_EVENT_BUS_CAPACITY", "30000");
        env.insert("BEARDOG_MAX_CONNECTIONS", "300");
        env.insert("BEARDOG_MIN_IDLE_CONNECTIONS", "30");
        env.insert("BEARDOG_CONNECTION_POOL_TIMEOUT_SECS", "90");
        env.insert("BEARDOG_MAX_MESSAGE_SIZE_BYTES", "31457280"); // 30 MB
        env.insert("BEARDOG_BUFFER_POOL_SIZE", "2048");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "30000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 3000);
        assert_eq!(config.discovery_queue_size, 300);
        assert_eq!(config.event_bus_capacity, 30000);
        assert_eq!(config.max_connections, 300);
        assert_eq!(config.min_idle_connections, 30);
        assert_eq!(config.connection_pool_timeout_secs, 90);
        assert_eq!(config.max_message_size_bytes, 31457280);
        assert_eq!(config.buffer_pool_size, 2048);
        assert_eq!(config.cache_max_entries, 30000);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_invalid_values_ignored() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "invalid");
        env.insert("BEARDOG_MAX_CONNECTIONS", "not_a_number");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        // Should use defaults when parsing fails
        assert_eq!(config.default_channel_buffer, 1000);
        assert_eq!(config.max_connections, 100);
    }

    #[serial_test::serial]
    #[test]
    fn test_from_env_provider_partial_overrides() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_MAX_CONNECTIONS", "200");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "20000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        // Overridden values
        assert_eq!(config.max_connections, 200);
        assert_eq!(config.cache_max_entries, 20000);

        // Default values for non-overridden fields
        assert_eq!(config.default_channel_buffer, 1000);
        assert_eq!(config.discovery_queue_size, 100);
    }

    // ============================================================================
    // from_env() Tests (Using Real Environment)
    //
    // CONCURRENCY NOTE: These tests use serial_test to prevent race conditions
    // when modifying shared process environment. This is the ONLY acceptable use
    // of serialization - environment variable tests. All other tests must be
    // fully concurrent and use proper synchronization primitives.
    // ============================================================================

    use serial_test::serial;

    #[test]
    #[serial] // Required: env vars are process-global
    fn test_from_env_no_variables() {
        std::env::remove_var("BEARDOG_CHANNEL_BUFFER");
        std::env::remove_var("BEARDOG_MAX_CONNECTIONS");

        let config = CapacityConfig::from_env();

        assert_eq!(config.default_channel_buffer, 1000);
        assert_eq!(config.max_connections, 100);
    }

    #[test]
    #[serial] // Required: env vars are process-global
    fn test_from_env_with_channel_buffer() {
        std::env::remove_var("BEARDOG_CHANNEL_BUFFER");
        std::env::set_var("BEARDOG_CHANNEL_BUFFER", "5000");

        let config = CapacityConfig::from_env();

        assert_eq!(config.default_channel_buffer, 5000);

        std::env::remove_var("BEARDOG_CHANNEL_BUFFER");
    }

    #[test]
    #[serial] // Required: env vars are process-global
    fn test_from_env_invalid_value() {
        std::env::remove_var("BEARDOG_EVENT_BUS_CAPACITY");
        std::env::set_var("BEARDOG_EVENT_BUS_CAPACITY", "not_valid");

        let config = CapacityConfig::from_env();

        // Should use default
        assert_eq!(config.event_bus_capacity, 10000);

        std::env::remove_var("BEARDOG_EVENT_BUS_CAPACITY");
    }

    // ============================================================================
    // Configuration Scenarios
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_low_resource_config() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "100");
        env.insert("BEARDOG_MAX_CONNECTIONS", "10");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "1000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 100);
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.cache_max_entries, 1000);
    }

    #[serial_test::serial]
    #[test]
    fn test_high_throughput_config() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "10000");
        env.insert("BEARDOG_EVENT_BUS_CAPACITY", "100000");
        env.insert("BEARDOG_MAX_CONNECTIONS", "1000");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "100000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 10000);
        assert_eq!(config.event_bus_capacity, 100000);
        assert_eq!(config.max_connections, 1000);
        assert_eq!(config.cache_max_entries, 100000);
    }

    #[serial_test::serial]
    #[test]
    fn test_large_message_config() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_MAX_MESSAGE_SIZE_BYTES", "104857600"); // 100 MB

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.max_message_size_bytes, 104857600);
    }

    // ============================================================================
    // Trait Implementation Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_clone() {
        let config1 = CapacityConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1, config2);
    }

    #[serial_test::serial]
    #[test]
    fn test_debug() {
        let config = CapacityConfig::default();
        let debug_str = format!("{:?}", config);

        assert!(debug_str.contains("CapacityConfig"));
    }

    #[serial_test::serial]
    #[test]
    fn test_partial_eq() {
        let config1 = CapacityConfig::default();
        let config2 = CapacityConfig::default();

        let mut env = HashMap::new();
        env.insert("BEARDOG_MAX_CONNECTIONS", "200");
        let config3 = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config1, config2);
        assert_ne!(config1, config3);
    }

    // ============================================================================
    // Serialization Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_serialization() {
        let config = CapacityConfig::default();

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: CapacityConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config, deserialized);
    }

    #[serial_test::serial]
    #[test]
    fn test_serialization_with_custom_values() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_MAX_CONNECTIONS", "500");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "50000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        let json = serde_json::to_string(&config).expect("Should serialize");
        let deserialized: CapacityConfig = serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(config.max_connections, deserialized.max_connections);
        assert_eq!(config.cache_max_entries, deserialized.cache_max_entries);
    }

    // ============================================================================
    // Edge Cases and Boundary Tests
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_zero_values() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "0");
        env.insert("BEARDOG_MAX_CONNECTIONS", "0");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 0);
        assert_eq!(config.max_connections, 0);
    }

    #[serial_test::serial]
    #[test]
    fn test_very_large_values() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_EVENT_BUS_CAPACITY", "1000000");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "10000000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.event_bus_capacity, 1000000);
        assert_eq!(config.cache_max_entries, 10000000);
    }

    #[serial_test::serial]
    #[test]
    fn test_negative_values_rejected() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_MAX_CONNECTIONS", "-100");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        // Should use default when parsing fails
        assert_eq!(config.max_connections, 100);
    }

    #[serial_test::serial]
    #[test]
    fn test_empty_string_values() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        // Should use default when parsing fails
        assert_eq!(config.default_channel_buffer, 1000);
    }

    #[serial_test::serial]
    #[test]
    fn test_whitespace_values() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_DISCOVERY_QUEUE_SIZE", "  ");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        // Should use default when parsing fails
        assert_eq!(config.discovery_queue_size, 100);
    }

    // ============================================================================
    // Realistic Production Scenarios
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_production_high_traffic_config() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "5000");
        env.insert("BEARDOG_DISCOVERY_QUEUE_SIZE", "500");
        env.insert("BEARDOG_EVENT_BUS_CAPACITY", "50000");
        env.insert("BEARDOG_MAX_CONNECTIONS", "500");
        env.insert("BEARDOG_MIN_IDLE_CONNECTIONS", "50");
        env.insert("BEARDOG_MAX_MESSAGE_SIZE_BYTES", "52428800");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "50000");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 5000);
        assert_eq!(config.max_connections, 500);
        assert_eq!(config.min_idle_connections, 50);
        assert_eq!(config.cache_max_entries, 50000);
    }

    #[serial_test::serial]
    #[test]
    fn test_development_config() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "10");
        env.insert("BEARDOG_MAX_CONNECTIONS", "5");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "100");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 10);
        assert_eq!(config.max_connections, 5);
        assert_eq!(config.cache_max_entries, 100);
    }

    #[serial_test::serial]
    #[test]
    fn test_embedded_device_config() {
        let mut env = HashMap::new();
        env.insert("BEARDOG_CHANNEL_BUFFER", "50");
        env.insert("BEARDOG_EVENT_BUS_CAPACITY", "1000");
        env.insert("BEARDOG_MAX_CONNECTIONS", "10");
        env.insert("BEARDOG_BUFFER_POOL_SIZE", "128");
        env.insert("BEARDOG_CACHE_MAX_ENTRIES", "500");

        let config = CapacityConfig::from_env_provider(|key| env.get(key).map(|s| s.to_string()));

        assert_eq!(config.default_channel_buffer, 50);
        assert_eq!(config.event_bus_capacity, 1000);
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.buffer_pool_size, 128);
        assert_eq!(config.cache_max_entries, 500);
    }

    // ============================================================================
    // Concurrent-Safe Testing Pattern Verification
    // ============================================================================

    #[serial_test::serial]
    #[test]
    fn test_env_provider_closure_isolation() {
        // Test 1 with one set of values
        let mut env1 = HashMap::new();
        env1.insert("BEARDOG_MAX_CONNECTIONS", "100");

        let config1 = CapacityConfig::from_env_provider(|key| env1.get(key).map(|s| s.to_string()));

        // Test 2 with different values (running concurrently safe)
        let mut env2 = HashMap::new();
        env2.insert("BEARDOG_MAX_CONNECTIONS", "200");

        let config2 = CapacityConfig::from_env_provider(|key| env2.get(key).map(|s| s.to_string()));

        // Should be different and isolated
        assert_eq!(config1.max_connections, 100);
        assert_eq!(config2.max_connections, 200);
    }
}
