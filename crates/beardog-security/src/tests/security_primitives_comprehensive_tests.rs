use crate::{compute_sha256_hash, compute_sha512_hash, generate_secure_random_bytes};

#[cfg(test)]
mod security_primitives_tests {
    use super::*;

    // ============================================================================
    // SHA-256 Hash Tests
    // ============================================================================

    #[test]
    fn test_sha256_empty_input() {
        let data = b"";
        let hash = compute_sha256_hash(data).expect("Failed to hash empty data");
        assert_eq!(hash.len(), 32, "SHA-256 should always produce 32 bytes");

        // Empty string SHA-256 is a known value
        let expected =
            hex::decode("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
                .expect("Failed to decode expected hash");
        assert_eq!(
            hash, expected,
            "Empty input should produce known SHA-256 hash"
        );
    }

    #[test]
    fn test_sha256_known_input() {
        let data = b"Hello, World!";
        let hash = compute_sha256_hash(data).expect("Failed to hash data");
        assert_eq!(hash.len(), 32);

        // Known SHA-256 for "Hello, World!"
        let expected =
            hex::decode("dffd6021bb2bd5b0af676290809ec3a53191dd81c7f70a4b28688a362182986f")
                .expect("Failed to decode expected hash");
        assert_eq!(hash, expected);
    }

    #[test]
    fn test_sha256_large_input() {
        let data = vec![0u8; 1_000_000]; // 1MB of zeros
        let hash = compute_sha256_hash(&data).expect("Failed to hash large data");
        assert_eq!(hash.len(), 32);
    }

    #[test]
    fn test_sha256_deterministic() {
        let data = b"Deterministic test";
        let hash1 = compute_sha256_hash(data).expect("Failed first hash");
        let hash2 = compute_sha256_hash(data).expect("Failed second hash");
        assert_eq!(hash1, hash2, "SHA-256 must be deterministic");
    }

    #[test]
    fn test_sha256_different_inputs() {
        let hash1 = compute_sha256_hash(b"input1").expect("Failed hash1");
        let hash2 = compute_sha256_hash(b"input2").expect("Failed hash2");
        assert_ne!(
            hash1, hash2,
            "Different inputs must produce different hashes"
        );
    }

    #[test]
    fn test_sha256_unicode_input() {
        let data = "Hello, 世界! 🦀".as_bytes();
        let hash = compute_sha256_hash(data).expect("Failed to hash Unicode");
        assert_eq!(hash.len(), 32);
    }

    // ============================================================================
    // SHA-512 Hash Tests
    // ============================================================================

    #[test]
    fn test_sha512_empty_input() {
        let data = b"";
        let hash = compute_sha512_hash(data).expect("Failed to hash empty data");
        assert_eq!(hash.len(), 64, "SHA-512 should always produce 64 bytes");
    }

    #[test]
    fn test_sha512_known_input() {
        let data = b"Hello, World!";
        let hash = compute_sha512_hash(data).expect("Failed to hash data");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_sha512_large_input() {
        let data = vec![0u8; 1_000_000]; // 1MB of zeros
        let hash = compute_sha512_hash(&data).expect("Failed to hash large data");
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_sha512_deterministic() {
        let data = b"Deterministic test";
        let hash1 = compute_sha512_hash(data).expect("Failed first hash");
        let hash2 = compute_sha512_hash(data).expect("Failed second hash");
        assert_eq!(hash1, hash2, "SHA-512 must be deterministic");
    }

    #[test]
    fn test_sha512_different_from_sha256() {
        let data = b"Compare algorithms";
        let sha256 = compute_sha256_hash(data).expect("Failed SHA-256");
        let sha512 = compute_sha512_hash(data).expect("Failed SHA-512");

        assert_ne!(sha256.len(), sha512.len());
        assert_eq!(sha256.len(), 32);
        assert_eq!(sha512.len(), 64);
    }

    // ============================================================================
    // Random Bytes Generation Tests
    // ============================================================================

    #[test]
    fn test_generate_random_bytes_length() {
        for size in [8, 16, 32, 64, 128, 256] {
            let bytes =
                generate_secure_random_bytes(size).expect("Failed to generate random bytes");
            assert_eq!(
                bytes.len(),
                size,
                "Random bytes should match requested size"
            );
        }
    }

    #[test]
    fn test_generate_random_bytes_zero_length() {
        let bytes = generate_secure_random_bytes(0).expect("Failed to generate zero bytes");
        assert_eq!(bytes.len(), 0);
    }

    #[test]
    fn test_generate_random_bytes_uniqueness() {
        let bytes1 = generate_secure_random_bytes(32).expect("Failed first generation");
        let bytes2 = generate_secure_random_bytes(32).expect("Failed second generation");

        assert_ne!(bytes1, bytes2, "Random bytes should be unique");
    }

    #[test]
    fn test_generate_random_bytes_distribution() {
        // Generate random bytes and check they're not all the same
        let bytes = generate_secure_random_bytes(256).expect("Failed to generate bytes");

        let first_byte = bytes[0];
        let all_same = bytes.iter().all(|&b| b == first_byte);

        assert!(!all_same, "Random bytes should have varied values");
    }

    #[test]
    fn test_generate_random_bytes_large() {
        let size = 1_000_000; // 1MB
        let bytes = generate_secure_random_bytes(size).expect("Failed to generate large random");
        assert_eq!(bytes.len(), size);
    }

    // ============================================================================
    // Cross-Function Integration Tests
    // ============================================================================

    #[test]
    fn test_hash_random_bytes() {
        let random = generate_secure_random_bytes(1024).expect("Failed to generate random");
        let hash256 = compute_sha256_hash(&random).expect("Failed to hash random");
        let hash512 = compute_sha512_hash(&random).expect("Failed to hash random");

        assert_eq!(hash256.len(), 32);
        assert_eq!(hash512.len(), 64);
    }

    #[test]
    fn test_hash_consistency_with_random() {
        let random = generate_secure_random_bytes(64).expect("Failed to generate");

        let hash1 = compute_sha256_hash(&random).expect("Failed hash1");
        let hash2 = compute_sha256_hash(&random).expect("Failed hash2");

        assert_eq!(
            hash1, hash2,
            "Hashing same random data should be consistent"
        );
    }

    // ============================================================================
    // Error Handling Tests
    // ============================================================================

    #[test]
    fn test_hash_functions_never_fail_on_valid_input() {
        // These should never fail with valid inputs
        let inputs = vec![
            b"".to_vec(),
            b"short".to_vec(),
            b"a".repeat(10000),
            vec![0u8; 1000],
            vec![255u8; 1000],
        ];

        for input in inputs {
            assert!(compute_sha256_hash(&input).is_ok());
            assert!(compute_sha512_hash(&input).is_ok());
        }
    }

    // ============================================================================
    // Performance Baseline Tests
    // ============================================================================

    #[test]
    fn test_hash_performance_baseline() {
        use std::time::Instant;

        let data = vec![0u8; 1_000_000]; // 1MB

        let start = Instant::now();
        let _ = compute_sha256_hash(&data).expect("Failed hash");
        let duration = start.elapsed();

        // Should complete in reasonable time (< 100ms for 1MB on modern hardware)
        assert!(
            duration.as_millis() < 1000,
            "Hash should be reasonably fast"
        );
    }

    #[test]
    fn test_random_generation_performance() {
        use std::time::Instant;

        let start = Instant::now();
        let _ = generate_secure_random_bytes(1_000_000).expect("Failed generation");
        let duration = start.elapsed();

        // Should complete in reasonable time
        assert!(
            duration.as_millis() < 1000,
            "Random generation should be reasonably fast"
        );
    }

    // ============================================================================
    // Edge Cases
    // ============================================================================

    #[test]
    fn test_sha256_max_chunk_boundary() {
        // SHA-256 processes in 512-bit (64-byte) chunks
        for size in [63, 64, 65, 127, 128, 129] {
            let data = vec![0u8; size];
            let hash = compute_sha256_hash(&data).expect("Failed at chunk boundary");
            assert_eq!(hash.len(), 32);
        }
    }

    #[test]
    fn test_sha512_max_chunk_boundary() {
        // SHA-512 processes in 1024-bit (128-byte) chunks
        for size in [127, 128, 129, 255, 256, 257] {
            let data = vec![0u8; size];
            let hash = compute_sha512_hash(&data).expect("Failed at chunk boundary");
            assert_eq!(hash.len(), 64);
        }
    }
}
