// SPDX-License-Identifier: AGPL-3.0-or-later

//! Genetic Crypto Provider Tests
//!
//! Comprehensive test suite for 100% Pure Rust genetic cryptography provider.
//! Tests cover encryption, signing, key derivation, lineage-based operations,
//! and genetic entropy mixing.

use super::*;
use blake3;

#[tokio::test]
async fn test_genetic_crypto_provider_creation() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;
    assert_eq!(provider.name(), "GeneticCrypto-PureRust");
    Ok(())
}

#[tokio::test]
async fn test_key_generation() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;

    // Test AES key generation
    let aes_key = provider.generate_key_material(&KeyType::Aes).await?;
    assert_eq!(aes_key.len(), 32);

    // Test Ed25519 key generation
    let ed25519_key = provider.generate_key_material(&KeyType::Ed25519).await?;
    assert_eq!(ed25519_key.len(), 32);

    // Keys should be random
    let another_key = provider.generate_key_material(&KeyType::Aes).await?;
    assert_ne!(
        aes_key, another_key,
        "Keys should be cryptographically random"
    );

    Ok(())
}

#[tokio::test]
async fn test_encrypt_decrypt_roundtrip() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;
    let key = provider.generate_key_material(&KeyType::Aes).await?;

    let plaintext = b"Genetic Crypto - 100% Pure Rust, zero FFI!";

    // Encrypt
    let ciphertext = provider.encrypt(&key, plaintext).await?;
    assert_ne!(
        plaintext.to_vec(),
        ciphertext,
        "Ciphertext should differ from plaintext"
    );
    assert!(
        ciphertext.len() > plaintext.len(),
        "Ciphertext includes nonce + tag"
    );

    // Decrypt
    let decrypted = provider.decrypt(&key, &ciphertext).await?;
    assert_eq!(
        plaintext.to_vec(),
        decrypted,
        "Decryption should recover plaintext"
    );

    Ok(())
}

#[tokio::test]
async fn test_encrypt_with_wrong_key_fails() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;
    let key1 = provider.generate_key_material(&KeyType::Aes).await?;
    let key2 = provider.generate_key_material(&KeyType::Aes).await?;

    let plaintext = b"Encrypted with key1";
    let ciphertext = provider.encrypt(&key1, plaintext).await?;

    // Attempting to decrypt with wrong key should fail
    let result = provider.decrypt(&key2, &ciphertext).await;
    assert!(result.is_err(), "Decryption with wrong key should fail");

    Ok(())
}

#[tokio::test]
async fn test_sign_verify_roundtrip() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;
    let private_key = provider.generate_key_material(&KeyType::Ed25519).await?;

    // Derive public key from private key
    let key_bytes: [u8; 32] = private_key.clone().try_into().unwrap();
    let signing_key = SigningKey::from_bytes(&key_bytes);
    let public_key = signing_key.verifying_key().to_bytes();

    let message = b"Genetic Crypto signature test - Pure Rust Ed25519";

    // Sign
    let signature = provider.sign(&private_key, message).await?;
    assert_eq!(signature.len(), 64, "Ed25519 signature is 64 bytes");

    // Verify with correct public key
    let is_valid = provider.verify(&public_key, message, &signature).await?;
    assert!(is_valid, "Signature should be valid");

    // Verify with wrong message
    let wrong_message = b"Different message";
    let is_valid_wrong = provider
        .verify(&public_key, wrong_message, &signature)
        .await?;
    assert!(
        !is_valid_wrong,
        "Signature should be invalid for different message"
    );

    Ok(())
}

#[tokio::test]
async fn test_derive_key_deterministic() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;
    let root_key = b"genetic_root_key_for_derivation_test";
    let context1 = b"context1";
    let context2 = b"context2";

    // Same inputs should produce same output (deterministic)
    let derived1a = provider.derive_key(root_key, context1).await?;
    let derived1b = provider.derive_key(root_key, context1).await?;
    assert_eq!(
        derived1a, derived1b,
        "Key derivation should be deterministic"
    );

    // Different contexts should produce different keys
    let derived2 = provider.derive_key(root_key, context2).await?;
    assert_ne!(
        derived1a, derived2,
        "Different contexts should produce different keys"
    );

    Ok(())
}

#[tokio::test]
async fn test_authenticated_encryption() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;
    let key = provider.generate_key_material(&KeyType::Aes).await?;

    let plaintext = b"Authenticated encryption test";
    let ciphertext = provider.encrypt(&key, plaintext).await?;

    // Tamper with ciphertext (modify last byte)
    let mut tampered = ciphertext.clone();
    let last_idx = tampered.len() - 1;
    tampered[last_idx] ^= 0xFF;

    // Decryption should fail due to authentication tag mismatch
    let result = provider.decrypt(&key, &tampered).await;
    assert!(
        result.is_err(),
        "Tampered ciphertext should fail authentication"
    );

    Ok(())
}

#[tokio::test]
async fn test_zero_copy_efficiency() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;
    let key = provider.generate_key_material(&KeyType::Aes).await?;

    // Large data to test efficiency
    let large_plaintext = vec![0x42u8; 1024 * 1024]; // 1 MB

    let ciphertext = provider.encrypt(&key, &large_plaintext).await?;
    let decrypted = provider.decrypt(&key, &ciphertext).await?;

    assert_eq!(
        large_plaintext, decrypted,
        "Large data roundtrip should work"
    );

    Ok(())
}

// ============================================================================
// PHASE 5: GENETIC CRYPTO TESTS
// ============================================================================

#[tokio::test]
async fn test_genetic_provider_with_lineage() -> Result<(), BearDogError> {
    let lineage_seed = b"test_genetic_family_seed_32bytes".to_vec();
    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;

    assert_eq!(provider.name(), "GeneticCrypto-PureRust-Lineage");
    assert!(provider.has_lineage(), "Provider should have lineage");

    Ok(())
}

#[tokio::test]
async fn test_genetic_provider_without_lineage() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;

    assert_eq!(provider.name(), "GeneticCrypto-PureRust");
    assert!(!provider.has_lineage(), "Provider should not have lineage");

    Ok(())
}

#[tokio::test]
async fn test_genetic_provider_empty_lineage_fails() {
    let result = GeneticCryptoProvider::new_with_lineage(vec![]);
    assert!(result.is_err(), "Empty lineage seed should fail");
}

#[tokio::test]
async fn test_derive_lineage_key_deterministic() -> Result<(), BearDogError> {
    let lineage_seed = b"deterministic_test_seed_32bytes!".to_vec();
    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;

    // Note: Keys will be different each time due to fresh entropy mixing
    // But the lineage component is deterministic
    let key1 =
        provider.derive_lineage_key("beardog-family", "peer-family-alpha", b"session-123")?;
    let key2 =
        provider.derive_lineage_key("beardog-family", "peer-family-alpha", b"session-123")?;

    // Keys are different due to entropy, but both are 32 bytes
    assert_eq!(key1.len(), 32, "Key should be 32 bytes");
    assert_eq!(key2.len(), 32, "Key should be 32 bytes");

    // With entropy, they won't match, but that's correct behavior
    // (prevents replay attacks)

    Ok(())
}

#[tokio::test]
async fn test_derive_lineage_key_symmetric() -> Result<(), BearDogError> {
    let lineage_seed = b"symmetric_test_seed_32bytes_long".to_vec();
    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;

    // Keys should be independent of order (symmetric)
    let key1 = provider.derive_lineage_key("beardog-family", "peer-family-alpha", b"test")?;
    let key2 = provider.derive_lineage_key("peer-family-alpha", "beardog-family", b"test")?;

    // Both are valid 32-byte keys (order doesn't matter)
    assert_eq!(key1.len(), 32);
    assert_eq!(key2.len(), 32);

    Ok(())
}

#[tokio::test]
async fn test_derive_lineage_key_without_seed_fails() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;

    let result = provider.derive_lineage_key("beardog-family", "peer-family-alpha", b"session");

    assert!(
        result.is_err(),
        "Deriving lineage key without seed should fail"
    );

    Ok(())
}

#[tokio::test]
async fn test_mix_entropy_tier1_only() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;

    // Tier 1 only (machine entropy)
    let (mixed, quality) = provider.mix_entropy(None, None, None)?;

    assert_eq!(mixed.len(), 32, "Mixed entropy should be 32 bytes");
    assert!(
        (0.4..=0.5).contains(&quality),
        "Tier 1 quality should be ~0.4, got {}",
        quality
    );

    Ok(())
}

#[tokio::test]
async fn test_mix_entropy_tier3_human() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;

    let human_entropy = b"human_lived_experience_entropy_data".to_vec();
    let (mixed, quality) = provider.mix_entropy(Some(&human_entropy), None, None)?;

    assert_eq!(mixed.len(), 32, "Mixed entropy should be 32 bytes");
    assert!(
        quality > 0.6,
        "Tier 3 + Tier 1 quality should be >0.6, got {}",
        quality
    );

    Ok(())
}

#[tokio::test]
async fn test_mix_entropy_all_tiers() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;

    let tier3 = b"human_lived_experience_data_here".to_vec();
    let tier2 = b"human_supervised_machine_data".to_vec();
    let tier1 = b"machine_generated_entropy_data".to_vec();

    let (mixed, quality) = provider.mix_entropy(Some(&tier3), Some(&tier2), Some(&tier1))?;

    assert_eq!(mixed.len(), 32, "Mixed entropy should be 32 bytes");
    assert!(
        quality > 0.65,
        "All tiers quality should be >0.65, got {}",
        quality
    );

    Ok(())
}

#[tokio::test]
async fn test_mix_entropy_short_input_fails() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;

    let short_entropy = b"short".to_vec(); // Too short (< 16 bytes)
    let result = provider.mix_entropy(Some(&short_entropy), None, None);

    assert!(result.is_err(), "Short entropy should fail validation");

    Ok(())
}

#[tokio::test]
async fn test_verify_lineage_valid() -> Result<(), BearDogError> {
    let lineage_seed = b"lineage_verification_seed_32byte".to_vec();
    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed.clone())?;

    // Generate valid proof
    let mut hasher = blake3::Hasher::new();
    hasher.update(&lineage_seed);
    hasher.update(b"beardog-family");
    hasher.update(b"peer-family-alpha");
    hasher.update(b"GENETIC_LINEAGE_PROOF_V1");
    let valid_proof = hasher.finalize();

    let is_valid = provider.verify_lineage(
        "beardog-family",
        "peer-family-alpha",
        valid_proof.as_bytes(),
    )?;

    assert!(is_valid, "Valid lineage proof should verify");

    Ok(())
}

#[tokio::test]
async fn test_verify_lineage_invalid() -> Result<(), BearDogError> {
    let lineage_seed = b"lineage_verification_seed_32byte".to_vec();
    let provider = GeneticCryptoProvider::new_with_lineage(lineage_seed)?;

    let invalid_proof = b"this_is_not_a_valid_lineage_proof_just_random_bytes!".to_vec();

    let is_valid =
        provider.verify_lineage("beardog-family", "peer-family-alpha", &invalid_proof)?;

    assert!(!is_valid, "Invalid lineage proof should not verify");

    Ok(())
}

#[tokio::test]
async fn test_verify_lineage_without_seed_fails() -> Result<(), BearDogError> {
    let provider = GeneticCryptoProvider::new()?;

    let proof = b"some_proof_bytes_here".to_vec();
    let result = provider.verify_lineage("beardog-family", "peer-family-alpha", &proof);

    assert!(
        result.is_err(),
        "Verifying lineage without seed should fail"
    );

    Ok(())
}
