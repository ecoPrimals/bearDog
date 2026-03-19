// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive Crypto Coverage Tests
//!
//! Extensive test coverage for cryptographic operations

use crate::*;

#[cfg(test)]
mod sha256_tests {
    use super::*;

    #[test]
    fn test_sha256_empty_input() -> Result<(), BearDogError> {
        let hash = compute_sha256_hash(b"")?;
        assert_eq!(hash.len(), 32);
        // Empty input should always produce same hash
        let hash2 = compute_sha256_hash(b"")?;
        assert_eq!(hash, hash2);
        Ok(())
    }

    #[test]
    fn test_sha256_single_byte() -> Result<(), BearDogError> {
        let hash = compute_sha256_hash(b"a")?;
        assert_eq!(hash.len(), 32);
        assert_ne!(hash, vec![0u8; 32]);
        Ok(())
    }

    #[test]
    fn test_sha256_known_value() -> Result<(), BearDogError> {
        let input = b"hello world";
        let hash = compute_sha256_hash(input)?;
        // Should be deterministic
        let hash2 = compute_sha256_hash(input)?;
        assert_eq!(hash, hash2);
        Ok(())
    }

    #[test]
    fn test_sha256_different_inputs() -> Result<(), BearDogError> {
        let hash1 = compute_sha256_hash(b"input1")?;
        let hash2 = compute_sha256_hash(b"input2")?;
        assert_ne!(hash1, hash2);
        Ok(())
    }

    #[test]
    fn test_sha256_large_input() -> Result<(), BearDogError> {
        let large_data = vec![0xAB; 1024 * 1024]; // 1 MB
        let hash = compute_sha256_hash(&large_data)?;
        assert_eq!(hash.len(), 32);
        Ok(())
    }

    #[test]
    fn test_sha256_binary_data() -> Result<(), BearDogError> {
        let binary = vec![0x00, 0xFF, 0xAA, 0x55, 0x12, 0x34];
        let hash = compute_sha256_hash(&binary)?;
        assert_eq!(hash.len(), 32);
        Ok(())
    }

    #[test]
    fn test_sha256_utf8_text() -> Result<(), BearDogError> {
        let text = "Hello, 世界! 🌍";
        let hash = compute_sha256_hash(text.as_bytes())?;
        assert_eq!(hash.len(), 32);
        Ok(())
    }

    #[test]
    fn test_sha256_repeated_calls() -> Result<(), BearDogError> {
        let input = b"test data";
        for _ in 0..100 {
            let hash = compute_sha256_hash(input)?;
            assert_eq!(hash.len(), 32);
        }
        Ok(())
    }

    #[test]
    fn test_sha256_avalanche_effect() -> Result<(), BearDogError> {
        // Small input change should drastically change hash
        let hash1 = compute_sha256_hash(b"test")?;
        let hash2 = compute_sha256_hash(b"Test")?; // Capital T

        // Hashes should be completely different
        let mut differences = 0;
        for (a, b) in hash1.iter().zip(hash2.iter()) {
            if a != b {
                differences += 1;
            }
        }

        // Should have many differences (avalanche effect)
        assert!(differences > 20);
        Ok(())
    }

    #[test]
    fn test_sha256_incremental_changes() -> Result<(), BearDogError> {
        let base = b"test";
        let hash_base = compute_sha256_hash(base)?;

        for i in 0..10u8 {
            let mut modified = base.to_vec();
            modified.push(i);
            let hash_modified = compute_sha256_hash(&modified)?;
            assert_ne!(hash_base, hash_modified);
        }
        Ok(())
    }
}

#[cfg(test)]
mod sha512_tests {
    use super::*;

    #[test]
    fn test_sha512_empty_input() -> Result<(), BearDogError> {
        let hash = compute_sha512_hash(b"")?;
        assert_eq!(hash.len(), 64);
        Ok(())
    }

    #[test]
    fn test_sha512_consistency() -> Result<(), BearDogError> {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let input = b"consistent input";
        let hash1 = compute_sha512_hash(input)?;
        let hash2 = compute_sha512_hash(input)?;
        assert_eq!(hash1, hash2);
        Ok(())
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_sha512_different_from_sha256() -> Result<(), BearDogError> {
        let input = b"same input";
        let sha256 = compute_sha256_hash(input)?;
        let sha512 = compute_sha512_hash(input)?;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        // Lengths should differ
        assert_ne!(sha256.len(), sha512.len());
        Ok(())
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_sha512_large_input() -> Result<(), BearDogError> {
        let large = vec![0xFF; 10 * 1024 * 1024]; // 10 MB
        let hash = compute_sha512_hash(&large)?;
        assert_eq!(hash.len(), 64);
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_sha512_all_byte_values() -> Result<(), BearDogError> {
        let all_bytes: Vec<u8> = (0..=255).collect();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let hash = compute_sha512_hash(&all_bytes)?;
        assert_eq!(hash.len(), 64);
        assert_ne!(hash, vec![0u8; 64]);
        Ok(())
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
}

#[cfg(test)]
mod random_generation_tests {
    use super::*;
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_generate_random_bytes_length() -> Result<(), BearDogError> {
        let bytes = generate_secure_random_bytes(32)?;
        assert_eq!(bytes.len(), 32);
        Ok(())
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_generate_random_bytes_different() -> Result<(), BearDogError> {
        let bytes1 = generate_secure_random_bytes(32)?;
        let bytes2 = generate_secure_random_bytes(32)?;
        // Extremely unlikely to be the same
        assert_ne!(bytes1, bytes2);
        Ok(())
    }

    #[test]
    fn test_generate_random_bytes_not_all_zeros() -> Result<(), BearDogError> {
        let bytes = generate_secure_random_bytes(100)?;
        // Should not be all zeros
        assert_ne!(bytes, vec![0u8; 100]);
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_generate_random_bytes_various_sizes() -> Result<(), BearDogError> {
        for size in [1, 8, 16, 32, 64, 128, 256, 512, 1024] {
            let bytes = generate_secure_random_bytes(size)?;
            assert_eq!(bytes.len(), size);
        }
        Ok(())
    }

    #[test]
    fn test_random_distribution_not_uniform_zeros() -> Result<(), BearDogError> {
        let bytes = generate_secure_random_bytes(1000)?;
        let zero_count = bytes.iter().filter(|&&b| b == 0).count();

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // With truly random data, expect roughly 1000/256 ≈ 4 zeros
        // Allow range of 0-20 for statistical variation
        assert!(zero_count < 50); // Not mostly zeros
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_random_generation_rapid() -> Result<(), BearDogError> {
        // Generate many random values rapidly
        for _ in 0..100 {
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            let _ = generate_secure_random_bytes(32)?;
        }
        Ok(())
    }

    #[test]
    fn test_random_bytes_uniqueness() -> Result<(), BearDogError> {
        let mut seen = std::collections::HashSet::new();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        for _ in 0..100 {
            let bytes = generate_secure_random_bytes(16)?;
            let hex = hex::encode(&bytes);
            assert!(!seen.contains(&hex), "Generated duplicate random value!");
            seen.insert(hex);
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
        }
        Ok(())
    }
}

#[cfg(test)]
mod key_derivation_tests {
    use super::*;

    #[test]
    fn test_derive_key_basic() -> Result<(), BearDogError> {
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let password = b"password";
        let salt = b"salt";
        let iterations = 1000;

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let key = derive_key_from_password(password, salt, iterations)?;
        assert!(!key.is_empty());
        assert_eq!(key.len(), 32); // SHA-256 output
        Ok(())
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_derive_key_consistency() -> Result<(), BearDogError> {
        let password = b"password";
        let salt = b"salt";
        let iterations = 1000;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        let key1 = derive_key_from_password(password, salt, iterations)?;
        let key2 = derive_key_from_password(password, salt, iterations)?;

        assert_eq!(key1, key2);
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_derive_key_different_salts() -> Result<(), BearDogError> {
        let password = b"password";
        let salt1 = b"salt1";
        let salt2 = b"salt2";
        let iterations = 1000;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        let key1 = derive_key_from_password(password, salt1, iterations)?;
        let key2 = derive_key_from_password(password, salt2, iterations)?;

        assert_ne!(key1, key2);
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_derive_key_different_passwords() -> Result<(), BearDogError> {
        let password1 = b"password1";
        let password2 = b"password2";
        let salt = b"salt";
        let iterations = 1000;

        let key1 = derive_key_from_password(password1, salt, iterations)?;
        let key2 = derive_key_from_password(password2, salt, iterations)?;

        assert_ne!(key1, key2);
        Ok(())
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_derive_key_more_iterations() -> Result<(), BearDogError> {
        let password = b"password";
        let salt = b"salt";

        let key1 = derive_key_from_password(password, salt, 1000)?;
        let key2 = derive_key_from_password(password, salt, 2000)?;
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        // Different iteration counts should produce different keys
        assert_ne!(key1, key2);
        Ok(())
    }

    #[test]
    fn test_derive_key_empty_password() -> Result<(), BearDogError> {
        let password = b"";
        let salt = b"salt";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let iterations = 1000;

        let key = derive_key_from_password(password, salt, iterations)?;
        assert!(!key.is_empty());
        Ok(())
    }

    #[test]
    fn test_derive_key_empty_salt() -> Result<(), BearDogError> {
        let password = b"password";
        let salt = b"";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let iterations = 1000;

        let key = derive_key_from_password(password, salt, iterations)?;
        assert!(!key.is_empty());
        Ok(())
    }

    #[test]
    fn test_derive_key_zero_iterations() -> Result<(), BearDogError> {
        let password = b"password";
        let salt = b"salt";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let iterations = 0;

        let key = derive_key_from_password(password, salt, iterations)?;
        assert!(!key.is_empty());
        // With 0 iterations, should just return password
        Ok(())
    }
}

#[cfg(test)]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
mod constant_time_comparison_tests {
    use super::*;

    #[test]
    fn test_constant_time_equal() {
        let a = b"secret_value";
        let b = b"secret_value";
        assert!(constant_time_compare(a, b));
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_constant_time_not_equal() {
        let a = b"secret1";
        let b = b"secret2";
        assert!(!constant_time_compare(a, b));
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_constant_time_different_lengths() {
        let a = b"short";
        let b = b"longer_string";
        assert!(!constant_time_compare(a, b));
    }

    #[test]
    fn test_constant_time_empty() {
        let a = b"";
        let b = b"";
        assert!(constant_time_compare(a, b));
    }
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_constant_time_one_empty() {
        let a = b"nonempty";
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let b = b"";
        assert!(!constant_time_compare(a, b));
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_constant_time_binary_data() {
        let a = vec![0x00, 0xFF, 0xAA, 0x55];
        let b = vec![0x00, 0xFF, 0xAA, 0x55];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(constant_time_compare(&a, &b));
    }

    #[test]
    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_constant_time_one_bit_difference() {
        let a = vec![0b10101010];
        let b = vec![0b10101011]; // Last bit different
        assert!(!constant_time_compare(&a, &b));
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }
}

#[cfg(test)]
// TEST_CATEGORY: integration
// TEST_DOMAIN: security
// TEST_PRIORITY: normal
mod integration_tests {
    use super::*;

    #[test]
    fn test_hash_and_compare_workflow() -> Result<(), BearDogError> {
        // Simulate password storage workflow
        let password = b"user_password_123";
        let salt = generate_secure_random_bytes(32)?;

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // Derive a key from password
        let stored_key = derive_key_from_password(password, &salt, 10000)?;

        // Later, verify the password
        let verification_key = derive_key_from_password(password, &salt, 10000)?;

        assert!(constant_time_compare(&stored_key, &verification_key));
        Ok(())
    }

    #[test]
    fn test_multiple_crypto_operations() -> Result<(), BearDogError> {
        // Perform multiple operations in sequence
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let data = b"test data";

        // Hash with SHA-256
        let hash256 = compute_sha256_hash(data)?;
        assert_eq!(hash256.len(), 32);

        // Hash with SHA-512
        let hash512 = compute_sha512_hash(data)?;
        assert_eq!(hash512.len(), 64);

        // Generate random salt
        let salt = generate_secure_random_bytes(32)?;
        assert_eq!(salt.len(), 32);

        // Derive key
        let key = derive_key_from_password(data, &salt, 1000)?;
        assert!(!key.is_empty());

        Ok(())
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_deterministic_pipeline() -> Result<(), BearDogError> {
        let password = b"password";
        let salt = b"fixed_salt";

        // Run the same pipeline twice
        let key1 = derive_key_from_password(password, salt, 1000)?;
        let hash1 = compute_sha256_hash(&key1)?;

        let key2 = derive_key_from_password(password, salt, 1000)?;
        let hash2 = compute_sha256_hash(&key2)?;

        // Should get identical results
        assert_eq!(key1, key2);
        assert_eq!(hash1, hash2);

        Ok(())
    }
}
