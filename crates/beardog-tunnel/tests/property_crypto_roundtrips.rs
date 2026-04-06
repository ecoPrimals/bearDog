// SPDX-License-Identifier: AGPL-3.0-or-later
#![allow(clippy::expect_used, clippy::unwrap_used, missing_docs)]
//! Property-Based Testing for Cryptographic Roundtrips
//!
//! This test suite verifies that cryptographic operations maintain
//! their invariants across arbitrary inputs using property-based testing.
//!
//! ## What we test:
//!
//! 1. **Encryption/Decryption Roundtrip**: `decrypt(encrypt(plaintext)) == plaintext`
//! 2. **Signature Roundtrip**: `verify(sign(message), message) == true`
//! 3. **Key Derivation Determinism**: Same inputs → Same outputs
//!
//! ## Deep Debt Solution
//!
//! Property-based testing catches edge cases that unit tests miss:
//! - Empty inputs
//! - Very large inputs
//! - Unicode/binary edge cases
//! - Boundary conditions

use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::software_hsm::CryptoProvider;
use beardog_tunnel::tunnel::hsm::software_hsm::crypto_providers::GeneticCryptoProvider;
use ed25519_dalek::SigningKey;
use rand::Rng;
use std::sync::Arc;

/// Maximum plaintext size for tests (1 MB)
const MAX_PLAINTEXT_SIZE: usize = 1024 * 1024;

/// Property test: Encryption/Decryption roundtrip
///
/// For any plaintext `p`, `decrypt(encrypt(p)) == p`
#[tokio::test]
async fn property_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
    let provider = Arc::new(GeneticCryptoProvider::new()?);
    let iterations = 100; // Run 100 random tests

    for iteration in 0..iterations {
        // Generate random plaintext (0 to 100KB)
        let plaintext_len = rand::rng().random_range(0..=100_000);
        let mut plaintext = vec![0u8; plaintext_len];
        rand::rng().fill(&mut plaintext[..]);

        // Generate random key
        let key_bytes = provider.generate_random_bytes(32)?;

        // Encrypt
        let ciphertext = provider.encrypt(&key_bytes, &plaintext).await?;

        // Decrypt
        let decrypted = provider.decrypt(&key_bytes, &ciphertext).await?;

        // Property: decrypt(encrypt(p)) == p
        assert_eq!(
            plaintext, decrypted,
            "Encryption roundtrip failed for {plaintext_len} bytes (iteration {iteration})"
        );
    }

    Ok(())
}

/// Property test: Empty input handling
///
/// Crypto operations should handle empty inputs gracefully
#[tokio::test]
async fn property_empty_input_handling() -> Result<(), BearDogError> {
    let provider = Arc::new(GeneticCryptoProvider::new()?);
    let key_bytes = provider.generate_random_bytes(32)?;

    // Empty plaintext should encrypt and decrypt
    let plaintext = vec![];
    let ciphertext = provider.encrypt(&key_bytes, &plaintext).await?;
    let decrypted = provider.decrypt(&key_bytes, &ciphertext).await?;

    assert_eq!(plaintext, decrypted, "Empty plaintext roundtrip failed");

    Ok(())
}

/// Property test: Large input handling
///
/// Crypto operations should handle large inputs (up to 1MB)
#[tokio::test]
async fn property_large_input_handling() -> Result<(), BearDogError> {
    let provider = Arc::new(GeneticCryptoProvider::new()?);
    let key_bytes = provider.generate_random_bytes(32)?;

    // Test with 1MB plaintext
    let plaintext_len = MAX_PLAINTEXT_SIZE;
    let mut plaintext = vec![0u8; plaintext_len];
    rand::rng().fill(&mut plaintext[..]);

    let ciphertext = provider.encrypt(&key_bytes, &plaintext).await?;
    let decrypted = provider.decrypt(&key_bytes, &ciphertext).await?;

    assert_eq!(
        plaintext, decrypted,
        "Large plaintext roundtrip failed ({plaintext_len} bytes)"
    );

    Ok(())
}

/// Property test: Signature roundtrip
///
/// For any message `m`, `verify(sign(m), m) == true`
///
#[tokio::test]
async fn property_sign_verify_roundtrip() -> Result<(), BearDogError> {
    let provider = Arc::new(GeneticCryptoProvider::new()?);
    let iterations = 100;

    for iteration in 0..iterations {
        let message_len = rand::rng().random_range(1..=10_000);
        let mut message = vec![0u8; message_len];
        rand::rng().fill(&mut message[..]);

        let signing_key_bytes = provider.generate_random_bytes(32)?;
        let public_key =
            SigningKey::from_bytes(signing_key_bytes.as_slice().try_into().expect("32 bytes"))
                .verifying_key()
                .to_bytes()
                .to_vec();

        let signature = provider.sign(&signing_key_bytes, &message).await?;

        let verified = provider.verify(&public_key, &message, &signature).await?;

        assert!(
            verified,
            "Signature verification failed for {message_len} bytes (iteration {iteration})"
        );
    }

    Ok(())
}

/// Property test: Invalid signatures are rejected
///
/// For any message `m` and corrupt signature `s'`, `verify(s', m) == false`
///
#[tokio::test]
async fn property_invalid_signatures_rejected() -> Result<(), BearDogError> {
    let provider = Arc::new(GeneticCryptoProvider::new()?);
    let iterations = 50;

    for iteration in 0..iterations {
        // Generate random message
        let message_len = rand::rng().random_range(1..=1000);
        let mut message = vec![0u8; message_len];
        rand::rng().fill(&mut message[..]);

        let signing_key_bytes = provider.generate_random_bytes(32)?;
        let public_key =
            SigningKey::from_bytes(signing_key_bytes.as_slice().try_into().expect("32 bytes"))
                .verifying_key()
                .to_bytes()
                .to_vec();

        let mut signature = provider.sign(&signing_key_bytes, &message).await?;

        let corrupt_byte = rand::rng().random_range(0..signature.len());
        signature[corrupt_byte] ^= 0x01;

        let verified = provider
            .verify(&public_key, &message, &signature)
            .await
            .unwrap_or(false);

        // Property: corrupt signatures are rejected
        assert!(
            !verified,
            "Corrupted signature incorrectly verified (iteration {iteration})"
        );
    }

    Ok(())
}

/// Property test: Key derivation determinism
///
/// Same input → Same derived key
#[tokio::test]
async fn property_key_derivation_determinism() -> Result<(), BearDogError> {
    let provider = Arc::new(GeneticCryptoProvider::new()?);
    let iterations = 50;

    for iteration in 0..iterations {
        // Generate random password and salt
        let password_len = rand::rng().random_range(8..=64);
        let mut password = vec![0u8; password_len];
        rand::rng().fill(&mut password[..]);

        let salt_len = rand::rng().random_range(16..=32);
        let mut salt = vec![0u8; salt_len];
        rand::rng().fill(&mut salt[..]);

        // Derive key twice
        let key1 = provider.derive_key(&password, &salt).await?;
        let key2 = provider.derive_key(&password, &salt).await?;

        // Property: derive(p, s) == derive(p, s)
        assert_eq!(
            key1, key2,
            "Key derivation non-deterministic (iteration {iteration})"
        );
    }

    Ok(())
}

/// Property test: Different salts produce different keys
///
/// For same password but different salts, derived keys should differ
#[tokio::test]
async fn property_key_derivation_salt_sensitivity() -> Result<(), BearDogError> {
    let provider = Arc::new(GeneticCryptoProvider::new()?);
    let iterations = 50;

    for iteration in 0..iterations {
        // Generate random password
        let password_len = rand::rng().random_range(8..=64);
        let mut password = vec![0u8; password_len];
        rand::rng().fill(&mut password[..]);

        // Generate two different salts
        let salt_len = rand::rng().random_range(16..=32);
        let mut salt1 = vec![0u8; salt_len];
        let mut salt2 = vec![0u8; salt_len];
        rand::rng().fill(&mut salt1[..]);
        rand::rng().fill(&mut salt2[..]);

        // Ensure they're different
        if salt1 == salt2 {
            salt2[0] ^= 0x01;
        }

        // Derive keys
        let key1 = provider.derive_key(&password, &salt1).await?;
        let key2 = provider.derive_key(&password, &salt2).await?;

        // Property: different salts → different keys
        assert_ne!(
            key1, key2,
            "Key derivation not salt-sensitive (iteration {iteration})"
        );
    }

    Ok(())
}

/// Property test: Encryption with wrong key fails to decrypt correctly
///
/// `decrypt(wrong_key, encrypt(right_key, p)) != p`
#[tokio::test]
async fn property_wrong_key_decryption_fails() -> Result<(), BearDogError> {
    let provider = Arc::new(GeneticCryptoProvider::new()?);
    let iterations = 50;

    for iteration in 0..iterations {
        // Generate random plaintext
        let plaintext_len = rand::rng().random_range(1..=1000);
        let mut plaintext = vec![0u8; plaintext_len];
        rand::rng().fill(&mut plaintext[..]);

        // Generate two different keys
        let key1 = provider.generate_random_bytes(32)?;
        let key2 = provider.generate_random_bytes(32)?;

        // Encrypt with key1
        let ciphertext = provider.encrypt(&key1, &plaintext).await?;

        // Try to decrypt with key2 (should fail)
        let decrypt_result = provider.decrypt(&key2, &ciphertext).await;

        // Property: wrong key should fail to decrypt
        assert!(
            decrypt_result.is_err() || decrypt_result.unwrap() != plaintext,
            "Wrong key successfully decrypted (iteration {iteration})"
        );
    }

    Ok(())
}
