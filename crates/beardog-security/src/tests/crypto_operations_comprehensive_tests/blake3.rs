// SPDX-License-Identifier: AGPL-3.0-or-later

// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

use beardog_errors::BearDogError;

use super::*;

#[test]
fn test_blake3_hash_generation() -> Result<(), BearDogError> {
    // Test BLAKE3 hash generation
    let data = b"Data to hash with BLAKE3";

    let hash = blake3_hash(data)?;

    assert_eq!(hash.len(), 32, "BLAKE3 hash should be 32 bytes");

    Ok(())
}

#[test]
fn test_blake3_hash_deterministic() -> Result<(), BearDogError> {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    // Test that same input produces same hash
    let data = b"Deterministic input";

    let hash1 = blake3_hash(data)?;
    let hash2 = blake3_hash(data)?;

    assert_eq!(hash1, hash2, "BLAKE3 should be deterministic");

    Ok(())
}

#[test]
fn test_blake3_hash_different_inputs() -> Result<(), BearDogError> {
    // Test that different inputs produce different hashes
    let data1 = b"Input 1";
    let data2 = b"Input 2";

    let hash1 = blake3_hash(data1)?;
    let hash2 = blake3_hash(data2)?;

    assert_ne!(
        hash1, hash2,
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        "Different inputs should produce different hashes"
    );

    Ok(())
}

#[test]
fn test_blake3_hash_empty_input() -> Result<(), BearDogError> {
    // Test hashing empty data
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let data = b"";

    let hash = blake3_hash(data)?;

    assert_eq!(
        hash.len(),
        32,
        "Empty input should still produce 32-byte hash"
    );

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    Ok(())
}

#[test]
fn test_blake3_hash_large_input() -> Result<(), BearDogError> {
    // Test hashing large data (10MB)
    let data = vec![0xAAu8; 10 * 1024 * 1024]; // 10MB

    let hash = blake3_hash(&data)?;

    assert_eq!(hash.len(), 32, "Large input should produce 32-byte hash");

    Ok(())
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

#[test]
fn test_blake3_keyed_hash() -> Result<(), BearDogError> {
    // Test keyed hashing (HMAC-like)
    let data = b"Data to authenticate";
    let key = b"BLAKE3-secret-key-32bytes-here!!"; // Exactly 32 bytes

    let mac = blake3_keyed_hash(key, data)?;

    assert_eq!(mac.len(), 32, "Keyed hash should be 32 bytes");

    Ok(())
}
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal

#[test]
fn test_blake3_keyed_hash_different_keys() -> Result<(), BearDogError> {
    // Test that different keys produce different MACs
    let data = b"Authenticated data";
    let key1 = b"Key-1-for-BLAKE3-authentication!"; // Exactly 32 bytes
    let key2 = b"Key-2-for-BLAKE3-authentication!"; // Exactly 32 bytes

    let mac1 = blake3_keyed_hash(key1, data)?;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    let mac2 = blake3_keyed_hash(key2, data)?;

    assert_ne!(mac1, mac2, "Different keys should produce different MACs");

    Ok(())
}
