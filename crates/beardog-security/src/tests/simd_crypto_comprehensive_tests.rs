// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive SIMD Crypto Tests
//!
//! `TEST_CATEGORY`: unit
//! `TEST_DOMAIN`: security/simd_crypto
//! `TEST_PRIORITY`: high
//!
//! Comprehensive test coverage for Safe SIMD cryptographic operations including:
//! - Hash operations (SHA-256)
//! - ChaCha20 operations
//! - Configuration and stats tracking
//! - Performance and safety guarantees
//! - Edge cases and error handling

use crate::simd_crypto::{SafeCryptoConfig, SafeCryptoEngine};

#[cfg(test)]
mod tests {
    use super::*;

    /// Test 1: Basic engine creation with default config
    #[test]
    fn test_engine_creation_default() {
        let config = SafeCryptoConfig::default();
        let engine = SafeCryptoEngine::new(config);

        let stats = engine.get_stats();
        assert_eq!(stats.get("operations").unwrap(), "0");
        assert_eq!(stats.get("bytes_processed").unwrap(), "0");
        assert_eq!(stats.get("safety").unwrap(), "100% - Memory-safe verified");
    }

    /// Test 2: Custom config creation
    #[test]
    fn test_engine_creation_custom_config() {
        let config = SafeCryptoConfig {
            enable_timing_attack_protection: false,
            use_secure_random: false,
        };

        let engine = SafeCryptoEngine::new(config);
        let stats = engine.get_stats();
        assert_eq!(stats.get("operations").unwrap(), "0");
    }

    /// Test 3: SHA-256 hash basic operation
    #[test]
    fn test_safe_hash_basic() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"Hello, BearDog!";
        let hash = engine.safe_hash(data).unwrap();

        assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes

        let stats = engine.get_stats();
        assert_eq!(stats.get("operations").unwrap(), "1");
        assert_eq!(stats.get("bytes_processed").unwrap(), "15");
    }

    /// Test 4: SHA-256 deterministic output
    #[test]
    fn test_safe_hash_deterministic() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"test data";
        let hash1 = engine.safe_hash(data).unwrap();
        let hash2 = engine.safe_hash(data).unwrap();

        assert_eq!(hash1, hash2);
    }

    /// Test 5: SHA-256 different inputs produce different outputs
    #[test]
    fn test_safe_hash_different_inputs() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let hash1 = engine.safe_hash(b"data1").unwrap();
        let hash2 = engine.safe_hash(b"data2").unwrap();

        assert_ne!(hash1, hash2);
    }

    /// Test 6: SHA-256 empty data
    #[test]
    fn test_safe_hash_empty() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let hash = engine.safe_hash(b"").unwrap();
        assert_eq!(hash.len(), 32);
    }

    /// Test 7: SHA-256 large data
    #[test]
    fn test_safe_hash_large_data() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let large_data = vec![0u8; 10_000];
        let hash = engine.safe_hash(&large_data).unwrap();

        assert_eq!(hash.len(), 32);

        let stats = engine.get_stats();
        assert_eq!(stats.get("bytes_processed").unwrap(), "10000");
    }

    /// Test 8: ChaCha20 basic operation
    #[test]
    fn test_safe_chacha20_basic() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"plaintext data";
        let key = b"encryption_key_32_bytes_long____";

        let encrypted = engine.safe_chacha20(data, key).unwrap();
        assert_eq!(encrypted.len(), data.len());
        assert_ne!(encrypted, data.to_vec());
    }

    /// Test 9: ChaCha20 reversibility (encrypt then decrypt returns original)
    #[test]
    fn test_safe_chacha20_reversibility() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let original = b"test data for encryption";
        // ChaCha20 requires 32-byte key
        let key = b"01234567890123456789012345678901"; // 32 bytes

        let encrypted = engine.safe_chacha20(original, key).unwrap();
        let decrypted = engine.safe_chacha20(&encrypted, key).unwrap();

        assert_eq!(decrypted, original.to_vec());
    }

    /// Test 10: ChaCha20 empty data
    #[test]
    fn test_safe_chacha20_empty() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"";
        let key = b"01234567890123456789012345678901"; // 32 bytes

        let result = engine.safe_chacha20(data, key).unwrap();
        assert!(result.is_empty());
    }

    /// Test 11: ChaCha20 large data
    #[test]
    fn test_safe_chacha20_large_data() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let large_data = vec![1u8; 50_000];
        let key = b"01234567890123456789012345678901"; // 32 bytes

        let encrypted = engine.safe_chacha20(&large_data, key).unwrap();
        assert_eq!(encrypted.len(), large_data.len());

        let stats = engine.get_stats();
        assert_eq!(stats.get("bytes_processed").unwrap(), "50000");
    }

    /// Test 12: Statistics tracking across multiple operations
    #[test]
    fn test_stats_tracking() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);
        let key = b"01234567890123456789012345678901"; // 32 bytes

        engine.safe_hash(b"test1").unwrap();
        engine.safe_hash(b"test2").unwrap();
        engine.safe_chacha20(b"data", key).unwrap();

        let stats = engine.get_stats();
        assert_eq!(stats.get("operations").unwrap(), "3");

        let bytes_processed: u64 = stats.get("bytes_processed").unwrap().parse().unwrap();
        assert_eq!(bytes_processed, 5 + 5 + 4); // test1 + test2 + data
    }

    /// Test: ChaCha20 rejects invalid key length
    #[test]
    fn test_safe_chacha20_invalid_key_length() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"test data";
        let short_key = b"too_short"; // Only 9 bytes

        let result = engine.safe_chacha20(data, short_key);
        assert!(result.is_err());
    }

    /// Test 13: Default config values
    #[test]
    fn test_default_config_values() {
        let config = SafeCryptoConfig::default();

        assert!(config.enable_timing_attack_protection);
        assert!(config.use_secure_random);
    }

    /// Test 14: Config cloning
    #[test]
    fn test_config_clone() {
        let config1 = SafeCryptoConfig {
            enable_timing_attack_protection: false,
            use_secure_random: true,
        };

        let config2 = config1.clone();

        assert_eq!(
            config1.enable_timing_attack_protection,
            config2.enable_timing_attack_protection
        );
        assert_eq!(config1.use_secure_random, config2.use_secure_random);
    }

    /// Test 15: Multiple engines operate independently
    #[test]
    fn test_multiple_engines_independent() {
        let config = SafeCryptoConfig::default();
        let mut engine1 = SafeCryptoEngine::new(config.clone());
        let mut engine2 = SafeCryptoEngine::new(config);

        engine1.safe_hash(b"data").unwrap();
        engine1.safe_hash(b"data").unwrap();

        engine2.safe_hash(b"data").unwrap();

        let stats1 = engine1.get_stats();
        let stats2 = engine2.get_stats();

        assert_eq!(stats1.get("operations").unwrap(), "2");
        assert_eq!(stats2.get("operations").unwrap(), "1");
    }

    /// Test 16: SHA-256 with binary data
    #[test]
    fn test_safe_hash_binary_data() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let binary_data = vec![0xFF, 0x00, 0xAB, 0xCD, 0x12, 0x34];
        let hash = engine.safe_hash(&binary_data).unwrap();

        assert_eq!(hash.len(), 32);
    }

    /// Test 17: ChaCha20 with binary data
    #[test]
    fn test_safe_chacha20_binary_data() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let binary_data = vec![0xFF, 0x00, 0xAB, 0xCD];
        let key = b"01234567890123456789012345678901"; // 32 bytes

        let encrypted = engine.safe_chacha20(&binary_data, key).unwrap();
        assert_eq!(encrypted.len(), binary_data.len());
    }

    /// Test 18: Stats include safety information
    #[test]
    fn test_stats_safety_info() {
        let config = SafeCryptoConfig::default();
        let engine = SafeCryptoEngine::new(config);

        let stats = engine.get_stats();

        assert!(stats.contains_key("safety"));
        assert!(stats.contains_key("performance"));
        assert_eq!(stats.get("safety").unwrap(), "100% - Memory-safe verified");
    }

    /// Test 19: Sequential hash operations maintain correctness
    #[test]
    fn test_sequential_hash_operations() {
        let config = SafeCryptoConfig::default();
        let mut engine = SafeCryptoEngine::new(config);

        let data = b"consistent data";
        let mut hashes = Vec::new();

        for _ in 0..5 {
            let hash = engine.safe_hash(data).unwrap();
            hashes.push(hash);
        }

        // All hashes should be identical (deterministic)
        for i in 1..hashes.len() {
            assert_eq!(hashes[0], hashes[i]);
        }

        let stats = engine.get_stats();
        assert_eq!(stats.get("operations").unwrap(), "5");
    }

    /// Test 20: Timing attack protection enabled by default
    #[test]
    fn test_timing_attack_protection_default() {
        let config = SafeCryptoConfig::default();
        assert!(config.enable_timing_attack_protection);

        // Verify it can be disabled if needed
        let custom_config = SafeCryptoConfig {
            enable_timing_attack_protection: false,
            ..Default::default()
        };
        assert!(!custom_config.enable_timing_attack_protection);
    }
}
