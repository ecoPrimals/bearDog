// SPDX-License-Identifier: AGPL-3.0-only

// ========================================================================
// memory_key_manager/metrics: 0% → test Default impls
// ========================================================================

mod metrics_tests {
    use crate::memory_key_manager::metrics::*;
    use std::time::Duration;

    #[test]
    fn test_key_manager_metrics_default() {
        let metrics = KeyManagerMetrics::default();
        assert_eq!(metrics.total_keys, 0);
        assert_eq!(metrics.keys_created, 0);
        assert_eq!(metrics.keys_accessed, 0);
        assert_eq!(metrics.keys_expired, 0);
        assert_eq!(metrics.avg_access_time, Duration::from_millis(0));
        assert_eq!(metrics.cache_hit_rate, 0.0);
        assert_eq!(metrics.memory_usage, 0);
    }

    #[test]
    fn test_key_manager_metrics_custom() {
        let metrics = KeyManagerMetrics {
            total_keys: 100,
            keys_created: 200,
            keys_accessed: 500,
            keys_expired: 10,
            avg_access_time: Duration::from_micros(50),
            cache_hit_rate: 0.95,
            memory_usage: 1024,
        };
        assert_eq!(metrics.total_keys, 100);
        assert_eq!(metrics.keys_created, 200);
        assert_eq!(metrics.cache_hit_rate, 0.95);
    }

    #[test]
    fn test_operation_metrics() {
        let op = OperationMetrics {
            operation_type: "generate".to_string(),
            duration: Duration::from_millis(5),
            success: true,
            error: None,
        };
        assert_eq!(op.operation_type, "generate");
        assert!(op.success);
        assert!(op.error.is_none());
    }

    #[test]
    fn test_operation_metrics_with_error() {
        let op = OperationMetrics {
            operation_type: "decrypt".to_string(),
            duration: Duration::from_millis(2),
            success: false,
            error: Some("key not found".to_string()),
        };
        assert!(!op.success);
        assert_eq!(op.error.as_deref(), Some("key not found"));
    }

    #[test]
    fn test_extended_metrics() {
        let ext = ExtendedMetrics {
            base_metrics: KeyManagerMetrics::default(),
            recent_operations: vec![OperationMetrics {
                operation_type: "store".to_string(),
                duration: Duration::from_millis(1),
                success: true,
                error: None,
            }],
            timestamp: chrono::Utc::now(),
        };
        assert_eq!(ext.base_metrics.total_keys, 0);
        assert_eq!(ext.recent_operations.len(), 1);
    }

    #[test]
    fn test_metrics_serde_roundtrip() {
        let metrics = KeyManagerMetrics::default();
        let json = serde_json::to_string(&metrics).unwrap();
        let deserialized: KeyManagerMetrics = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.total_keys, 0);
        assert_eq!(deserialized.keys_created, 0);
    }
}

// ========================================================================
// memory_key_manager/config: ~33% → test KeyStorageConfig default
// ========================================================================

mod config_tests {
    use crate::memory_key_manager::config::*;

    #[test]
    fn test_memory_key_config_default() {
        let config = MemoryKeyConfig::default();
        assert_eq!(config.max_keys, 1000);
        assert_eq!(config.key_expiration_seconds, 0);
        assert!(!config.enable_rotation);
    }

    #[test]
    fn test_key_storage_config_default() {
        let config = KeyStorageConfig::default();
        assert_eq!(config.backend, "memory");
        assert_eq!(config.max_capacity, 10000);
        assert!(config.encrypt_at_rest);
    }

    #[test]
    fn test_key_storage_config_custom() {
        let config = KeyStorageConfig {
            backend: "disk".to_string(),
            max_capacity: 5000,
            encrypt_at_rest: false,
        };
        assert_eq!(config.backend, "disk");
        assert_eq!(config.max_capacity, 5000);
        assert!(!config.encrypt_at_rest);
    }

    #[test]
    fn test_key_manager_config_alias() {
        // KeyManagerConfig is a type alias for MemoryKeyConfig
        let config: KeyManagerConfig = MemoryKeyConfig::default();
        assert_eq!(config.max_keys, 1000);
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let config = MemoryKeyConfig {
            max_keys: 500,
            key_expiration_seconds: 3600,
            enable_rotation: true,
        };
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: MemoryKeyConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.max_keys, 500);
        assert_eq!(deserialized.key_expiration_seconds, 3600);
        assert!(deserialized.enable_rotation);
    }

    #[test]
    fn test_key_storage_config_serde_roundtrip() {
        let config = KeyStorageConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: KeyStorageConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.backend, "memory");
        assert!(deserialized.encrypt_at_rest);
    }
}
