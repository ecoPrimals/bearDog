// SPDX-License-Identifier: AGPL-3.0-only

// Security Integration Tests
// Testing integration between security components

use crate::memory_key_manager::{MemoryKeyManager, config::MemoryKeyConfig};
use crate::*;

#[test]
fn test_sha256_hash_deterministic() {
    let data = b"test data";

    let hash1 = compute_sha256_hash(data).expect("Hash should succeed");
    let hash2 = compute_sha256_hash(data).expect("Hash should succeed");

    assert_eq!(hash1, hash2, "SHA-256 should be deterministic");
}

#[test]
fn test_sha256_hash_length() {
    let data = b"any data";

    let hash = compute_sha256_hash(data).expect("Hash should succeed");

    assert_eq!(hash.len(), 32, "SHA-256 should produce 32 bytes");
}

#[test]
fn test_sha512_hash_deterministic() {
    let data = b"test data";

    let hash1 = compute_sha512_hash(data).expect("Hash should succeed");
    let hash2 = compute_sha512_hash(data).expect("Hash should succeed");

    assert_eq!(hash1, hash2, "SHA-512 should be deterministic");
}

#[test]
fn test_sha512_hash_length() {
    let data = b"any data";

    let hash = compute_sha512_hash(data).expect("Hash should succeed");

    assert_eq!(hash.len(), 64, "SHA-512 should produce 64 bytes");
}

#[test]
fn test_sha256_empty_input() {
    let data = b"";

    let hash = compute_sha256_hash(data).expect("Hash of empty input should succeed");

    assert_eq!(hash.len(), 32);
}

#[test]
fn test_sha512_empty_input() {
    let data = b"";

    let hash = compute_sha512_hash(data).expect("Hash of empty input should succeed");

    assert_eq!(hash.len(), 64);
}

#[test]
fn test_hash_uniqueness() {
    let data1 = b"test data 1";
    let data2 = b"test data 2";

    let hash256_1 = compute_sha256_hash(data1).expect("Hash should succeed");
    let hash256_2 = compute_sha256_hash(data2).expect("Hash should succeed");

    assert_ne!(
        hash256_1, hash256_2,
        "Different inputs should produce different hashes"
    );
}

#[test]
fn test_hash_collision_resistance() {
    let data = b"test";
    let similar_data = b"Test"; // Different case

    let hash1 = compute_sha256_hash(data).expect("Hash should succeed");
    let hash2 = compute_sha256_hash(similar_data).expect("Hash should succeed");

    assert_ne!(
        hash1, hash2,
        "Small differences should produce different hashes" // TEST_CATEGORY: integration
                                                            // TEST_DOMAIN: security
                                                            // TEST_PRIORITY: normal
    );
}

#[test]
fn test_hash_algorithm_difference() {
    let data = b"test data";

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let hash256 = compute_sha256_hash(data).expect("SHA-256 should succeed");
    let hash512 = compute_sha512_hash(data).expect("SHA-512 should succeed");

    assert_ne!(hash256.len(), hash512.len());
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_secure_random_32_bytes() {
    let random = generate_secure_random_bytes(32).expect("Random generation should succeed");

    assert_eq!(random.len(), 32);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_secure_random_unique() {
    let random1 = generate_secure_random_bytes(32).expect("Random generation should succeed");
    let random2 = generate_secure_random_bytes(32).expect("Random generation should succeed");

    assert_ne!(
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        random1,
        random2,
        "Sequential random values should be unique"
    );
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_secure_random_zero_length() {
    let random = generate_secure_random_bytes(0).expect("Zero length request should succeed");

    assert_eq!(random.len(), 0);
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_secure_random_large() {
    let random = generate_secure_random_bytes(4096).expect("Large random request should succeed");

    assert_eq!(random.len(), 4096);
}

#[test]
fn test_key_manager_creation() {
    let config = MemoryKeyConfig::default();
    let _manager = MemoryKeyManager::new(config).expect("MemoryKeyManager creation should succeed");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    // Verify manager is created successfully
    // Test placeholder - actual verification done via expect() above
}

#[test]
fn test_key_generation() {
    let config = MemoryKeyConfig::default();
    let manager = MemoryKeyManager::new(config).expect("MemoryKeyManager creation should succeed");

    let key_id = manager
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        .generate_key()
        .expect("Key generation should succeed");

    assert!(!key_id.is_empty());
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_key_retrieval() {
    let config = MemoryKeyConfig::default();
    let manager = MemoryKeyManager::new(config).expect("MemoryKeyManager creation should succeed");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let key_id = manager
        .generate_key()
        .expect("Key generation should succeed");

    let key = manager
        .get_key(&key_id)
        .expect("Key retrieval should succeed");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert!(!key.is_empty());
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_key_deletion() {
    let config = MemoryKeyConfig::default();
    let mut manager =
        MemoryKeyManager::new(config).expect("MemoryKeyManager creation should succeed");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    let key_id = manager
        .generate_key()
        .expect("Key generation should succeed");

    manager
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        .delete_key(&key_id)
        .expect("Key deletion should succeed");

    let result = manager.get_key(&key_id);

    assert!(result.is_err());
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[ignore = "key rotation not yet implemented in current API"]
fn test_key_rotation() {
    // Test pending key rotation API implementation
    // Placeholder preserved for future implementation
}

#[test]
#[ignore = "key expiration tracking not yet in current API"]
fn test_key_expiration() {
    // Test pending expiration tracking API implementation
    // Placeholder preserved for future implementation
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_key_not_found() {
    let config = MemoryKeyConfig::default();
    let manager = MemoryKeyManager::new(config).expect("MemoryKeyManager creation should succeed");

    let result = manager.get_key("nonexistent_key");

    assert!(result.is_err());
}

#[test]
fn test_encryption_config() {
    let config = EncryptionConfig::default();

    assert!(config.key_size > 0);
    // EncryptionAlgorithm is an enum, not a string - it always has a value
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // Just verify it exists
    let _ = config.algorithm;
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
#[ignore = "SecurityMetrics module reorganized"]
fn test_security_metrics() {
    // Test pending new metrics API stabilization
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // Placeholder preserved for future implementation
}

#[test]
fn test_constant_time_compare() {
    let a = b"secret_data";
    let b = b"secret_data";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    let c = b"different_data";

    assert!(constant_time_compare(a, b), "Equal data should return true");
    assert!(
        !constant_time_compare(a, c),
        "Different data should return false"
    );
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
}

#[test]
fn test_constant_time_compare_different_lengths() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let a = b"short";
    let b = b"longer_string";

    assert!(
        !constant_time_compare(a, b),
        "Different lengths should return false"
    );
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_secure_zero_memory() {
    let mut data = vec![1u8, 2, 3, 4, 5];
    secure_zero_memory(&mut data);

    // Verify all bytes are zeroed
    for &byte in &data {
        assert_eq!(byte, 0);
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
}

#[test]
fn test_key_derivation() {
    let password = b"test_password";
    let salt = b"test_salt";

    let key =
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        derive_key_from_password(password, salt, 1000).expect("Key derivation should succeed");

    assert!(!key.is_empty());
}

#[test]
fn test_key_derivation_deterministic() {
    let password = b"test_password";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let salt = b"test_salt";

    let key1 =
        derive_key_from_password(password, salt, 1000).expect("Key derivation should succeed");
    let key2 =
        derive_key_from_password(password, salt, 1000).expect("Key derivation should succeed");

    assert_eq!(key1, key2, "Key derivation should be deterministic");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_key_derivation_different_salts() {
    let password = b"test_password";
    let salt1 = b"salt1";
    let salt2 = b"salt2";

    let key1 =
        derive_key_from_password(password, salt1, 1000).expect("Key derivation should succeed");
    let key2 =
        derive_key_from_password(password, salt2, 1000).expect("Key derivation should succeed");

    assert_ne!(key1, key2, "Different salts should produce different keys");
}
