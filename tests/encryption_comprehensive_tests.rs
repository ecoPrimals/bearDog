// BearDog - Enterprise Security Ecosystem
// Copyright (C) 2025 EcoPrimals
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


//! Comprehensive Encryption Tests for BearDog
//!
//! Tests for encryption engine, key management, and cryptographic operations

use beardog::config::EncryptionConfig;
use beardog::crypto_utils::BearDogCrypto;
use beardog::encryption::*;
use beardog::utils::crypto_utils;
use beardog::*;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::test]
async fn test_encryption_engine_initialization() -> BearDogResult<()> {
    // Test encryption engine initialization with various configurations
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config).await?;

    // Verify engine is properly initialized
    // Note: get_current_algorithm() doesn't exist, so we'll test basic functionality
    let test_data = b"test initialization";
    let encrypted = engine.encrypt(test_data, None).await?;
    assert!(!encrypted.ciphertext.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_basic_encryption_decryption() -> BearDogResult<()> {
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config).await?;

    let test_data = b"BearDog protects the digital forest";

    // Test encryption
    let encrypted = engine.encrypt(test_data, None).await?;
    assert!(
        !encrypted.ciphertext.is_empty(),
        "Encrypted data should not be empty"
    );

    // Test decryption
    let decrypted = engine.decrypt(&encrypted).await?;
    assert_eq!(
        test_data,
        decrypted.as_slice(),
        "Decrypted data should match original"
    );

    Ok(())
}

#[tokio::test]
async fn test_encryption_with_different_algorithms() -> BearDogResult<()> {
    // Test multiple encryption algorithms
    let algorithms = vec![
        Some(EncryptionAlgorithm::Aes256Gcm),
        Some(EncryptionAlgorithm::ChaCha20Poly1305),
    ];

    for algorithm in algorithms {
        let config = EncryptionConfig::default();
        let engine = EncryptionEngine::new(config).await?;

        let test_data = format!("Testing {algorithm:?} algorithm");
        let encrypted = engine.encrypt(test_data.as_bytes(), algorithm).await?;
        let decrypted = engine.decrypt(&encrypted).await?;

        assert_eq!(test_data.as_bytes(), decrypted.as_slice());
        println!("✅ {algorithm:?} encryption/decryption successful");
    }

    Ok(())
}

#[tokio::test]
async fn test_key_derivation() -> BearDogResult<()> {
    // Test key derivation functions
    let password = "strong_password_for_beardog";
    let salt = "unique_salt_for_testing";

    // Test PBKDF2 key derivation using utils
    let derived_key =
        crypto_utils::pbkdf2_hmac_sha256(password.as_bytes(), salt.as_bytes(), 10000, 32);
    assert_eq!(derived_key.len(), 32);

    // Test consistency - same input should produce same output
    let derived_key2 =
        crypto_utils::pbkdf2_hmac_sha256(password.as_bytes(), salt.as_bytes(), 10000, 32);
    assert_eq!(derived_key, derived_key2);

    // Test different passwords produce different keys
    let different_key = crypto_utils::pbkdf2_hmac_sha256(
        "different_password".as_bytes(),
        salt.as_bytes(),
        10000,
        32,
    );
    assert_ne!(derived_key, different_key);

    Ok(())
}

#[test]
fn test_crypto_utils_hash_functions() -> BearDogResult<()> {
    // Test SHA-256 hashing using utils (returns hex string)
    let data = b"BearDog security testing";
    let hash = crypto_utils::sha256_hash(data);
    assert_eq!(hash.len(), 64); // SHA-256 hex encoded = 64 chars

    // Test consistency
    let hash2 = crypto_utils::sha256_hash(data);
    assert_eq!(hash, hash2);

    // Test different data produces different hash
    let different_hash = crypto_utils::sha256_hash(b"different data");
    assert_ne!(hash, different_hash);

    Ok(())
}

#[test]
fn test_crypto_utils_hmac() -> BearDogResult<()> {
    // Test HMAC-SHA256 using utils (returns hex string)
    let key = b"secret_hmac_key";
    let data = b"data to authenticate";

    let hmac = crypto_utils::hmac_sha256(key, data);
    assert_eq!(hmac.len(), 64); // HMAC-SHA256 hex encoded = 64 chars

    // Test consistency
    let hmac2 = crypto_utils::hmac_sha256(key, data);
    assert_eq!(hmac, hmac2);

    // Test different key produces different HMAC
    let different_hmac = crypto_utils::hmac_sha256(b"different_key", data);
    assert_ne!(hmac, different_hmac);

    Ok(())
}

#[test]
fn test_password_generation() -> BearDogResult<()> {
    // Test secure password generation using utils
    let password = crypto_utils::generate_password(16);
    assert_eq!(password.len(), 16);

    // Test that multiple generations produce different passwords
    let password2 = crypto_utils::generate_password(16);
    assert_ne!(password, password2);

    // Test different lengths
    for length in [8, 12, 20, 32] {
        let pwd = crypto_utils::generate_password(length);
        assert_eq!(pwd.len(), length);
    }

    Ok(())
}

#[test]
fn test_hex_encoding() -> BearDogResult<()> {
    // Test hex encoding/decoding using utils
    let data = b"test data for hex encoding";

    let hex_encoded = crypto_utils::bytes_to_hex(data);
    assert_eq!(hex_encoded.len(), data.len() * 2); // Each byte becomes 2 hex chars

    let decoded = crypto_utils::hex_to_bytes(&hex_encoded)?;
    assert_eq!(data, decoded.as_slice());

    // Test invalid hex decoding
    assert!(crypto_utils::hex_to_bytes("invalid_hex_string").is_err());

    Ok(())
}

#[test]
fn test_secure_random_bytes() -> BearDogResult<()> {
    // Test secure random byte generation using utils
    let random1 = crypto_utils::secure_random_bytes(32);
    let random2 = crypto_utils::secure_random_bytes(32);

    assert_eq!(random1.len(), 32);
    assert_eq!(random2.len(), 32);
    assert_ne!(random1, random2); // Should be different

    // Test different sizes
    for size in [8, 16, 32, 64] {
        let random = crypto_utils::secure_random_bytes(size);
        assert_eq!(random.len(), size);
    }

    Ok(())
}

#[tokio::test]
async fn test_encryption_with_large_data() -> BearDogResult<()> {
    // Test encryption with larger data sets
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config).await?;

    // Test with 1MB of data
    let large_data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();

    let encrypted = engine.encrypt(&large_data, None).await?;
    let decrypted = engine.decrypt(&encrypted).await?;

    assert_eq!(large_data, decrypted);
    println!(
        "✅ Large data encryption/decryption successful: {} bytes",
        large_data.len()
    );

    Ok(())
}

#[tokio::test]
async fn test_concurrent_encryption_operations() -> BearDogResult<()> {
    // Test concurrent encryption operations
    let config = EncryptionConfig::default();
    let engine = Arc::new(EncryptionEngine::new(config).await?);

    let test_data = b"concurrent test data";

    // Run multiple encryption operations concurrently
    let mut handles = Vec::new();
    for i in 0..10 {
        let engine_clone = Arc::clone(&engine);
        let data = format!("test data {i}");
        let handle = tokio::spawn(async move {
            let encrypted = engine_clone.encrypt(data.as_bytes(), None).await?;
            let decrypted = engine_clone.decrypt(&encrypted).await?;
            assert_eq!(data.as_bytes(), decrypted.as_slice());
            Ok::<(), BearDogError>(())
        });
        handles.push(handle);
    }

    // Wait for all operations to complete
    for handle in handles {
        handle.await.map_err(|e| {
    tracing::error!("Operation failed: {:?}", e);
    beardog_errors::BearDogError::internal(format!("Operation failed: {:?}", e))
})??;
    }

    Ok(())
}

#[tokio::test]
async fn test_encryption_error_handling() -> BearDogResult<()> {
    // Test error handling in encryption operations
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config).await?;

    // Test decryption with invalid data
    let invalid_encrypted = EncryptedData {
        algorithm: EncryptionAlgorithm::Aes256Gcm,
        ciphertext: vec![0; 16],
        nonce: vec![0; 8], // Invalid nonce length
        tag: None,
        metadata: HashMap::new(),
        key_id: None,
    };

    let result = engine.decrypt(&invalid_encrypted).await;
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_password_hashing() -> BearDogResult<()> {
    // Test password hashing using BearDogCrypto
    let password = "secure_password_123";
    let hash = BearDogCrypto::hash_password_argon2(password)?;

    // Test verification
    let is_valid = BearDogCrypto::verify_password_argon2(password, &hash)?;
    assert!(is_valid);

    // Test with wrong password
    let is_invalid = BearDogCrypto::verify_password_argon2("wrong_password", &hash)?;
    assert!(!is_invalid);

    Ok(())
}

#[tokio::test]
async fn test_key_rotation() -> BearDogResult<()> {
    // Test key rotation functionality
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config).await?;

    // Test key rotation
    let result = engine.rotate_keys().await;
    assert!(result.is_ok());

    // Test that encryption still works after rotation
    let test_data = b"test data after rotation";
    let encrypted = engine.encrypt(test_data, None).await?;
    let decrypted = engine.decrypt(&encrypted).await?;
    assert_eq!(test_data, decrypted.as_slice());

    Ok(())
}

// Helper functions for test compatibility
fn derive_key_argon2(password: &[u8], salt: &[u8], length: usize) -> BearDogResult<Vec<u8>> {
    // Use BearDogCrypto's derive_key_pbkdf2 as a substitute
    BearDogCrypto::derive_key_pbkdf2(password, salt, 10000, length)
}

fn hash_password_argon2(password: &str) -> BearDogResult<String> {
    BearDogCrypto::hash_password_argon2(password)
}

fn verify_password_argon2(password: &str, hash: &str) -> BearDogResult<bool> {
    BearDogCrypto::verify_password_argon2(password, hash)
}
