// SPDX-License-Identifier: AGPL-3.0-only

//! Key Management Tests
//!
//! Comprehensive testing of key lifecycle management including:
//! - Key generation and storage
//! - Key retrieval and deletion
//! - Key metadata management
//! - Recovery key management
//! - Key rotation scenarios
//! - Concurrent access handling

use crate::memory_key_manager::{KeyMetadata, MemoryKeyConfig, MemoryKeyManager};

// ============================================================================
// Key Generation Tests
// ============================================================================

#[test]
fn test_key_generation() {
    let config = MemoryKeyConfig::default();
    let manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let key_id = manager
        .generate_key()
        .expect("Key generation should succeed");

    // Verify key ID format
    assert!(
        key_id.starts_with("key_"),
        "Key ID should have correct prefix"
    );
    assert!(key_id.len() > 10, "Key ID should be sufficiently long");
}

#[test]
fn test_multiple_key_generation() {
    let config = MemoryKeyConfig::default();
    let manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let mut key_ids = Vec::new();

    // Generate multiple keys
    for _ in 0..10 {
        let key_id = manager
            .generate_key()
            .expect("Key generation should succeed");
        key_ids.push(key_id);
    }

    // Verify all keys are unique
    for i in 0..key_ids.len() {
        for j in (i + 1)..key_ids.len() {
            assert_ne!(
                key_ids[i], key_ids[j],
                "All generated key IDs should be unique"
            );
        }
    }
}

#[test]
fn test_generated_key_exists() {
    let config = MemoryKeyConfig::default();
    let manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let key_id = manager
        .generate_key()
        .expect("Key generation should succeed");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    let exists = manager
        .key_exists(&key_id)
        .expect("Key existence check should succeed");

    assert!(exists, "Generated key should exist");
}

// ============================================================================
// Key Storage and Retrieval Tests
// ============================================================================

#[test]
fn test_store_and_retrieve_key() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let key_data = b"test_key_data_1234567890";
    let metadata = KeyMetadata {
        id: "test_key".to_string(),
        created_at: chrono::Utc::now(),
        key_type: "aes256".to_string(),
    };

    // Store key
    let key_id = manager
        .store_key(key_data, metadata)
        .expect("Key storage should succeed");

    // Retrieve key
    let retrieved = manager
        .get_key(&key_id)
        .expect("Key retrieval should succeed");

    assert_eq!(retrieved, key_data, "Retrieved key should match stored key");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_store_empty_key() {
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let key_data = b"";
    let metadata = KeyMetadata {
        id: "empty_key".to_string(),
        created_at: chrono::Utc::now(),
        key_type: "test".to_string(),
    };

    let key_id = manager
        .store_key(key_data, metadata)
        .expect("Empty key storage should succeed");

    let retrieved = manager
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        .get_key(&key_id)
        .expect("Empty key retrieval should succeed");

    assert_eq!(retrieved.len(), 0, "Empty key should be stored correctly");
}

#[test]
fn test_store_large_key() {
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    // Large key (1 MB)
    let key_data = vec![0xABu8; 1024 * 1024];
    let metadata = KeyMetadata {
        id: "large_key".to_string(),
        created_at: chrono::Utc::now(),
        key_type: "large".to_string(),
    };

    let key_id = manager
        .store_key(&key_data, metadata)
        .expect("Large key storage should succeed");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    let retrieved = manager
        .get_key(&key_id)
        .expect("Large key retrieval should succeed");

    assert_eq!(
        retrieved.len(),
        key_data.len(),
        "Large key should be stored completely"
    );
    assert_eq!(retrieved, key_data, "Large key data should match");
}

#[test]
fn test_retrieve_nonexistent_key() {
    let config = MemoryKeyConfig::default();
    let manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let result = manager.get_key("nonexistent_key");
    assert!(result.is_err(), "Retrieving nonexistent key should fail");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
}

#[test]
fn test_multiple_keys_storage() {
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let mut stored_keys = Vec::new();

    // Store multiple keys
    for i in 0..5 {
        let key_data = format!("key_data_{i}").into_bytes();
        let metadata = KeyMetadata {
            id: format!("key_{i}"),
            created_at: chrono::Utc::now(),
            key_type: "test".to_string(),
        };

        let key_id = manager
            .store_key(&key_data, metadata)
            .expect("Key storage should succeed");
        stored_keys.push((key_id, key_data));
    }

    // Retrieve and verify all keys
    for (key_id, expected_data) in stored_keys {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let retrieved = manager
            .get_key(&key_id)
            .expect("Key retrieval should succeed");
        assert_eq!(
            retrieved, expected_data,
            "Retrieved key should match stored key" // TEST_CATEGORY: integration
                                                    // TEST_DOMAIN: security
                                                    // TEST_PRIORITY: normal
        );
    }
}

// ============================================================================
// Key Deletion Tests
// ============================================================================

#[test]
fn test_delete_key() {
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let key_data = b"test_key_to_delete";
    let metadata = KeyMetadata {
        id: "delete_test".to_string(),
        created_at: chrono::Utc::now(),
        key_type: "test".to_string(),
    };

    let key_id = manager
        .store_key(key_data, metadata)
        .expect("Key storage should succeed");

    // Verify key exists
    assert!(
        manager.key_exists(&key_id).unwrap(),
        "Key should exist before deletion"
    );

    // Delete key
    manager
        .delete_key(&key_id)
        .expect("Key deletion should succeed");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // Verify key no longer exists
    assert!(
        !manager.key_exists(&key_id).unwrap(),
        "Key should not exist after deletion"
    );
}

#[test]
fn test_delete_nonexistent_key() {
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    // Deleting nonexistent key should not error (idempotent)
    let result = manager.delete_key("nonexistent_key");
    assert!(
        result.is_ok(),
        "Deleting nonexistent key should be idempotent"
    );
}

#[test]
fn test_delete_and_recreate_key() {
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let key_data1 = b"first_key_data";
    let key_data2 = b"second_key_data";
    let metadata = KeyMetadata {
        id: "test_key".to_string(),
        created_at: chrono::Utc::now(),
        key_type: "test".to_string(),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    };

    // Store first key
    let key_id1 = manager
        .store_key(key_data1, metadata.clone())
        .expect("First key storage should succeed");

    // Delete key
    manager
        .delete_key(&key_id1)
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        .expect("Key deletion should succeed");

    // Store new key (will have different ID)
    let key_id2 = manager
        .store_key(key_data2, metadata)
        .expect("Second key storage should succeed");

    // New key should have different ID
    assert_ne!(key_id1, key_id2, "Recreated key should have new ID");

    // Should be able to retrieve new key
    let retrieved = manager
        .get_key(&key_id2)
        .expect("New key retrieval should succeed");
    assert_eq!(retrieved, key_data2, "New key data should be correct");
}

// ============================================================================
// Key Existence Tests
// ============================================================================

#[test]
fn test_key_exists() {
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let key_data = b"test_key";
    let metadata = KeyMetadata {
        id: "exists_test".to_string(),
        created_at: chrono::Utc::now(),
        key_type: "test".to_string(),
    };

    let key_id = manager
        .store_key(key_data, metadata)
        .expect("Key storage should succeed");

    assert!(
        manager.key_exists(&key_id).unwrap(),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        "Stored key should exist"
    );
    assert!(
        !manager.key_exists("nonexistent").unwrap(),
        "Nonexistent key should not exist"
    );
}

// Recovery key management tests are blocked on recovery module naming conflict
// (both recovery.rs and recovery/mod.rs exist). Tracked for the large-file
// refactoring pass.

// ============================================================================
// Key Configuration Tests
// ============================================================================

#[test]
fn test_key_config_defaults() {
    let config = MemoryKeyConfig::default();

    assert_eq!(config.max_keys, 1000, "Default max keys should be 1000");
    assert_eq!(
        config.key_expiration_seconds, 0,
        "Default expiration should be 0 (no expiry)"
    );
    assert!(
        !config.enable_rotation,
        "Rotation should be disabled by default"
    );
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_custom_key_config() {
    let config = MemoryKeyConfig {
        max_keys: 500,
        key_expiration_seconds: 3600,
        enable_rotation: true,
    };

    let manager =
        MemoryKeyManager::new(config).expect("Key manager with custom config should succeed");

    // Manager should be created successfully with custom config
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert!(
        manager.generate_key().is_ok(),
        "Key generation with custom config should work"
    );
}

// ============================================================================
// Concurrent Access Tests
// ============================================================================

#[test]
fn test_concurrent_key_generation() {
    use std::sync::Arc;
    use std::thread;

    let config = MemoryKeyConfig::default();
    let manager =
        Arc::new(MemoryKeyManager::new(config).expect("Key manager creation should succeed"));

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let mut handles = vec![];

    // Generate keys concurrently
    for _ in 0..10 {
        let manager_clone = Arc::clone(&manager);
        let handle = thread::spawn(move || manager_clone.generate_key());
        handles.push(handle);
    }

    // Collect results
    let mut key_ids = Vec::new();
    for handle in handles {
        let key_id = handle
            .join()
            .expect("Thread should complete")
            .expect("Key generation should succeed");
        key_ids.push(key_id);
    }

    // Verify all keys are unique
    for i in 0..key_ids.len() {
        for j in (i + 1)..key_ids.len() {
            assert_ne!(
                key_ids[i], key_ids[j],
                "Concurrently generated keys should be unique"
            );
        }
    }
}

#[test]
fn test_concurrent_key_access() {
    use std::sync::Arc;
    use std::thread;

    let config = MemoryKeyConfig::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let manager =
        Arc::new(MemoryKeyManager::new(config).expect("Key manager creation should succeed"));

    // Generate a key
    let key_id = manager
        .generate_key()
        .expect("Key generation should succeed");

    let mut handles = vec![];

    // Access key concurrently
    for _ in 0..10 {
        let manager_clone = Arc::clone(&manager);
        let key_id_clone = key_id.clone();
        let handle = thread::spawn(move || manager_clone.key_exists(&key_id_clone));
        handles.push(handle);
    }

    // All accesses should succeed
    for handle in handles {
        let exists = handle
            .join()
            .expect("Thread should complete")
            .expect("Key existence check should succeed");
        assert!(exists, "Key should exist in concurrent access");
    }
}

// ============================================================================
// Edge Cases and Error Handling
// ============================================================================

#[test]
fn test_empty_key_id_retrieval() {
    let config = MemoryKeyConfig::default();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    let result = manager.get_key("");
    assert!(result.is_err(), "Retrieving with empty key ID should fail");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_key_lifecycle_complete() {
    let config = MemoryKeyConfig::default();
    let mut manager = MemoryKeyManager::new(config).expect("Key manager creation should succeed");

    // Generate
    let key_id = manager
        .generate_key()
        .expect("Key generation should succeed");

    // Verify exists
    assert!(manager.key_exists(&key_id).unwrap());

    // Retrieve
    let _key_data = manager
        .get_key(&key_id)
        .expect("Key retrieval should succeed");

    // Delete
    manager
        .delete_key(&key_id)
        .expect("Key deletion should succeed");

    // Verify deleted
    assert!(!manager.key_exists(&key_id).unwrap());

    // Retrieval should fail
    assert!(manager.get_key(&key_id).is_err());
}
