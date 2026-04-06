// SPDX-License-Identifier: AGPL-3.0-or-later

// Comprehensive Security Operations Tests
//
// Extensive test coverage for security primitives and operations

#[cfg(test)]
mod security_operations_tests {
    use crate::*;

    #[test]
    fn test_sha256_hash_empty_input() {
        let data = b"";
        let result = compute_sha256_hash(data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
    }

    #[test]
    fn test_sha256_hash_consistency() {
        let data = b"test data";
        let hash1 = compute_sha256_hash(data).unwrap();
        let hash2 = compute_sha256_hash(data).unwrap();
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_sha256_hash_different_inputs() {
        let data1 = b"test1";
        let data2 = b"test2";
        let hash1 = compute_sha256_hash(data1).unwrap();
        let hash2 = compute_sha256_hash(data2).unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_sha512_hash_empty_input() {
        let data = b"";
        let result = compute_sha512_hash(data);
        assert!(result.is_ok());
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert_eq!(result.unwrap().len(), 64);
    }

    #[test]
    fn test_sha512_hash_consistency() {
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let data = b"test data";
        let hash1 = compute_sha512_hash(data).unwrap();
        let hash2 = compute_sha512_hash(data).unwrap();
        assert_eq!(hash1, hash2);
    }
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal

    #[test]
    fn test_sha512_hash_different_inputs() {
        let data1 = b"test1";
        let data2 = b"test2";
        let hash1 = compute_sha512_hash(data1).unwrap();
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let hash2 = compute_sha512_hash(data2).unwrap();
        assert_ne!(hash1, hash2);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_sha256_hash_large_input() {
        let data = vec![0u8; 1024 * 1024]; // 1 MB
        let result = compute_sha256_hash(&data);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 32);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
    }

    #[test]
    fn test_sha512_hash_large_input() {
        let data = vec![0u8; 1024 * 1024]; // 1 MB
        let result = compute_sha512_hash(&data);
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 64);
    }

    #[test]
    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    fn test_encryption_service_creation() {
        let config = EncryptionConfig {
            algorithm: EncryptionAlgorithm::Aes256Gcm,
            key_size: 32,
        };
        // TEST_CATEGORY: unit
        // TEST_DOMAIN: security
        // TEST_PRIORITY: critical
        let service = EncryptionService::new(config);
        assert!(service.is_initialized());
    }

    #[test]
    fn test_encryption_service_with_different_key_sizes() {
        for key_size in [16, 24, 32] {
            // TEST_CATEGORY: unit
            // TEST_DOMAIN: security
            // TEST_PRIORITY: critical
            let config = EncryptionConfig {
                algorithm: EncryptionAlgorithm::Aes256Gcm,
                key_size,
            };
            let service = EncryptionService::new(config);
            assert!(service.is_initialized());
        }
    }

    // TEST_CATEGORY: unit
    // TEST_DOMAIN: security
    // TEST_PRIORITY: critical
    #[test]
    fn test_encryption_service_chacha20() {
        let config = EncryptionConfig {
            algorithm: EncryptionAlgorithm::ChaCha20Poly1305,
            key_size: 32,
        };
        let service = EncryptionService::new(config);
        assert!(service.is_initialized());
    }
}
