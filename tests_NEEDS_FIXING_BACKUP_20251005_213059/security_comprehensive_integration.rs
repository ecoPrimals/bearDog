use beardog_core::security::*;
use beardog_errors::BearDogError;
use beardog_security::crypto_utils::*;
use beardog_security::encryption::*;
use beardog_security::memory_key_manager::*;
use beardog_types::canonical::security::*;
use std::collections::HashMap;
use tracing::info;

#[tokio::test]
async fn test_crypto_utilities_basic() -> Result<(), BearDogError> {
    info!("🔐 Testing basic crypto utilities");

    // Test that crypto utilities are accessible and functional
    let test_data = "btest encryption data";
    let key = generate_secure_key(32)?;

    // Basic encryption/decryption test
    let encrypted = encrypt_data_aes256(test_data, &key)?;
    assert_ne!(
        encrypted, test_data,
        "Encrypted data should differ from original"
    );

    let decrypted = decrypt_data_aes256(&encrypted, &key)?;
    assert_eq!(decrypted, test_data, "Decrypted data should match original");

    Ok(())
}

#[tokio::test]
async fn test_encryption_functionality() -> Result<(), BearDogError> {
    info!("🔒 Testing encryption functionality");

    let plaintext = "sensitive data for encryption testing";
    let key_material = generate_secure_key(32)?;

    // Test ChaCha20-Poly1305 encryption
    let encrypted = encrypt_chacha20_poly1305(plaintext.as_bytes(), &key_material)?;
    assert!(!encrypted.is_empty(), "Encrypted data should not be empty");

    let decrypted = decrypt_chacha20_poly1305(&encrypted, &key_material)?;
    assert_eq!(
        String::from_utf8(decrypted)?,
        plaintext,
        "Decrypted text should match original"
    );

    Ok(())
}

#[tokio::test]
async fn test_memory_key_management() -> Result<(), BearDogError> {
    info!("🗝️ Testing memory key management");

    let mut key_manager = MemoryKeyManager::new();
    let key_id = "test_key_001";

    // Test key generation and storage
    let key_data = generate_secure_key(32)?;
    key_manager.store_key(key_id, key_data.clone())?;

    // Test key retrieval
    let retrieved_key = key_manager.get_key(key_id)?;
    assert_eq!(
        retrieved_key, key_data,
        "Retrieved key should match stored key"
    );

    // Test key deletion
    key_manager.delete_key(key_id)?;
    assert!(
        key_manager.get_key(key_id).is_err(),
        "Key should be deleted"
    );

    Ok(())
}

#[tokio::test]
async fn test_security_error_handling() -> Result<(), BearDogError> {
    info!("⚠️ Testing security error handling");

    // Test invalid key size error
    let result = generate_secure_key(0);
    assert!(result.is_err(), "Should error on invalid key size");

    // Test decryption with wrong key
    let data = "btest data";
    let key1 = generate_secure_key(32)?;
    let key2 = generate_secure_key(32)?;

    let encrypted = encrypt_data_aes256(data, &key1)?;
    let result = decrypt_data_aes256(&encrypted, &key2);
    assert!(result.is_err(), "Should error on wrong decryption key");

    Ok(())
}

#[tokio::test]
async fn test_cryptographic_operations() -> Result<(), BearDogError> {
    info!("🧮 Testing cryptographic operations");

    let message = "test message for cryptographic operations";

    // Test BLAKE3 hashing
    let hash1 = hash_blake3(message.as_bytes())?;
    let hash2 = hash_blake3(message.as_bytes())?;
    assert_eq!(hash1, hash2, "Same input should produce same hash");

    // Test different input produces different hash
    let different_hash = hash_blake3("bdifferent message")?;
    assert_ne!(
        hash1, different_hash,
        "Different inputs should produce different hashes"
    );

    Ok(())
}

#[tokio::test]
async fn test_key_derivation() -> Result<(), BearDogError> {
    info!("🔑 Testing key derivation");

    let password = "test_password_123";
    let salt = generate_secure_salt()?;

    // Test PBKDF2 key derivation
    let derived_key1 = derive_key_pbkdf2(password.as_bytes(), &salt, 10000, 32)?;
    let derived_key2 = derive_key_pbkdf2(password.as_bytes(), &salt, 10000, 32)?;

    assert_eq!(
        derived_key1, derived_key2,
        "Same inputs should produce same derived key"
    );
    assert_eq!(derived_key1.len(), 32, "Derived key should be 32 bytes");

    // Test different salt produces different key
    let different_salt = generate_secure_salt()?;
    let different_key = derive_key_pbkdf2(password.as_bytes(), &different_salt, 10000, 32)?;
    assert_ne!(
        derived_key1, different_key,
        "Different salt should produce different key"
    );

    Ok(())
}

#[tokio::test]
async fn test_signature_operations() -> Result<(), BearDogError> {
    info!("✍️ Testing signature operations");

    let message = "message to be signed";

    // Test Ed25519 signature operations
    let (public_key, private_key) = generate_ed25519_keypair()?;

    let signature = sign_ed25519(message.as_bytes(), &private_key)?;
    assert!(!signature.is_empty(), "Signature should not be empty");

    let is_valid = verify_ed25519_signature(message.as_bytes(), &signature, &public_key)?;
    assert!(is_valid, "Valid signature should verify successfully");

    // Test invalid signature
    let invalid_signature = vec![0u8; signature.len()];
    let is_invalid = verify_ed25519_signature(message.as_bytes(), &invalid_signature, &public_key)?;
    assert!(!is_invalid, "Invalid signature should not verify");

    Ok(())
}

#[tokio::test]
async fn test_hash_operations() -> Result<(), BearDogError> {
    info!("# Testing hash operations");

    let test_data = "bcomprehensive hash testing data";

    // Test SHA-256
    let sha256_hash = hash_sha256(test_data)?;
    assert_eq!(sha256_hash.len(), 32, "SHA-256 hash should be 32 bytes");

    // Test SHA-3
    let sha3_hash = hash_sha3_256(test_data)?;
    assert_eq!(sha3_hash.len(), 32, "SHA-3 hash should be 32 bytes");

    // Test BLAKE3
    let blake3_hash = hash_blake3(test_data)?;
    assert_eq!(blake3_hash.len(), 32, "BLAKE3 hash should be 32 bytes");

    // Verify hashes are different algorithms
    assert_ne!(
        sha256_hash, sha3_hash,
        "SHA-256 and SHA-3 should produce different results"
    );
    assert_ne!(
        sha256_hash, blake3_hash,
        "SHA-256 and BLAKE3 should produce different results"
    );
    assert_ne!(
        sha3_hash, blake3_hash,
        "SHA-3 and BLAKE3 should produce different results"
    );

    Ok(())
}
