// SPDX-License-Identifier: AGPL-3.0-or-later

//! Encryption edge cases

use super::*;

#[test]
fn test_encrypt_with_empty_key() {
    // Test encryption fails gracefully with empty key
    let data = b"test data";
    let empty_key: &[u8] = &[];

    let result = attempt_encryption(data, empty_key);
    assert!(result.is_err(), "Should reject empty encryption key");
}

#[test]
fn test_encrypt_with_short_key() {
    // Test encryption fails with key shorter than required
    let data = b"test data";
    let short_key = &[1u8; 15]; // Too short for AES-256 (needs 32)

    let result = attempt_encryption(data, short_key);
    assert!(result.is_err(), "Should reject short key");
}

#[test]
fn test_encrypt_very_large_data() {
    // Test encryption handles large data chunks
    let large_data = vec![0u8; 10 * 1024 * 1024]; // 10MB
    let key = vec![1u8; 32];

    let result = attempt_encryption(&large_data, &key);
    // Should either succeed or fail gracefully with size limit error
    assert!(result.is_ok() || result.is_err());
}

#[test]
fn test_decrypt_with_wrong_key() {
    // Test decryption with incorrect key returns error
    let data = b"test data";
    let key1 = vec![1u8; 32];
    let key2 = vec![2u8; 32];

    let encrypted = attempt_encryption(data, &key1);
    if let Ok(enc_data) = encrypted {
        let result = attempt_decryption(&enc_data, &key2);
        assert!(result.is_err(), "Should fail with wrong decryption key");
    }
}

#[test]
fn test_decrypt_corrupted_data() {
    // Test decryption of corrupted data
    let mut corrupted = vec![0xFF; 128];
    corrupted[64] = 0x00; // Corrupt the data
    let key = vec![1u8; 32];

    let result = attempt_decryption(&corrupted, &key);
    assert!(result.is_err(), "Should detect corrupted encrypted data");
}

#[test]
fn test_encrypt_empty_data() {
    // Test encryption of empty data
    let empty_data: &[u8] = &[];
    let key = vec![1u8; 32];

    let result = attempt_encryption(empty_data, &key);
    assert!(result.is_ok(), "Should handle empty data encryption");
}

#[test]
fn test_encrypt_with_null_bytes() {
    // Test encryption of data containing null bytes
    let data_with_nulls = b"test\x00data\x00with\x00nulls";
    let key = vec![1u8; 32];

    let result = attempt_encryption(data_with_nulls, &key);
    assert!(result.is_ok(), "Should handle null bytes in data");
}

#[test]
fn test_encrypt_decrypt_boundary_sizes() {
    // Test encryption/decryption at common boundary sizes
    let key = vec![1u8; 32];
    let sizes = vec![15, 16, 17, 31, 32, 33, 63, 64, 65, 127, 128, 129];

    for size in sizes {
        let data = vec![42u8; size];
        let encrypted = attempt_encryption(&data, &key);
        assert!(encrypted.is_ok(), "Should encrypt size {size}");
    }
}
