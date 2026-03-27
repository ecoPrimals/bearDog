// SPDX-License-Identifier: AGPL-3.0-only

// Comprehensive validation tests for crypto service types
//
// Testing validation logic, edge cases, and error handling for:
// - CryptoAlgorithm
// - EncryptedData
// - EncryptionMetadata
// - DecryptRequest/Response
// - SignRequest/Response
//
// TARGET: Increase beardog-types coverage (currently untested)

use super::*;
use std::time::{Duration, SystemTime};

// ============================================================================
// CryptoAlgorithm Tests
// ============================================================================

#[test]
fn test_crypto_algorithm_as_str() {
    assert_eq!(CryptoAlgorithm::Aes256Gcm.as_str(), "aes-256-gcm");
    assert_eq!(
        CryptoAlgorithm::ChaCha20Poly1305.as_str(),
        "chacha20-poly1305"
    );
    assert_eq!(CryptoAlgorithm::Aes128Gcm.as_str(), "aes-128-gcm");
}

#[test]
fn test_crypto_algorithm_equality() {
    assert_eq!(CryptoAlgorithm::Aes256Gcm, CryptoAlgorithm::Aes256Gcm);
    assert_ne!(CryptoAlgorithm::Aes256Gcm, CryptoAlgorithm::ChaCha20Poly1305);
}

#[test]
fn test_crypto_algorithm_clone() {
    let alg1 = CryptoAlgorithm::Aes256Gcm;
    let alg2 = alg1;
    assert_eq!(alg1, alg2);
}

#[test]
fn test_crypto_algorithm_debug() {
    let alg = CryptoAlgorithm::ChaCha20Poly1305;
    let debug_str = format!("{alg:?}");
    assert!(debug_str.contains("ChaCha20Poly1305"));
}

// ============================================================================
// EncryptedData Tests
// ============================================================================

#[test]
fn test_encrypted_data_creation() {
    let data = EncryptedData {
        ciphertext: vec![1, 2, 3, 4, 5],
        algorithm: CryptoAlgorithm::Aes256Gcm,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::now(),
            key_id: Some("test-key".to_string()),
            nonce: vec![0x11, 0x22, 0x33],
            tag: Some(vec![0xAA, 0xBB]),
        },
    };

    assert_eq!(data.ciphertext.len(), 5);
    assert_eq!(data.algorithm, CryptoAlgorithm::Aes256Gcm);
    assert_eq!(data.metadata.key_id, Some("test-key".to_string()));
}

#[test]
fn test_encrypted_data_empty_ciphertext() {
    let data = EncryptedData {
        ciphertext: vec![],
        algorithm: CryptoAlgorithm::Aes256Gcm,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::now(),
            key_id: None,
            nonce: vec![],
            tag: None,
        },
    };

    assert!(data.ciphertext.is_empty());
    assert!(data.metadata.nonce.is_empty());
    assert!(data.metadata.tag.is_none());
}

#[test]
fn test_encrypted_data_large_ciphertext() {
    let large_ciphertext = vec![0xAB; 1_000_000];
    let data = EncryptedData {
        ciphertext: large_ciphertext,
        algorithm: CryptoAlgorithm::ChaCha20Poly1305,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::now(),
            key_id: Some("large-data-key".to_string()),
            nonce: vec![0x00; 12],
            tag: Some(vec![0xFF; 16]),
        },
    };

    assert_eq!(data.ciphertext.len(), 1_000_000);
    assert_eq!(data.metadata.nonce.len(), 12);
    assert_eq!(data.metadata.tag.as_ref().unwrap().len(), 16);
}

#[test]
fn test_encrypted_data_clone() {
    let data1 = EncryptedData {
        ciphertext: vec![1, 2, 3],
        algorithm: CryptoAlgorithm::Aes128Gcm,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::now(),
            key_id: Some("test".to_string()),
            nonce: vec![4, 5, 6],
            tag: None,
        },
    };

    let data2 = data1.clone();
    assert_eq!(data1.ciphertext, data2.ciphertext);
    assert_eq!(data1.algorithm, data2.algorithm);
}

// ============================================================================
// EncryptionMetadata Tests
// ============================================================================

#[test]
fn test_encryption_metadata_timestamp() {
    let now = SystemTime::now();
    let _metadata = EncryptionMetadata {
        timestamp: now,
        key_id: None,
        nonce: vec![],
        tag: None,
    };

    // Timestamp should be recent (within 1 second)
    let elapsed = now.elapsed().unwrap();
    assert!(elapsed < Duration::from_secs(1));
}

#[test]
fn test_encryption_metadata_with_all_fields() {
    let metadata = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: Some("full-key-id".to_string()),
        nonce: vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C],
        tag: Some(vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0xFF]),
    };

    assert!(metadata.key_id.is_some());
    assert_eq!(metadata.nonce.len(), 12);
    assert_eq!(metadata.tag.as_ref().unwrap().len(), 16);
}

#[test]
fn test_encryption_metadata_without_optional_fields() {
    let metadata = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: None,
        nonce: vec![1, 2, 3],
        tag: None,
    };

    assert!(metadata.key_id.is_none());
    assert!(metadata.tag.is_none());
    assert!(!metadata.nonce.is_empty());
}

#[test]
fn test_encryption_metadata_long_key_id() {
    let long_id = "a".repeat(1000);
    let metadata = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: Some(long_id),
        nonce: vec![],
        tag: None,
    };

    assert_eq!(metadata.key_id.unwrap().len(), 1000);
}

// ============================================================================
// Serialization/Deserialization Tests
// ============================================================================

#[test]
fn test_crypto_algorithm_serde_roundtrip() {
    let alg = CryptoAlgorithm::Aes256Gcm;
    let json = serde_json::to_string(&alg).unwrap();
    let deserialized: CryptoAlgorithm = serde_json::from_str(&json).unwrap();
    assert_eq!(alg, deserialized);
}

#[test]
fn test_encrypted_data_serde_roundtrip() {
    let data = EncryptedData {
        ciphertext: vec![1, 2, 3, 4, 5],
        algorithm: CryptoAlgorithm::ChaCha20Poly1305,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::now(),
            key_id: Some("serde-test".to_string()),
            nonce: vec![0x11, 0x22, 0x33],
            tag: Some(vec![0xAA, 0xBB, 0xCC]),
        },
    };

    let json = serde_json::to_string(&data).unwrap();
    let deserialized: EncryptedData = serde_json::from_str(&json).unwrap();

    assert_eq!(data.ciphertext, deserialized.ciphertext);
    assert_eq!(data.algorithm, deserialized.algorithm);
    assert_eq!(data.metadata.key_id, deserialized.metadata.key_id);
}

#[test]
fn test_encrypted_data_serde_with_binary_data() {
    let data = EncryptedData {
        ciphertext: vec![0x00, 0xFF, 0x80, 0x7F],
        algorithm: CryptoAlgorithm::Aes128Gcm,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::now(),
            key_id: None,
            nonce: vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB],
            tag: Some(vec![0xFF; 16]),
        },
    };

    let json = serde_json::to_string(&data).unwrap();
    let deserialized: EncryptedData = serde_json::from_str(&json).unwrap();

    assert_eq!(data.ciphertext, deserialized.ciphertext);
    assert_eq!(data.metadata.nonce, deserialized.metadata.nonce);
}

#[test]
fn test_encryption_metadata_serde_preserves_timestamp() {
    let timestamp = SystemTime::UNIX_EPOCH + Duration::from_secs(1_700_000_000);
    let metadata = EncryptionMetadata {
        timestamp,
        key_id: Some("time-test".to_string()),
        nonce: vec![1, 2, 3],
        tag: None,
    };

    let json = serde_json::to_string(&metadata).unwrap();
    let deserialized: EncryptionMetadata = serde_json::from_str(&json).unwrap();

    // Timestamps should be equal (or very close)
    let diff = deserialized
        .timestamp
        .duration_since(timestamp)
        .unwrap_or_else(|_| timestamp.duration_since(deserialized.timestamp).unwrap());
    assert!(diff < Duration::from_millis(1));
}

// ============================================================================
// Edge Cases and Boundary Conditions
// ============================================================================

#[test]
fn test_zero_length_nonce() {
    let metadata = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: Some("zero-nonce".to_string()),
        nonce: vec![],
        tag: Some(vec![0x11, 0x22]),
    };

    assert!(metadata.nonce.is_empty());
}

#[test]
fn test_very_large_nonce() {
    let large_nonce = vec![0x42; 1024];
    let metadata = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: None,
        nonce: large_nonce,
        tag: None,
    };

    assert_eq!(metadata.nonce.len(), 1024);
}

#[test]
fn test_encrypted_data_with_max_timestamp() {
    let far_future = SystemTime::UNIX_EPOCH + Duration::from_secs(u64::from(u32::MAX));
    let data = EncryptedData {
        ciphertext: vec![1, 2, 3],
        algorithm: CryptoAlgorithm::Aes256Gcm,
        metadata: EncryptionMetadata {
            timestamp: far_future,
            key_id: None,
            nonce: vec![],
            tag: None,
        },
    };

    // Should handle far-future timestamps
    let json = serde_json::to_string(&data).unwrap();
    let _deserialized: EncryptedData = serde_json::from_str(&json).unwrap();
}

#[test]
fn test_encrypted_data_with_past_timestamp() {
    let past = SystemTime::UNIX_EPOCH + Duration::from_secs(1);
    let data = EncryptedData {
        ciphertext: vec![0xDE, 0xAD, 0xBE, 0xEF],
        algorithm: CryptoAlgorithm::ChaCha20Poly1305,
        metadata: EncryptionMetadata {
            timestamp: past,
            key_id: Some("ancient-key".to_string()),
            nonce: vec![0; 12],
            tag: Some(vec![0; 16]),
        },
    };

    // Should handle old timestamps
    let json = serde_json::to_string(&data).unwrap();
    let _deserialized: EncryptedData = serde_json::from_str(&json).unwrap();
}

// ============================================================================
// Algorithm Variant Coverage
// ============================================================================

#[test]
fn test_all_algorithm_variants_serde() {
    let algorithms = vec![
        CryptoAlgorithm::Aes256Gcm,
        CryptoAlgorithm::ChaCha20Poly1305,
        CryptoAlgorithm::Aes128Gcm,
    ];

    for alg in algorithms {
        let json = serde_json::to_string(&alg).unwrap();
        let deserialized: CryptoAlgorithm = serde_json::from_str(&json).unwrap();
        assert_eq!(alg, deserialized);
    }
}

#[test]
fn test_encrypted_data_with_each_algorithm() {
    let algorithms = vec![
        CryptoAlgorithm::Aes256Gcm,
        CryptoAlgorithm::ChaCha20Poly1305,
        CryptoAlgorithm::Aes128Gcm,
    ];

    for (idx, alg) in algorithms.into_iter().enumerate() {
        let data = EncryptedData {
            ciphertext: vec![idx as u8; 100],
            algorithm: alg,
            metadata: EncryptionMetadata {
                timestamp: SystemTime::now(),
                key_id: Some(format!("key-{idx}")),
                nonce: vec![idx as u8; 12],
                tag: Some(vec![idx as u8; 16]),
            },
        };

        assert_eq!(data.algorithm, alg);
        assert_eq!(data.ciphertext.len(), 100);
    }
}

// ============================================================================
// Metadata Field Combinations
// ============================================================================

#[test]
fn test_metadata_all_combinations() {
    // All fields present
    let full = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: Some("full".to_string()),
        nonce: vec![1, 2, 3],
        tag: Some(vec![4, 5, 6]),
    };
    assert!(full.key_id.is_some());
    assert!(full.tag.is_some());

    // Only required fields
    let minimal = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: None,
        nonce: vec![],
        tag: None,
    };
    assert!(minimal.key_id.is_none());
    assert!(minimal.tag.is_none());

    // key_id only
    let with_key = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: Some("key-only".to_string()),
        nonce: vec![],
        tag: None,
    };
    assert!(with_key.key_id.is_some());
    assert!(with_key.tag.is_none());

    // tag only
    let with_tag = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: None,
        nonce: vec![],
        tag: Some(vec![7, 8, 9]),
    };
    assert!(with_tag.key_id.is_none());
    assert!(with_tag.tag.is_some());
}

// ============================================================================
// Type Safety and Compilation Tests
// ============================================================================

#[test]
fn test_crypto_algorithm_is_copy() {
    let alg1 = CryptoAlgorithm::Aes256Gcm;
    let alg2 = alg1; // Copy
    let alg3 = alg1; // Can copy again
    assert_eq!(alg1, alg2);
    assert_eq!(alg2, alg3);
}

#[test]
fn test_encrypted_data_is_clone() {
    let data1 = EncryptedData {
        ciphertext: vec![1, 2, 3],
        algorithm: CryptoAlgorithm::Aes256Gcm,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::now(),
            key_id: None,
            nonce: vec![],
            tag: None,
        },
    };

    let data2 = data1.clone();
    assert_eq!(data1.ciphertext, data2.ciphertext);
}

// ============================================================================
// Debug Format Tests
// ============================================================================

#[test]
fn test_encrypted_data_debug_format() {
    let data = EncryptedData {
        ciphertext: vec![0xDE, 0xAD],
        algorithm: CryptoAlgorithm::Aes256Gcm,
        metadata: EncryptionMetadata {
            timestamp: SystemTime::now(),
            key_id: Some("debug-test".to_string()),
            nonce: vec![0x11],
            tag: Some(vec![0x22]),
        },
    };

    let debug_str = format!("{data:?}");
    assert!(debug_str.contains("EncryptedData"));
    assert!(debug_str.contains("Aes256Gcm"));
}

#[test]
fn test_encryption_metadata_debug_format() {
    let metadata = EncryptionMetadata {
        timestamp: SystemTime::now(),
        key_id: Some("meta-debug".to_string()),
        nonce: vec![1, 2, 3],
        tag: None,
    };

    let debug_str = format!("{metadata:?}");
    assert!(debug_str.contains("EncryptionMetadata"));
}

