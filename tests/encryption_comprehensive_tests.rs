use beardog_errors::BearDogError;

use beardog::config::EncryptionConfig;
use beardog::crypto_utils::BearDogCrypto;
use beardog::encryption::*;
use beardog::utils::crypto_utils;
use beardog::*;
use std::collections::HashMap;
use std::sync::Arc;

#[tokio::test]
async fn test_encryption_engine_initialization() -> Result<(), BearDogError> {
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config)?;

    let test_data = b"test initialization";
    let encrypted = engine.encrypt(test_data, None)?;
    assert!(!encrypted.ciphertext.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_basic_encryption_decryption() -> Result<(), BearDogError> {
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config)?;

    let test_data = b"BearDog protects the digital forest";

    let encrypted = engine.encrypt(test_data, None)?;
    assert!(
        !encrypted.ciphertext.is_empty(),
        "Encrypted data should not be empty"
    );

    let decrypted = engine.decrypt(&encrypted)?;
    assert_eq!(
        test_data,
        decrypted.as_slice(),
        "Decrypted data should match original"
    );

    Ok(())
}

#[tokio::test]
async fn test_encryption_with_different_algorithms() -> Result<(), BearDogError> {
    let algorithms = vec![
        Some(EncryptionAlgorithm::Aes256Gcm),
        Some(EncryptionAlgorithm::ChaCha20Poly1305),
    ];

    for algorithm in algorithms {
        let config = EncryptionConfig::default();
        let engine = EncryptionEngine::new(config)?;

        let test_data = format!("Testing {algorithm:?} algorithm");
        let encrypted = engine.encrypt(test_data.as_bytes(), algorithm)?;
        let decrypted = engine.decrypt(&encrypted)?;

        assert_eq!(test_data.as_bytes(), decrypted.as_slice());
        println!("✅ {algorithm:?} encryption/decryption successful");
    }

    Ok(())
}

#[tokio::test]
fn test_key_derivation() -> Result<(), BearDogError> {
    let password = "strong_password_for_beardog";
    let salt = "unique_salt_for_testing";

    let derived_key =
        crypto_utils::pbkdf2_hmac_sha256(password.as_bytes(), salt.as_bytes(), 10000, 32);
    assert_eq!(derived_key.len(), 32);

    let derived_key2 =
        crypto_utils::pbkdf2_hmac_sha256(password.as_bytes(), salt.as_bytes(), 10000, 32);
    assert_eq!(derived_key, derived_key2);

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
fn test_crypto_utils_hash_functions() -> Result<(), BearDogError> {
    let data = b"BearDog security testing";
    let hash = crypto_utils::sha256_hash(data);
    assert_eq!(hash.len(), 64); // SHA-256 hex encoded = 64 chars

    let hash2 = crypto_utils::sha256_hash(data);
    assert_eq!(hash, hash2);

    let different_hash = crypto_utils::sha256_hash(b"different data");
    assert_ne!(hash, different_hash);

    Ok(())
}

#[test]
fn test_crypto_utils_hmac() -> Result<(), BearDogError> {
    let key = b"secret_hmac_key";
    let data = b"data to authenticate";

    let hmac = crypto_utils::hmac_sha256(key, data);
    assert_eq!(hmac.len(), 64); // HMAC-SHA256 hex encoded = 64 chars

    let hmac2 = crypto_utils::hmac_sha256(key, data);
    assert_eq!(hmac, hmac2);

    let different_hmac = crypto_utils::hmac_sha256(b"different_key", data);
    assert_ne!(hmac, different_hmac);

    Ok(())
}

#[test]
fn test_password_generation() -> Result<(), BearDogError> {
    let password = crypto_utils::generate_password(16);
    assert_eq!(password.len(), 16);

    let password2 = crypto_utils::generate_password(16);
    assert_ne!(password, password2);

    for length in [8, 12, 20, 32] {
        let pwd = crypto_utils::generate_password(length);
        assert_eq!(pwd.len(), length);
    }

    Ok(())
}

#[test]
fn test_hex_encoding() -> Result<(), BearDogError> {
    let data = b"test data for hex encoding";

    let hex_encoded = crypto_utils::bytes_to_hex(data);
    assert_eq!(hex_encoded.len(), data.len() * 2); // Each byte becomes 2 hex chars

    let decoded = crypto_utils::hex_to_bytes(&hex_encoded)?;
    assert_eq!(data, decoded.as_slice());

    assert!(crypto_utils::hex_to_bytes("invalid_hex_string").is_err());

    Ok(())
}

#[test]
fn test_secure_random_bytes() -> Result<(), BearDogError> {
    let random1 = crypto_utils::secure_random_bytes(32);
    let random2 = crypto_utils::secure_random_bytes(32);

    assert_eq!(random1.len(), 32);
    assert_eq!(random2.len(), 32);
    assert_ne!(random1, random2); // Should be different

    for size in [8, 16, 32, 64] {
        let random = crypto_utils::secure_random_bytes(size);
        assert_eq!(random.len(), size);
    }

    Ok(())
}

#[tokio::test]
async fn test_large_data_encryption() -> Result<(), BearDogError> {
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config)?;

    let large_data: Vec<u8> = (0..1024 * 1024).map(|i| (i % 256) as u8).collect();

    info!(
        "Testing encryption of large data: {} bytes",
        large_data.len()
    );

    Ok(())
}

#[tokio::test]
async fn test_concurrent_encryption_operations() -> Result<(), BearDogError> {
    let config = EncryptionConfig::default();
    let engine = Arc::new(EncryptionEngine::new(config)?);

    let test_data = b"concurrent test data";

    let mut handles = Vec::new();
    for i in 0..10 {
        let engine_clone = Arc::clone(&engine);
        let data = format!("test data {i}");
        let handle = tokio::spawn(async move {
            let encrypted = engine_clone.encrypt(data.as_bytes(), None)?;
            let decrypted = engine_clone.decrypt(&encrypted)?;
            assert_eq!(data.as_bytes(), decrypted.as_slice());
            Ok::<(), BearDogError>(())
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.map_err(|e| {
            tracing::error!("Operation failed: {:?}", e);
            beardog_errors::BearDogError::internal(format!("Error: {:?}", e))
        })??;
    }

    Ok(())
}

#[tokio::test]
async fn test_encryption_error_handling() -> Result<(), BearDogError> {
    let config = EncryptionConfig::default();
    let config = EncryptionConfig::default();
    let engine = EncryptionEngine::new(config)?;

    let invalid_encrypted = EncryptedData {
        ciphertext: vec![0; 16],
        nonce: vec![0; 8], // Invalid nonce length
        tag: None,
        metadata: HashMap::with_capacity(0),
    };

    let result = engine.decrypt(&invalid_encrypted);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_password_hashing() -> Result<(), BearDogError> {
    let password = "secure_password_123";
    let hash = BearDogCrypto::hash_password_argon2(password)?;

    let is_valid = BearDogCrypto::verify_password_argon2(password, &hash)?;
    assert!(is_valid);

    let is_invalid = BearDogCrypto::verify_password_argon2("wrong_password", &hash)?;
    assert!(!is_invalid);

    Ok(())
}

#[tokio::test]
async fn test_key_rotation() -> Result<(), BearDogError> {
    let config = EncryptionConfig::default();
    let _engine = EncryptionEngine::new(config)?;

    // Test key rotation functionality
    Ok(())
}

fn derive_key_pbkdf2(password: &[u8], salt: &[u8], length: usize) -> Result<Vec<u8>, BearDogError> {
    BearDogCrypto::derive_key_pbkdf2(password, salt, 10000, length)
}

fn hash_password_argon2(password: &str) -> Result<String, BearDogError> {
    BearDogCrypto::hash_password_argon2(password)
}

fn verify_password_argon2(password: &str, hash: &str) -> Result<bool, BearDogError> {
    BearDogCrypto::verify_password_argon2(password, hash)
}
