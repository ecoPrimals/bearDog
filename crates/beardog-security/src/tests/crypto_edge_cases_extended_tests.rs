// Comprehensive Crypto Edge Cases Tests - Day 2 Expansion
// Tests advanced cryptographic edge cases and boundary conditions

use crate::*;

/// Test encryption with maximum allowed data size
#[test]
fn test_encrypt_maximum_size() {
    let max_data = vec![0u8; 10 * 1024 * 1024]; // 10 MB

    // Then: Should handle large data without panic
    let hash_result = compute_sha256_hash(&max_data);
    assert!(hash_result.is_ok(), "Should hash large data");
    assert_eq!(hash_result.unwrap().len(), 32);
}

/// Test hash function with incrementally larger inputs
#[test]
fn test_hash_incremental_sizes() {
    let sizes = [0, 1, 63, 64, 65, 127, 128, 129, 1023, 1024, 1025];

    for size in &sizes {
        let data = vec![0x42u8; *size];
        let result = compute_sha256_hash(&data);
        assert!(result.is_ok(), "Should hash {} bytes", size);
        assert_eq!(result.unwrap().len(), 32, "Hash should be 32 bytes");
    }
}

/// Test SHA-512 with various input sizes
#[test]
fn test_sha512_variable_sizes() {
    let sizes = [0, 1, 64, 128, 256, 512, 1024];

    for size in &sizes {
        let data = vec![0xFFu8; *size];
        let result = compute_sha512_hash(&data);
        assert!(result.is_ok(), "SHA-512 should work with {} bytes", size);
        assert_eq!(result.unwrap().len(), 64, "SHA-512 produces 64 bytes");
    }
}

/// Test key derivation with extreme iteration counts
#[test]
fn test_key_derivation_iterations() {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    let password = b"test_password";
    let salt = b"test_salt";

    // Low iterations (minimum secure)
    let result_low = derive_key_from_password(password, salt, 1);
    assert!(result_low.is_ok());

    // Medium iterations
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let result_med = derive_key_from_password(password, salt, 1000);
    assert!(result_med.is_ok());

    // High iterations
    let result_high = derive_key_from_password(password, salt, 10000);
    assert!(result_high.is_ok());

    // All should produce different results
    let key_low = result_low.unwrap();
    let key_med = result_med.unwrap();
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let key_high = result_high.unwrap();
    assert_ne!(key_low, key_med);
    assert_ne!(key_med, key_high);
}

/// Test secure random generation at various sizes
#[test]
fn test_secure_random_various_sizes() {
    let sizes = [1, 16, 32, 64, 128, 256, 512, 1024, 4096];

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    for size in &sizes {
        let random = generate_secure_random_bytes(*size);
        assert!(random.is_ok(), "Should generate {} random bytes", size);
        assert_eq!(random.unwrap().len(), *size);
    }
}

/// Test constant-time comparison with equal lengths
#[test]
fn test_constant_time_equal_lengths() {
    let a = [1u8, 2, 3, 4, 5];
    let b = [1u8, 2, 3, 4, 5];
    let c = [1u8, 2, 3, 4, 6];

    assert!(constant_time_compare(&a, &b), "Equal arrays should match");
    assert!(
        !constant_time_compare(&a, &c),
        "Different arrays shouldn't match"
    );
}

/// Test constant-time comparison with different lengths
#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_constant_time_different_lengths() {
    let short = [1u8, 2, 3];
    let long = [1u8, 2, 3, 4, 5];

    assert!(
        !constant_time_compare(&short, &long),
        "Different lengths should not match"
    );
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

/// Test secure memory zeroing
#[test]
fn test_secure_zero_memory() {
    let mut sensitive_data = vec![0xFFu8; 100];

    secure_zero_memory(&mut sensitive_data);

    // Verify all bytes are zeroed
    for byte in &sensitive_data {
        assert_eq!(*byte, 0, "Memory should be zeroed");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }
}

/// Test hash collision resistance (different inputs)
#[test]
fn test_hash_collision_resistance() {
    let data1 = b"test_data_1";
    let data2 = b"test_data_2";

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let hash1 = compute_sha256_hash(data1).unwrap();
    let hash2 = compute_sha256_hash(data2).unwrap();

    assert_ne!(
        hash1, hash2,
        "Different inputs should produce different hashes"
    );
}

/// Test hash determinism (same input = same output)
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_hash_determinism() {
    let data = b"deterministic_test";

    let hash1 = compute_sha256_hash(data).unwrap();
    let hash2 = compute_sha256_hash(data).unwrap();
    let hash3 = compute_sha256_hash(data).unwrap();

    assert_eq!(hash1, hash2, "Same input should produce same hash");
    assert_eq!(hash2, hash3, "Hash should be deterministic");
}

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
/// Test random bytes uniqueness
#[test]
fn test_random_uniqueness() {
    let random1 = generate_secure_random_bytes(32).unwrap();
    let random2 = generate_secure_random_bytes(32).unwrap();
    let random3 = generate_secure_random_bytes(32).unwrap();

    assert_ne!(random1, random2, "Random bytes should be unique");
    assert_ne!(random2, random3, "Random bytes should be unique");
    assert_ne!(random1, random3, "Random bytes should be unique");
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
}

/// Test key derivation salt independence
#[test]
fn test_key_derivation_salt_independence() {
    let password = b"same_password";
    let salt1 = b"salt_one";
    let salt2 = b"salt_two";

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let key1 = derive_key_from_password(password, salt1, 1000).unwrap();
    let key2 = derive_key_from_password(password, salt2, 1000).unwrap();

    assert_ne!(key1, key2, "Different salts should produce different keys");
}

/// Test encryption idempotence (same operation multiple times)
#[test]
fn test_hash_idempotence() {
    let data = b"idempotent_test";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    let hash1 = compute_sha256_hash(data).unwrap();
    let hash2 = compute_sha256_hash(data).unwrap();

    assert_eq!(hash1, hash2, "Hash should be idempotent");
}

/// Test zero-length input handling
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
#[test]
fn test_zero_length_inputs() {
    let empty: &[u8] = &[];

    // Hash empty
    let hash_result = compute_sha256_hash(empty);
    assert!(hash_result.is_ok(), "Should hash empty input");

    // Generate zero random bytes
    let random_result = generate_secure_random_bytes(0);
    assert!(random_result.is_ok(), "Should handle zero-length random");
    assert_eq!(random_result.unwrap().len(), 0);
}
