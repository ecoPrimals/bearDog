// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key management edge cases

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use super::*;

#[test]
fn test_key_generation_uniqueness() {
    // Test that generated keys are unique
    let key1 = generate_test_key();
    let key2 = generate_test_key();

    assert!(
        key1.is_ok() && key2.is_ok(),
        "Key generation should succeed"
    );
    if let (Ok(k1), Ok(k2)) = (key1, key2) {
        assert_ne!(k1, k2, "Generated keys should be unique");
    }
}

#[test]
fn test_key_generation_correct_length() {
    // Test that generated keys have correct length
    let key = generate_test_key();

    if let Ok(k) = key {
        assert_eq!(k.len(), 32, "Key should be 32 bytes for AES-256");
    }
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: critical

#[test]
fn test_key_deletion_nonexistent() {
    // Test deletion of non-existent key
    let result = delete_nonexistent_key("nonexistent_key_id");
    assert!(
        result.is_err(),
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        "Should error when deleting non-existent key"
    );
}

#[test]
fn test_key_rotation_failure_recovery() {
    // Test that failed key rotation can be recovered
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    let original_key = generate_test_key().expect("Should generate key");

    // Simulate rotation failure
    let rotation_result = simulate_failed_key_rotation(&original_key);
    assert!(rotation_result.is_err(), "Rotation should fail");

    // Original key should still be usable
    let data = b"test";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    let encrypt_result = attempt_encryption(data, &original_key);
    assert!(encrypt_result.is_ok(), "Original key should still work");
}

#[test]
fn test_concurrent_key_access() {
    // Test concurrent access to the same key
    let key_id = "shared_key".to_string();
    let success = Arc::new(AtomicBool::new(true));

    let handles: Vec<_> = (0..10)
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        .map(|_| {
            let key_id_clone = key_id.clone();
            let success_clone = Arc::clone(&success);
            std::thread::spawn(move || {
                let result = access_key(&key_id_clone);
                if result.is_err() {
                    success_clone.store(false, Ordering::SeqCst);
                }
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            })
        })
        .collect();

    for handle in handles {
        handle.join().expect("Thread should complete");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical

    assert!(
        success.load(Ordering::SeqCst),
        "Concurrent access should succeed"
    );
}
