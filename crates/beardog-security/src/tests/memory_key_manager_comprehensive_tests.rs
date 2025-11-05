//! Comprehensive Memory Key Manager Tests
//!
//! TEST_CATEGORY: unit
//! TEST_DOMAIN: security/key_management
//! TEST_PRIORITY: critical
//!
//! Comprehensive test coverage for the MemoryKeyManager including:
//! - Key generation and storage
//! - Key retrieval and deletion
//! - Key existence checks
//! - Concurrent access patterns
//! - Error handling
//! - Edge cases

use crate::memory_key_manager::{KeyMetadata, MemoryKeyConfig, MemoryKeyManager};
use chrono::Utc;

#[cfg(test)]
mod tests {
    use super::*;

    /// Test 1: Basic key generation
    #[test]
    fn test_key_generation_basic() {
        let config = MemoryKeyConfig::default();
        let manager = MemoryKeyManager::new(config).unwrap();

        let key_id = manager.generate_key().unwrap();

        assert!(!key_id.is_empty());
        assert!(key_id.starts_with("key_"));
    }

    /// Test 2: Key storage and retrieval
    #[test]
    fn test_key_storage_and_retrieval() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let test_key = b"test_key_data_123".to_vec();
        let metadata = KeyMetadata {
            id: "test_key_1".to_string(),
            created_at: Utc::now(),
            key_type: "AES256".to_string(),
        };

        let key_id = manager.store_key(&test_key, metadata).unwrap();

        let retrieved_key = manager.get_key(&key_id).unwrap();
        assert_eq!(retrieved_key, test_key);
    }

    /// Test 3: Key existence check
    #[test]
    fn test_key_exists() {
        let config = MemoryKeyConfig::default();
        let manager = MemoryKeyManager::new(config).unwrap();

        let key_id = manager.generate_key().unwrap();

        assert!(manager.key_exists(&key_id).unwrap());
        assert!(!manager.key_exists("nonexistent_key").unwrap());
    }

    /// Test 4: Key deletion
    #[test]
    fn test_key_deletion() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let key_id = manager.generate_key().unwrap();
        assert!(manager.key_exists(&key_id).unwrap());

        manager.delete_key(&key_id).unwrap();
        assert!(!manager.key_exists(&key_id).unwrap());
    }

    /// Test 5: Get nonexistent key returns error
    #[test]
    fn test_get_nonexistent_key_error() {
        let config = MemoryKeyConfig::default();
        let manager = MemoryKeyManager::new(config).unwrap();

        let result = manager.get_key("nonexistent_key");
        assert!(result.is_err());
    }

    /// Test 6: Multiple key generation produces unique IDs
    #[test]
    fn test_multiple_key_generation_uniqueness() {
        let config = MemoryKeyConfig::default();
        let manager = MemoryKeyManager::new(config).unwrap();

        let key1 = manager.generate_key().unwrap();
        let key2 = manager.generate_key().unwrap();
        let key3 = manager.generate_key().unwrap();

        assert_ne!(key1, key2);
        assert_ne!(key2, key3);
        assert_ne!(key1, key3);
    }

    /// Test 7: Store and retrieve multiple keys
    #[test]
    fn test_multiple_keys_storage() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let keys_data = [
            b"key_data_1".to_vec(),
            b"key_data_2".to_vec(),
            b"key_data_3".to_vec(),
        ];

        let mut key_ids = Vec::new();
        for (i, key_data) in keys_data.iter().enumerate() {
            let metadata = KeyMetadata {
                id: format!("test_key_{}", i),
                created_at: Utc::now(),
                key_type: "AES256".to_string(),
            };
            let key_id = manager.store_key(key_data, metadata).unwrap();
            key_ids.push(key_id);
        }

        for (i, key_id) in key_ids.iter().enumerate() {
            let retrieved = manager.get_key(key_id).unwrap();
            assert_eq!(retrieved, keys_data[i]);
        }
    }

    /// Test 8: Key deletion doesn't affect other keys
    #[test]
    fn test_deletion_isolation() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let key1 = manager.generate_key().unwrap();
        let key2 = manager.generate_key().unwrap();
        let key3 = manager.generate_key().unwrap();

        manager.delete_key(&key2).unwrap();

        assert!(manager.key_exists(&key1).unwrap());
        assert!(!manager.key_exists(&key2).unwrap());
        assert!(manager.key_exists(&key3).unwrap());
    }

    /// Test 9: Config with custom parameters
    #[test]
    fn test_custom_config() {
        let config = MemoryKeyConfig {
            max_keys: 500,
            key_expiration_seconds: 3600,
            enable_rotation: true,
        };

        let manager = MemoryKeyManager::new(config.clone()).unwrap();
        let key_id = manager.generate_key().unwrap();
        assert!(manager.key_exists(&key_id).unwrap());
    }

    /// Test 10: Empty key data storage
    #[test]
    fn test_empty_key_storage() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let empty_key = Vec::new();
        let metadata = KeyMetadata {
            id: "empty_key".to_string(),
            created_at: Utc::now(),
            key_type: "EMPTY".to_string(),
        };

        let key_id = manager.store_key(&empty_key, metadata).unwrap();
        let retrieved = manager.get_key(&key_id).unwrap();
        assert!(retrieved.is_empty());
    }

    /// Test 11: Large key data storage
    #[test]
    fn test_large_key_storage() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let large_key = vec![0u8; 1024]; // 1KB key
        let metadata = KeyMetadata {
            id: "large_key".to_string(),
            created_at: Utc::now(),
            key_type: "LARGE".to_string(),
        };

        let key_id = manager.store_key(&large_key, metadata).unwrap();
        let retrieved = manager.get_key(&key_id).unwrap();
        assert_eq!(retrieved.len(), 1024);
    }

    /// Test 12: Key overwrite by storing with same ID
    #[test]
    fn test_key_update_via_new_storage() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let key1 = b"original_data".to_vec();
        let key2 = b"updated_data".to_vec();

        let metadata1 = KeyMetadata {
            id: "test_key".to_string(),
            created_at: Utc::now(),
            key_type: "AES256".to_string(),
        };

        let key_id1 = manager.store_key(&key1, metadata1).unwrap();

        let metadata2 = KeyMetadata {
            id: "test_key".to_string(),
            created_at: Utc::now(),
            key_type: "AES256".to_string(),
        };

        let key_id2 = manager.store_key(&key2, metadata2).unwrap();

        // Each store creates a new key ID
        assert_ne!(key_id1, key_id2);

        let retrieved1 = manager.get_key(&key_id1).unwrap();
        let retrieved2 = manager.get_key(&key_id2).unwrap();

        assert_eq!(retrieved1, key1);
        assert_eq!(retrieved2, key2);
    }

    /// Test 13: Double deletion is safe
    #[test]
    fn test_double_deletion_safe() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let key_id = manager.generate_key().unwrap();

        manager.delete_key(&key_id).unwrap();
        // Second deletion should not panic (removes nothing)
        manager.delete_key(&key_id).unwrap();
    }

    /// Test 14: List keys returns empty vec (simplified implementation)
    #[test]
    fn test_list_keys() {
        let config = MemoryKeyConfig::default();
        let manager = MemoryKeyManager::new(config).unwrap();

        let _key1 = manager.generate_key().unwrap();
        let _key2 = manager.generate_key().unwrap();

        // Current implementation returns empty vec
        let keys = manager.list_keys().unwrap();
        assert!(keys.is_empty());
    }

    /// Test 15: Concurrent key generation (thread safety)
    #[test]
    fn test_concurrent_key_generation() {
        use std::sync::Arc;
        use std::thread;

        let config = MemoryKeyConfig::default();
        let manager = Arc::new(MemoryKeyManager::new(config).unwrap());

        let mut handles = vec![];

        for _ in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let handle = thread::spawn(move || manager_clone.generate_key());
            handles.push(handle);
        }

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All should succeed
        assert_eq!(results.len(), 10);
        for result in &results {
            assert!(result.is_ok());
        }

        // All should be unique
        let key_ids: Vec<String> = results.into_iter().map(|r| r.unwrap()).collect();

        for i in 0..key_ids.len() {
            for j in (i + 1)..key_ids.len() {
                assert_ne!(key_ids[i], key_ids[j]);
            }
        }
    }

    /// Test 16: Concurrent key retrieval (read-only operations)
    #[test]
    fn test_concurrent_key_retrieval() {
        use std::sync::Arc;
        use std::thread;

        let config = MemoryKeyConfig::default();
        let manager = Arc::new(MemoryKeyManager::new(config).unwrap());

        let key_id = manager.generate_key().unwrap();
        let mut handles = vec![];

        for _ in 0..10 {
            let manager_clone = Arc::clone(&manager);
            let key_id_clone = key_id.clone();
            let handle = thread::spawn(move || manager_clone.key_exists(&key_id_clone));
            handles.push(handle);
        }

        let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All should succeed and return true
        assert_eq!(results.len(), 10);
        for result in results {
            assert!(result.is_ok());
            assert!(result.unwrap());
        }
    }

    /// Test 17: Default config values
    #[test]
    fn test_default_config() {
        let config = MemoryKeyConfig::default();

        assert_eq!(config.max_keys, 1000);
        assert_eq!(config.key_expiration_seconds, 0);
        assert!(!config.enable_rotation);
    }

    /// Test 18: Key metadata types
    #[test]
    fn test_key_metadata_types() {
        let config = MemoryKeyConfig::default();
        let mut manager = MemoryKeyManager::new(config).unwrap();

        let key_types = vec!["AES256", "RSA2048", "ED25519", "HMAC", "Custom"];

        for key_type in key_types {
            let metadata = KeyMetadata {
                id: format!("key_{}", key_type),
                created_at: Utc::now(),
                key_type: key_type.to_string(),
            };

            let key_data = format!("data_for_{}", key_type).into_bytes();
            let key_id = manager.store_key(&key_data, metadata).unwrap();

            let retrieved = manager.get_key(&key_id).unwrap();
            assert_eq!(retrieved, key_data);
        }
    }

    /// Test 19: Zero-sized config values
    #[test]
    fn test_zero_config_values() {
        let config = MemoryKeyConfig {
            max_keys: 0,
            key_expiration_seconds: 0,
            enable_rotation: false,
        };

        // Manager should still be creatable
        let manager = MemoryKeyManager::new(config).unwrap();
        let key_id = manager.generate_key().unwrap();
        assert!(manager.key_exists(&key_id).unwrap());
    }

    /// Test 20: Very large config values
    #[test]
    fn test_large_config_values() {
        let config = MemoryKeyConfig {
            max_keys: 1_000_000,
            key_expiration_seconds: u64::MAX,
            enable_rotation: true,
        };

        let manager = MemoryKeyManager::new(config).unwrap();
        let key_id = manager.generate_key().unwrap();
        assert!(manager.key_exists(&key_id).unwrap());
    }
}
