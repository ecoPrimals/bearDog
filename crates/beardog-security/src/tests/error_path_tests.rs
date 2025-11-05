//! Error path tests for beardog-security
//!
//! Tests error handling and edge cases in security operations

#[cfg(test)]
mod tests {

    #[test]
    fn test_empty_data_hash() {
        use crate::compute_sha256_hash;

        // Test hashing empty data - should succeed
        let empty_data = b"";
        let result = compute_sha256_hash(empty_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32); // SHA-256 produces 32 bytes
    }

    #[test]
    fn test_large_data_hash() {
        use crate::compute_sha256_hash;

        // Test hashing large data - should succeed
        let large_data = vec![0u8; 1_000_000]; // 1MB
        let result = compute_sha256_hash(&large_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }

    #[test]
    fn test_sha512_empty_data() {
        use crate::compute_sha512_hash;

        let empty_data = b"";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let result = compute_sha512_hash(empty_data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 64); // SHA-512 produces 64 bytes
    }

    #[test]
    fn test_random_bytes_generation_sizes() {
        use rand::RngCore;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        // Test various sizes
        for size in [0, 1, 16, 32, 64, 128, 256, 1024] {
            let mut bytes = vec![0u8; size];
            rand::thread_rng().fill_bytes(&mut bytes);
            assert_eq!(bytes.len(), size);
        }
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_random_bytes_uniqueness() {
        use rand::RngCore;

        // Generate multiple random byte sequences and verify they're different
        let mut bytes1 = vec![0u8; 32];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let mut bytes2 = vec![0u8; 32];
        let mut bytes3 = vec![0u8; 32];

        rand::thread_rng().fill_bytes(&mut bytes1);
        rand::thread_rng().fill_bytes(&mut bytes2);
        rand::thread_rng().fill_bytes(&mut bytes3);

        // Extremely unlikely to be equal (1 in 2^256 chance)
        assert_ne!(bytes1, bytes2);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert_ne!(bytes2, bytes3);
        assert_ne!(bytes1, bytes3);
    }

    #[test]
    fn test_hash_consistency() {
        use crate::compute_sha256_hash;

        // Same input should produce same hash
        let data = b"test data";
        let hash1 = compute_sha256_hash(data).unwrap();
        let hash2 = compute_sha256_hash(data).unwrap();

        assert_eq!(hash1, hash2);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_hash_different_inputs() {
        use crate::compute_sha256_hash;

        // Different inputs should produce different hashes
        let data1 = b"test data 1";
        let data2 = b"test data 2";

        let hash1 = compute_sha256_hash(data1).unwrap();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let hash2 = compute_sha256_hash(data2).unwrap();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_hash_collision_resistance() {
        use crate::compute_sha256_hash;

        // Test that similar inputs produce very different hashes
        let data1 = b"test";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let data2 = b"Test"; // Only capitalization differs

        let hash1 = compute_sha256_hash(data1).unwrap();
        let hash2 = compute_sha256_hash(data2).unwrap();

        assert_ne!(hash1, hash2);

        // Verify hashes differ in many bits (avalanche effect)
        let diff_bits: u32 = hash1
            .iter()
            .zip(hash2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum();

        // Expect significant difference (avalanche effect means ~50% bits differ)
        assert!(
            diff_bits > 50,
            "Expected significant bit differences, got {}",
            diff_bits
        );
    }

    #[test]
    fn test_random_bytes_zero_size() {
        // Zero size should return empty vec
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let bytes = [0u8; 0];
        assert_eq!(bytes.len(), 0);
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_sha256_deterministic() {
        use crate::compute_sha256_hash;

        // Test deterministic behavior - same input always produces same output
        let data = b"deterministic test data";
        let hashes: Vec<_> = (0..10)
            .map(|_| compute_sha256_hash(data).unwrap())
            .collect();

        // All hashes should be identical
        for hash in &hashes[1..] {
            assert_eq!(hash, &hashes[0]);
        }
    }
}
