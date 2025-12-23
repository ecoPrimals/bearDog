//! Hash Validation Tests
//!
//! Tests for hash operations, validation, and security properties.

use sha2::{Digest, Sha256};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_deterministic() {
        // Test that hashing is deterministic
        let data = b"test data";

        let hash1 = Sha256::digest(data);
        let hash2 = Sha256::digest(data);

        assert_eq!(hash1, hash2, "Hash should be deterministic");
    }

    #[test]
    fn test_hash_different_inputs_different_outputs() {
        // Test that different inputs produce different hashes
        let data1 = b"test data 1";
        let data2 = b"test data 2";

        let hash1 = Sha256::digest(data1);
        let hash2 = Sha256::digest(data2);

        assert_ne!(
            hash1, hash2,
            "Different inputs should produce different hashes"
        );
    }

    #[test]
    fn test_hash_length_sha256() {
        // Test that SHA-256 produces 32-byte hashes
        let data = b"test data";
        let hash = Sha256::digest(data);

        assert_eq!(hash.len(), 32, "SHA-256 should produce 32-byte hash");
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_hash_empty_input() {
        // Test that hashing empty input works
        let data = b"";
        let hash = Sha256::digest(data);

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert_eq!(
            hash.len(),
            32,
            "Hash of empty input should still be 32 bytes"
        );
    }

    #[test]
    fn test_hash_large_input() {
        // Test that hashing large input works
        let data = vec![0u8; 1024 * 1024]; // 1 MB
        let hash = Sha256::digest(&data);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        assert_eq!(hash.len(), 32, "Hash of large input should be 32 bytes");
    }

    #[test]
    fn test_hash_avalanche_effect() {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // Test avalanche effect - small input change causes large hash change
        let data1 = b"test data";
        let data2 = b"test datb"; // Changed last character

        let hash1 = Sha256::digest(data1);
        let hash2 = Sha256::digest(data2);

        // Count different bits
        let mut diff_bits = 0;
        for i in 0..32 {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            diff_bits += (hash1[i] ^ hash2[i]).count_ones();
        }

        // Should have many different bits (avalanche effect)
        assert!(
            diff_bits > 50,
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            "Should have significant bit differences: {}",
            diff_bits
        );
    }

    #[test]
    fn test_hash_collision_resistance() {
        // Test that we don't easily find collisions
        let mut hashes = std::collections::HashSet::new();

        for i in 0..100 {
            let data = format!("test data {}", i);
            let hash = Sha256::digest(data.as_bytes());

            assert!(
                hashes.insert(hash.to_vec()),
                "Found duplicate hash - collision!"
            );
        }
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_hash_one_way() {
        // Test that hash is one-way (can't reverse)
        // This is a documentation test - we can't actually reverse it
        let data = b"secret data";
        let hash = Sha256::digest(data);

        // Hash should not contain original data
        let hash_contains_data = hash.windows(data.len()).any(|window| window == data);

        assert!(!hash_contains_data, "Hash should not contain original data");
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_hash_consistency_across_calls() {
        // Test consistency across multiple hash operations
        let data = b"test data";
        let mut hashes = Vec::new();

        for _ in 0..10 {
            let hash = Sha256::digest(data);
            hashes.push(hash.to_vec());
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
        }

        // All hashes should be identical
        for hash in &hashes[1..] {
            assert_eq!(hash, &hashes[0], "All hashes should be identical");
        }
    }

    #[test]
    fn test_hash_thread_safety() {
        // Test that hashing is thread-safe
        use std::sync::Arc;
        use std::thread;

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let data = Arc::new(b"test data".to_vec());
        let mut handles = vec![];

        for _ in 0..5 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || Sha256::digest(&*data_clone));
            handles.push(handle);
        }

        let hashes: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();

        // All hashes should be identical
        for hash in &hashes[1..] {
            assert_eq!(hash.as_slice(), hashes[0].as_slice());
        }
    }

    #[test]
    fn test_hash_binary_data() {
        // Test hashing binary data
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let data: Vec<u8> = (0..=255).collect();
        let hash = Sha256::digest(&data);

        assert_eq!(hash.len(), 32);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_hash_unicode_data() {
        // Test hashing Unicode data
        let data = "Hello 世界! 🦀";
        let hash = Sha256::digest(data.as_bytes());

        assert_eq!(hash.len(), 32);
    }
}
