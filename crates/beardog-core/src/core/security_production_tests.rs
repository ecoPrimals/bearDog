// SPDX-License-Identifier: AGPL-3.0-or-later

// Strategic Test Coverage - CoreSecurityProvider Real Crypto
//
// Tests for the newly evolved real cryptographic operations

use crate::core::security::CoreSecurityProvider;
use beardog_errors::BearDogError;
use beardog_types::canonical::config::unified::UnifiedBearDogConfig;
use beardog_types::canonical::providers_unified::traits::UnifiedSecurityProvider;

#[tokio::test]
async fn test_encrypt_decrypt_roundtrip_real_crypto() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    let plaintext = b"BearDog production-grade encryption test";
    let key_id = "test-key-1";

    // Encrypt with real AES-256-GCM
    let ciphertext = provider.encrypt(plaintext, key_id).await?;

    // Verify encryption happened (ciphertext != plaintext)
    assert_ne!(
        ciphertext.as_slice(),
        plaintext,
        "Ciphertext must differ from plaintext"
    );

    // Verify ciphertext is larger (nonce + auth tag overhead)
    assert!(
        ciphertext.len() > plaintext.len(),
        "Ciphertext should include nonce and auth tag"
    );

    // Decrypt
    let decrypted = provider.decrypt(&ciphertext, key_id).await?;

    // Verify perfect round-trip
    assert_eq!(
        decrypted.as_slice(),
        plaintext,
        "Decryption must recover original plaintext"
    );

    Ok(())
}

#[tokio::test]
async fn test_generate_random_is_not_zeros() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    // Generate random bytes
    let random1 = provider.generate_random(32).await?;

    // Verify not all zeros (would indicate mock implementation)
    let all_zeros = random1.iter().all(|&b| b == 0);
    assert!(!all_zeros, "Random generation must not return all zeros");

    Ok(())
}

#[tokio::test]
async fn test_generate_random_uniqueness() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    // Generate two samples
    let random1 = provider.generate_random(32).await?;
    let random2 = provider.generate_random(32).await?;

    // They should be different (extremely unlikely to be identical)
    assert_ne!(
        random1, random2,
        "Consecutive random samples must be unique"
    );

    Ok(())
}

#[tokio::test]
async fn test_generate_random_distribution() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    // Generate larger sample for statistical testing
    let random = provider.generate_random(1024).await?;

    // Calculate byte frequency distribution
    let mut frequency = [0u32; 256];
    for &byte in &random {
        frequency[byte as usize] += 1;
    }

    // Check that we have reasonable distribution (not all same byte)
    let unique_bytes = frequency.iter().filter(|&&count| count > 0).count();
    assert!(
        unique_bytes > 200,
        "Random data should have diverse byte distribution, got {unique_bytes} unique bytes"
    );

    Ok(())
}

#[tokio::test]
async fn test_sign_produces_valid_signature() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    let data = b"Important message to sign";
    let key_id = "signing-key-1";

    // Sign data with real Ed25519
    let signature = provider.sign(data, key_id).await?;

    // Ed25519 signatures are always 64 bytes
    assert_eq!(
        signature.len(),
        64,
        "Ed25519 signature must be 64 bytes, got {}",
        signature.len()
    );

    // Verify signature is not all zeros
    let all_zeros = signature.iter().all(|&b| b == 0);
    assert!(
        !all_zeros,
        "Signature must not be all zeros (would indicate mock)"
    );

    Ok(())
}

#[tokio::test]
async fn test_encrypt_different_data_different_output() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    let key_id = "test-key-2";

    // Encrypt two different plaintexts
    let plaintext1 = b"First message";
    let plaintext2 = b"Second message";

    let ciphertext1 = provider.encrypt(plaintext1, key_id).await?;
    let ciphertext2 = provider.encrypt(plaintext2, key_id).await?;

    // Ciphertexts must be different
    assert_ne!(
        ciphertext1, ciphertext2,
        "Different plaintexts must produce different ciphertexts"
    );

    Ok(())
}

#[tokio::test]
async fn test_encrypt_same_data_different_output() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    let plaintext = b"Same message encrypted twice";
    let key_id = "test-key-3";

    // Encrypt same data twice
    let ciphertext1 = provider.encrypt(plaintext, key_id).await?;
    let ciphertext2 = provider.encrypt(plaintext, key_id).await?;

    // Ciphertexts must be different (due to random nonce)
    assert_ne!(
        ciphertext1, ciphertext2,
        "Same plaintext encrypted twice must produce different ciphertexts (IND-CPA security)"
    );

    Ok(())
}

#[tokio::test]
async fn test_encrypt_empty_data() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    let plaintext = b"";
    let key_id = "test-key-empty";

    // Should handle empty data
    let ciphertext = provider.encrypt(plaintext, key_id).await?;

    // Even empty plaintext produces ciphertext (nonce + auth tag)
    assert!(
        !ciphertext.is_empty(),
        "Empty plaintext should still produce ciphertext (nonce + tag)"
    );

    // Decrypt should recover empty plaintext
    let decrypted = provider.decrypt(&ciphertext, key_id).await?;
    assert_eq!(decrypted.as_slice(), plaintext);

    Ok(())
}

#[tokio::test]
async fn test_encrypt_large_data() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    // Test with 1 MB of data
    let plaintext = vec![42u8; 1024 * 1024];
    let key_id = "test-key-large";

    // Should handle large data efficiently
    let ciphertext = provider.encrypt(&plaintext, key_id).await?;

    // Verify size is reasonable (overhead should be small)
    let overhead = ciphertext.len() - plaintext.len();
    assert!(
        overhead < 100,
        "Overhead should be minimal (nonce + tag), got {overhead} bytes"
    );

    // Decrypt and verify
    let decrypted = provider.decrypt(&ciphertext, key_id).await?;
    assert_eq!(decrypted.len(), plaintext.len());

    Ok(())
}

#[tokio::test]
async fn test_decrypt_invalid_ciphertext_fails() {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    // Invalid ciphertext (too short, no nonce)
    let invalid_ciphertext = vec![1, 2, 3];
    let key_id = "test-key-invalid";

    // Should fail gracefully
    let result = provider.decrypt(&invalid_ciphertext, key_id).await;
    assert!(
        result.is_err(),
        "Decryption of invalid ciphertext should fail"
    );
}

#[tokio::test]
async fn test_random_generation_concurrent() -> Result<(), BearDogError> {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    // Generate random in parallel
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let p = provider.clone();
            tokio::spawn(async move { p.generate_random(32).await })
        })
        .collect();

    // Collect all results
    let mut results = Vec::new();
    for handle in handles {
        let random = handle.await.unwrap()?;
        results.push(random);
    }

    // All should be unique
    for i in 0..results.len() {
        for j in i + 1..results.len() {
            assert_ne!(
                results[i], results[j],
                "Concurrent random generation must produce unique outputs"
            );
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_security_context_reflects_production() {
    let config = UnifiedBearDogConfig::default();
    let provider = CoreSecurityProvider::new(config);

    let context = provider.security_context();

    // Should reflect production-grade security
    assert_eq!(context.security_level, "production");

    // Should list real algorithms (not mock)
    assert!(
        context
            .encryption_algorithms
            .contains(&"aes-256-gcm".to_string())
    );
    assert!(
        context
            .signature_algorithms
            .contains(&"ed25519".to_string())
    );
    assert!(context.random_generators.contains(&"os_csprng".to_string()));

    // Should not list mock generators
    assert!(!context.random_generators.contains(&"mock".to_string()));
    assert!(!context.random_generators.contains(&"zeros".to_string()));
}
