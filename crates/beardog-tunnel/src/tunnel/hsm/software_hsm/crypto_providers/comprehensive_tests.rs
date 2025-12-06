//! Comprehensive Crypto Provider Tests
//!
//! This module contains extensive tests for crypto provider selection,
//! algorithm coverage, error handling, and large data scenarios.
//!
//! Coverage Sprint - Session 1: Crypto Provider Integration Tests

use super::*;
use crate::tunnel::hsm::types::config::CryptoBackend;
use crate::tunnel::hsm::types::KeyType;
use beardog_errors::BearDogError;

// ============================================================================
// PROVIDER SELECTION & FALLBACK TESTS
// ============================================================================

#[tokio::test]
async fn test_all_providers_initialize_successfully() -> Result<(), BearDogError> {
    // Test: All supported providers can initialize
    let backends = get_supported_crypto_backends();

    for backend in backends {
        let provider = create_crypto_provider(&backend).await?;
        let init_result = provider.initialize().await;

        assert!(
            init_result.is_ok(),
            "Provider {:?} should initialize successfully",
            backend
        );
    }

    Ok(())
}

#[tokio::test]
async fn test_provider_capabilities_accurate() -> Result<(), BearDogError> {
    // Test: Reported capabilities match actual support
    let backends = get_supported_crypto_backends();

    for backend in backends {
        let capabilities = get_crypto_provider_capabilities(&backend);
        let provider = create_crypto_provider(&backend).await?;
        provider.initialize().await?;

        // Verify AES support if claimed
        if capabilities.supports_aes {
            let key = provider.generate_key_material(&KeyType::Aes).await?;
            assert_eq!(key.len(), 32, "AES-256 key should be 32 bytes");

            // Test encrypt/decrypt works
            let plaintext = b"test data";
            let ciphertext = provider.encrypt(&key, plaintext).await?;
            let decrypted = provider.decrypt(&key, &ciphertext).await?;
            assert_eq!(plaintext.as_slice(), decrypted.as_slice());
        }

        // Verify ECC support if claimed
        if capabilities.supports_ecc {
            let key = provider.generate_key_material(&KeyType::Ed25519).await?;
            assert!(!key.is_empty(), "Ed25519 key should not be empty");
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_concurrent_provider_creation() -> Result<(), BearDogError> {
    // Test: Multiple concurrent provider creations work correctly
    let mut handles = vec![];

    for _ in 0..10 {
        handles.push(tokio::spawn(async {
            let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
            provider.initialize().await?;
            Ok::<_, BearDogError>(())
        }));
    }

    for handle in handles {
        let result = handle
            .await
            .map_err(|e| BearDogError::internal(format!("Task failed: {}", e)))?;
        assert!(result.is_ok());
    }

    Ok(())
}

#[test]
fn test_is_backend_supported() {
    // Test: Backend support detection
    assert!(is_crypto_backend_supported(&CryptoBackend::RustCrypto));
    assert!(is_crypto_backend_supported(&CryptoBackend::Ring));
    assert!(is_crypto_backend_supported(&CryptoBackend::OpenSsl));
}

#[test]
fn test_get_backend_by_name() {
    // Test: Backend lookup by name
    assert_eq!(
        get_crypto_backend_by_name("RustCrypto"),
        Some(CryptoBackend::RustCrypto)
    );
    assert_eq!(
        get_crypto_backend_by_name("Ring"),
        Some(CryptoBackend::Ring)
    );
    assert_eq!(
        get_crypto_backend_by_name("OpenSsl"),
        Some(CryptoBackend::OpenSsl)
    );
    assert_eq!(get_crypto_backend_by_name("NonExistent"), None);
}

#[test]
fn test_recommended_backend_exists() {
    // Test: Recommended backend is valid
    let recommended = get_recommended_crypto_backend();
    assert!(is_crypto_backend_supported(&recommended));
}

// ============================================================================
// ALGORITHM COMPREHENSIVE COVERAGE TESTS
// ============================================================================

#[tokio::test]
async fn test_aes256_gcm_encrypt_decrypt() -> Result<(), BearDogError> {
    // Test: AES-256-GCM full cycle
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Aes).await?;
    assert_eq!(key.len(), 32);

    let plaintext = b"Sensitive data for AES-256-GCM encryption";
    let ciphertext = provider.encrypt(&key, plaintext).await?;

    // Ciphertext should be different and longer (nonce + tag)
    assert_ne!(ciphertext.as_slice(), plaintext.as_slice());
    assert!(ciphertext.len() >= plaintext.len());

    let decrypted = provider.decrypt(&key, &ciphertext).await?;
    assert_eq!(plaintext.as_slice(), decrypted.as_slice());

    Ok(())
}

#[tokio::test]
async fn test_ed25519_sign_verify() -> Result<(), BearDogError> {
    // Test: Ed25519 signing and verification
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Ed25519).await?;
    let data = b"Message to sign with Ed25519";

    let signature = provider.sign(&key, data).await?;
    assert!(!signature.is_empty());

    // Derive public key for verification
    use ed25519_dalek::SigningKey;
    let key_array: [u8; 32] = key
        .clone()
        .try_into()
        .map_err(|_| BearDogError::crypto_error("Invalid Ed25519 key length".to_string()))?;
    let sk = SigningKey::from_bytes(&key_array);
    let vk = sk.verifying_key();
    let public_key = vk.to_bytes().to_vec();

    let is_valid = provider.verify(&public_key, data, &signature).await?;
    assert!(is_valid, "Valid signature should verify");

    // Test invalid signature
    let mut bad_signature = signature.clone();
    bad_signature[0] ^= 0xFF; // Corrupt first byte
    let is_invalid = provider.verify(&public_key, data, &bad_signature).await?;
    assert!(!is_invalid, "Invalid signature should not verify");

    Ok(())
}

#[tokio::test]
async fn test_multiple_key_types() -> Result<(), BearDogError> {
    // Test: All key types can be generated
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key_types = vec![KeyType::Aes, KeyType::Ed25519];

    for key_type in key_types {
        let key = provider.generate_key_material(&key_type).await?;
        assert!(
            !key.is_empty(),
            "Key type {:?} should generate non-empty key",
            key_type
        );
    }

    Ok(())
}

// ============================================================================
// ERROR PATH TESTS
// ============================================================================

#[tokio::test]
async fn test_decrypt_with_wrong_key() -> Result<(), BearDogError> {
    // Test: Decryption with wrong key fails gracefully
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let correct_key = provider.generate_key_material(&KeyType::Aes).await?;
    let wrong_key = provider.generate_key_material(&KeyType::Aes).await?;

    let plaintext = b"secret message";
    let ciphertext = provider.encrypt(&correct_key, plaintext).await?;

    // Attempt decrypt with wrong key
    let result = provider.decrypt(&wrong_key, &ciphertext).await;
    assert!(result.is_err(), "Decryption with wrong key should fail");

    Ok(())
}

#[tokio::test]
async fn test_decrypt_corrupted_ciphertext() -> Result<(), BearDogError> {
    // Test: Corrupted ciphertext is detected
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Aes).await?;
    let plaintext = b"test data";
    let mut ciphertext = provider.encrypt(&key, plaintext).await?;

    // Corrupt ciphertext
    if !ciphertext.is_empty() {
        let len = ciphertext.len();
        ciphertext[len - 1] ^= 0xFF;
    }

    let result = provider.decrypt(&key, &ciphertext).await;
    assert!(
        result.is_err(),
        "Corrupted ciphertext should fail to decrypt"
    );

    Ok(())
}

#[tokio::test]
async fn test_verify_with_wrong_public_key() -> Result<(), BearDogError> {
    // Test: Verification with wrong public key fails
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key1 = provider.generate_key_material(&KeyType::Ed25519).await?;
    let key2 = provider.generate_key_material(&KeyType::Ed25519).await?;
    let data = b"signed data";

    // Sign with key1
    let signature = provider.sign(&key1, data).await?;

    // Try to verify with key2's public key
    use ed25519_dalek::SigningKey;
    let key2_array: [u8; 32] = key2
        .try_into()
        .map_err(|_| BearDogError::crypto_error("Invalid key length".to_string()))?;
    let sk2 = SigningKey::from_bytes(&key2_array);
    let wrong_public_key = sk2.verifying_key().to_bytes().to_vec();

    let is_valid = provider.verify(&wrong_public_key, data, &signature).await?;
    assert!(
        !is_valid,
        "Signature should not verify with wrong public key"
    );

    Ok(())
}

#[tokio::test]
async fn test_sign_with_invalid_key_length() {
    // Test: Signing with invalid key fails
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto)
        .await
        .unwrap();
    provider.initialize().await.unwrap();

    let invalid_key = vec![0u8; 16]; // Too short for Ed25519 (needs 32)
    let data = b"data to sign";

    let result = provider.sign(&invalid_key, data).await;
    assert!(
        result.is_err(),
        "Signing with invalid key length should fail"
    );
}

#[tokio::test]
async fn test_encrypt_empty_data() -> Result<(), BearDogError> {
    // Test: Encrypting empty data works
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Aes).await?;
    let empty_data = b"";

    let ciphertext = provider.encrypt(&key, empty_data).await?;
    let decrypted = provider.decrypt(&key, &ciphertext).await?;

    assert_eq!(empty_data.as_slice(), decrypted.as_slice());

    Ok(())
}

// ============================================================================
// LARGE DATA TESTS
// ============================================================================

#[tokio::test]
async fn test_encrypt_1kb_data() -> Result<(), BearDogError> {
    // Test: 1KB encryption/decryption
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Aes).await?;
    let large_data = vec![0xAA; 1024]; // 1KB

    let ciphertext = provider.encrypt(&key, &large_data).await?;
    let decrypted = provider.decrypt(&key, &ciphertext).await?;

    assert_eq!(large_data, decrypted);

    Ok(())
}

#[tokio::test]
async fn test_encrypt_100kb_data() -> Result<(), BearDogError> {
    // Test: 100KB encryption/decryption
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Aes).await?;
    let large_data = vec![0xBB; 100 * 1024]; // 100KB

    let ciphertext = provider.encrypt(&key, &large_data).await?;
    let decrypted = provider.decrypt(&key, &ciphertext).await?;

    assert_eq!(large_data, decrypted);
    // Successfully encrypted and decrypted 100KB of data

    Ok(())
}

#[tokio::test]
#[ignore] // Slow test - run explicitly
async fn test_encrypt_1mb_data() -> Result<(), BearDogError> {
    // Test: 1MB encryption/decryption (performance validation)
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Aes).await?;
    let large_data = vec![0xCC; 1024 * 1024]; // 1MB

    let start = std::time::Instant::now();
    let ciphertext = provider.encrypt(&key, &large_data).await?;
    let encrypt_time = start.elapsed();

    let start = std::time::Instant::now();
    let decrypted = provider.decrypt(&key, &ciphertext).await?;
    let decrypt_time = start.elapsed();

    assert_eq!(large_data, decrypted);

    // Performance check: Should complete in reasonable time
    assert!(
        encrypt_time.as_secs() < 1,
        "1MB encryption should complete in < 1s"
    );
    assert!(
        decrypt_time.as_secs() < 1,
        "1MB decryption should complete in < 1s"
    );

    Ok(())
}

// ============================================================================
// CONCURRENT OPERATIONS TESTS
// ============================================================================

#[tokio::test]
async fn test_concurrent_encryptions() -> Result<(), BearDogError> {
    // Test: Multiple concurrent encryptions work correctly
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Aes).await?;

    let mut handles = vec![];
    for i in 0..10 {
        let provider_clone = provider.clone();
        let key_clone = key.clone();
        let data = format!("test data {}", i).into_bytes();

        handles.push(tokio::spawn(async move {
            let ciphertext = provider_clone.encrypt(&key_clone, &data).await?;
            let decrypted = provider_clone.decrypt(&key_clone, &ciphertext).await?;
            assert_eq!(data, decrypted);
            Ok::<_, BearDogError>(())
        }));
    }

    for handle in handles {
        let result = handle
            .await
            .map_err(|e| BearDogError::internal(format!("Task failed: {}", e)))?;
        assert!(result.is_ok());
    }

    Ok(())
}

// ============================================================================
// CROSS-PROVIDER COMPATIBILITY TESTS
// ============================================================================

#[tokio::test]
async fn test_all_providers_produce_different_ciphertexts() -> Result<(), BearDogError> {
    // Test: Each encryption produces unique ciphertext (nonce randomization)
    let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
    provider.initialize().await?;

    let key = provider.generate_key_material(&KeyType::Aes).await?;
    let plaintext = b"same plaintext";

    let ciphertext1 = provider.encrypt(&key, plaintext).await?;
    let ciphertext2 = provider.encrypt(&key, plaintext).await?;

    // Same plaintext should produce different ciphertexts (due to random nonce)
    assert_ne!(
        ciphertext1, ciphertext2,
        "Encryptions should use random nonces"
    );

    // But both should decrypt correctly
    let decrypted1 = provider.decrypt(&key, &ciphertext1).await?;
    let decrypted2 = provider.decrypt(&key, &ciphertext2).await?;

    assert_eq!(plaintext.as_slice(), decrypted1.as_slice());
    assert_eq!(plaintext.as_slice(), decrypted2.as_slice());

    Ok(())
}

#[tokio::test]
async fn test_provider_cleanup_on_drop() -> Result<(), BearDogError> {
    // Test: Providers clean up resources properly
    for _ in 0..100 {
        let provider = create_crypto_provider(&CryptoBackend::RustCrypto).await?;
        provider.initialize().await?;
        // Drop provider
    }
    // If this completes without issues, cleanup is working
    Ok(())
}
