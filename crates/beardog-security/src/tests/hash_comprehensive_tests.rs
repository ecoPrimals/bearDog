// SPDX-License-Identifier: AGPL-3.0-only

//! Hash Function Comprehensive Tests
//!
//! Extensive test coverage for SHA-256 and SHA-512 hashing operations

use crate::*;

#[cfg(test)]
mod sha256_comprehensive {
    use super::*;

    #[test]
    fn test_sha256_empty_input() -> Result<(), BearDogError> {
        let hash = compute_sha256_hash(b"")?;
        assert_eq!(hash.len(), 32);
        Ok(())
    }

    #[test]
    fn test_sha256_single_byte_inputs() -> Result<(), BearDogError> {
        for byte in 0u8..=255u8 {
            let hash = compute_sha256_hash(&[byte])?;
            assert_eq!(hash.len(), 32);
            assert_ne!(hash, vec![0u8; 32], "Hash should not be all zeros");
        }
        Ok(())
    }

    #[test]
    fn test_sha256_deterministic() -> Result<(), BearDogError> {
        let data = b"deterministic test";
        let hash1 = compute_sha256_hash(data)?;
        let hash2 = compute_sha256_hash(data)?;
        let hash3 = compute_sha256_hash(data)?;

        assert_eq!(hash1, hash2);
        assert_eq!(hash2, hash3);
        Ok(())
    }

    #[test]
    fn test_sha256_collision_resistance() -> Result<(), BearDogError> {
        // Test that similar inputs produce very different hashes
        let data1 = b"test";
        let data2 = b"test ";
        let data3 = b" test";

        let hash1 = compute_sha256_hash(data1)?;
        let hash2 = compute_sha256_hash(data2)?;
        let hash3 = compute_sha256_hash(data3)?;

        assert_ne!(hash1, hash2);
        assert_ne!(hash2, hash3);
        assert_ne!(hash1, hash3);
        Ok(())
    }

    #[test]
    fn test_sha256_various_sizes() -> Result<(), BearDogError> {
        let sizes = vec![0, 1, 10, 100, 256, 1000, 4096, 10000];

        for size in sizes {
            let data = vec![0xAB; size];
            let hash = compute_sha256_hash(&data)?;
            assert_eq!(
                hash.len(),
                32,
                "Hash should always be 32 bytes for size {size}"
            );
        }
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_sha256_unicode_text() -> Result<(), BearDogError> {
        let texts = vec![
            "Hello, World!",
            "你好世界",
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            "مرحبا بالعالم",
            "Привет мир",
            "🌍🌎🌏",
            "Ñoño",
        ];

        for text in texts {
            let hash = compute_sha256_hash(text.as_bytes())?;
            assert_eq!(hash.len(), 32);
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
        }
        Ok(())
    }

    #[test]
    fn test_sha256_binary_patterns() -> Result<(), BearDogError> {
        let patterns = vec![
            vec![0x00; 32],
            vec![0xFF; 32],
            vec![0xAA; 32],
            vec![0x55; 32],
            (0..32).collect::<Vec<u8>>(),
            (0..32).rev().collect::<Vec<u8>>(),
        ];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        for pattern in patterns {
            let hash = compute_sha256_hash(&pattern)?;
            assert_eq!(hash.len(), 32);
        }
        Ok(())
    }

    #[test]
    fn test_sha256_large_input() -> Result<(), BearDogError> {
        let large_data = vec![0x42; 1024 * 1024]; // 1 MB
        let hash = compute_sha256_hash(&large_data)?;
        assert_eq!(hash.len(), 32);
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_sha256_avalanche_effect() -> Result<(), BearDogError> {
        // Small input change should cause large hash change
        let data1 = b"avalanche test";
        let mut data2 = data1.to_vec();
        data2[0] ^= 0x01; // Flip one bit

        let hash1 = compute_sha256_hash(data1)?;
        let hash2 = compute_sha256_hash(&data2)?;

        // Count different bits
        let diff_bits: u32 = hash1
            .iter()
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            .zip(hash2.iter())
            .map(|(a, b)| (a ^ b).count_ones())
            .sum();

        // Should differ in many bits (avalanche effect)
        assert!(
            diff_bits > 50,
            "Only {diff_bits} bits differ, avalanche effect weak"
        );
        Ok(())
    }

    #[test]
    fn test_sha256_concurrent_hashing() -> Result<(), BearDogError> {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // Test that hashing is thread-safe
        use std::sync::Arc;
        use std::thread;

        let data = Arc::new(b"concurrent test".to_vec());
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let mut handles = vec![];

        for _ in 0..10 {
            let data_clone = Arc::clone(&data);
            let handle = thread::spawn(move || compute_sha256_hash(&data_clone).unwrap());
            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            results.push(handle.join().unwrap());
        }

        // All results should be identical
        for i in 1..results.len() {
            assert_eq!(results[0], results[i]);
        }
        Ok(())
    }
}

#[cfg(test)]
mod sha512_comprehensive {
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    use super::*;

    #[test]
    fn test_sha512_empty_input() -> Result<(), BearDogError> {
        let hash = compute_sha512_hash(b"")?;
        assert_eq!(hash.len(), 64);
        Ok(())
    }

    #[test]
    fn test_sha512_deterministic() -> Result<(), BearDogError> {
        let data = b"deterministic test";
        let hash1 = compute_sha512_hash(data)?;
        let hash2 = compute_sha512_hash(data)?;

        assert_eq!(hash1, hash2);
        Ok(())
    }

    #[test]
    fn test_sha512_vs_sha256_different() -> Result<(), BearDogError> {
        let data = b"test data";
        let hash256 = compute_sha256_hash(data)?;
        let hash512 = compute_sha512_hash(data)?;

        assert_eq!(hash256.len(), 32);
        assert_eq!(hash512.len(), 64);
        assert_ne!(&hash512[..32], &hash256[..]);
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_sha512_large_input() -> Result<(), BearDogError> {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let large_data = vec![0x55; 1024 * 1024]; // 1 MB
        let hash = compute_sha512_hash(&large_data)?;
        assert_eq!(hash.len(), 64);
        Ok(())
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_sha512_various_sizes() -> Result<(), BearDogError> {
        let sizes = vec![0, 1, 10, 100, 512, 1000, 4096];

        for size in sizes {
            let data = vec![0xCD; size];
            let hash = compute_sha512_hash(&data)?;
            assert_eq!(
                hash.len(),
                64,
                // TEST_CATEGORY: integration
                // TEST_DOMAIN: security
                // TEST_PRIORITY: normal
                "Hash should always be 64 bytes for size {size}"
            );
        }
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }
}

#[cfg(test)]
mod hash_comparison_tests {
    use super::*;

    #[test]
    fn test_same_input_different_algorithms() -> Result<(), BearDogError> {
        let data = b"compare algorithms";

        let hash256 = compute_sha256_hash(data)?;
        let hash512 = compute_sha512_hash(data)?;

        assert_eq!(hash256.len(), 32);
        assert_eq!(hash512.len(), 64);

        // Hashes from different algorithms should be different
        assert_ne!(&hash512[..32], &hash256[..]);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        Ok(())
    }

    #[test]
    fn test_hash_consistency_across_calls() -> Result<(), BearDogError> {
        let data = b"consistency test";

        // Multiple calls to each algorithm
        let sha256_results: Vec<_> = (0..5).map(|_| compute_sha256_hash(data).unwrap()).collect();

        let sha512_results: Vec<_> = (0..5).map(|_| compute_sha512_hash(data).unwrap()).collect();

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // All SHA-256 results should match
        for i in 1..sha256_results.len() {
            assert_eq!(sha256_results[0], sha256_results[i]);
        }

        // All SHA-512 results should match
        for i in 1..sha512_results.len() {
            assert_eq!(sha512_results[0], sha512_results[i]);
        }

        Ok(())
    }
}

#[cfg(test)]
mod edge_cases {
    use super::*;

    #[test]
    fn test_hash_null_bytes() -> Result<(), BearDogError> {
        let data = vec![0x00; 100];
        let hash256 = compute_sha256_hash(&data)?;
        let hash512 = compute_sha512_hash(&data)?;

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert_eq!(hash256.len(), 32);
        assert_eq!(hash512.len(), 64);
        Ok(())
    }

    #[test]
    fn test_hash_max_bytes() -> Result<(), BearDogError> {
        let data = vec![0xFF; 100];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let hash256 = compute_sha256_hash(&data)?;
        let hash512 = compute_sha512_hash(&data)?;

        assert_eq!(hash256.len(), 32);
        assert_eq!(hash512.len(), 64);
        Ok(())
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_hash_alternating_pattern() -> Result<(), BearDogError> {
        let data: Vec<u8> = (0..100)
            .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
            .collect();
        let hash256 = compute_sha256_hash(&data)?;
        let hash512 = compute_sha512_hash(&data)?;

        assert_eq!(hash256.len(), 32);
        assert_eq!(hash512.len(), 64);
        Ok(())
    }
}
