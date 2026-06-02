// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hash function edge cases

use super::*;

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: critical

#[test]
fn test_hash_empty_input() {
    // Test hashing empty input produces valid hash
    let empty: &[u8] = &[];
    let hash = compute_test_hash(empty);

    assert!(hash.is_ok(), "Should hash empty input");
    if let Ok(h) = hash {
        assert_eq!(h.len(), 32, "SHA-256 hash should be 32 bytes");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
}

#[test]
fn test_hash_very_large_input() {
    // Test hashing large input
    let large_input = vec![0u8; 100 * 1024 * 1024]; // 100MB
    let hash = compute_test_hash(&large_input);

    assert!(hash.is_ok(), "Should hash large input");
}

#[test]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
fn test_hash_deterministic() {
    // Test hash function is deterministic
    let data = b"test data for hashing";

    let hash1 = compute_test_hash(data).expect("Should hash");
    let hash2 = compute_test_hash(data).expect("Should hash");

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    assert_eq!(hash1, hash2, "Hash should be deterministic");
}

#[test]
fn test_hash_avalanche_effect() {
    // Test that small input changes produce large hash changes
    let data1 = b"test data";
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: important
    let data2 = b"test datb"; // One bit different

    let hash1 = compute_test_hash(data1).expect("Should hash");
    let hash2 = compute_test_hash(data2).expect("Should hash");

    assert_ne!(hash1, hash2, "Hashes should differ");

    // Count different bits (should be ~50% for good hash)
    let different_bits: u32 = hash1
        .iter()
        .zip(hash2.iter())
        .map(|(a, b)| (a ^ b).count_ones())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        .sum();

    assert!(
        different_bits > 50,
        "Should have significant bit difference"
    );
}
