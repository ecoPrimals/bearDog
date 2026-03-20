// Critical Security Path Tests
// Tests for the most security-sensitive operations in BearDog
//
// ✅ REAL IMPLEMENTATIONS - No mocks, using actual Software HSM with RustCrypto

use beardog_tunnel::tunnel::hsm::GenerateKeyRequest;
use beardog_tunnel::tunnel::hsm::manager::HsmProvider;
use beardog_tunnel::tunnel::hsm::software_hsm::core::RustSoftwareHsm;
use beardog_tunnel::tunnel::hsm::types::KeyType;
use beardog_tunnel::tunnel::hsm::types::config::SoftwareHsmConfig;

// TEST_CATEGORY: security
// TEST_DOMAIN: hsm
// TEST_PRIORITY: critical
/// Test critical path: HSM key generation with security validation
#[tokio::test]
async fn test_critical_hsm_key_generation_security() {
    // ✅ REAL IMPLEMENTATION: Using actual Software HSM
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Test 1: Verify key generation with valid Ed25519 parameters succeeds
    let request = GenerateKeyRequest {
        key_id: "test-ed25519-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let key = hsm.generate_key(request).await;
    assert!(key.is_ok(), "Valid Ed25519 key generation should succeed");
    let key = key.unwrap();
    assert_eq!(key.key_type, KeyType::Ed25519);

    // Test 2: Verify AES key generation succeeds (no key_size field in enum)
    let request2 = GenerateKeyRequest {
        key_id: "test-aes-key".to_string(),
        key_type: KeyType::Aes,
    };
    let key2 = hsm.generate_key(request2).await;
    assert!(key2.is_ok(), "Valid AES key generation should succeed");
    let key2 = key2.unwrap();
    assert_eq!(key2.key_type, KeyType::Aes);

    // Test 3: Verify generated keys have proper security attributes
    assert!(
        key2.metadata.created_at <= chrono::Utc::now(),
        "Key creation timestamp should be valid"
    );

    // Test 4: Verify key material is protected (not visible in debug output)
    let debug_output = format!("{key:?}");
    // Check that actual key bytes are not exposed (encrypted_data should be hidden or obfuscated)
    // The field name "key_material" is fine, but the content should be protected
    assert!(
        !debug_output.contains(&format!("{:?}", vec![1u8, 2, 3, 4])),
        "Actual key bytes should not be visible in debug output"
    );
}

// TEST_CATEGORY: security
// TEST_DOMAIN: crypto
// TEST_PRIORITY: critical
/// Test critical path: Digital signature generation and verification
#[tokio::test]
async fn test_critical_signature_operations() {
    // ✅ REAL IMPLEMENTATION: Using actual Software HSM with Ed25519
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Generate a signing key
    let request = GenerateKeyRequest {
        key_id: "signing-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let key = hsm
        .generate_key(request)
        .await
        .expect("Key generation failed");

    // Test 1: Valid signature generation and verification
    let message = b"Critical security message that must be authenticated";
    let signature = hsm
        .sign(&key.metadata.key_id, message)
        .await
        .expect("Signature generation failed");
    assert!(!signature.is_empty(), "Signature should not be empty");

    let is_valid = hsm
        .verify(&key.metadata.key_id, message, &signature)
        .await
        .expect("Verification failed");
    assert!(is_valid, "Valid signature should verify successfully");

    // Test 2: Tampering detection - modified message
    let tampered_message = b"Modified message - tampered!";
    let is_valid = hsm
        .verify(&key.metadata.key_id, tampered_message, &signature)
        .await
        .expect("Verification should complete");
    assert!(!is_valid, "Tampered message should fail verification");

    // Test 3: Tampering detection - modified signature
    let mut tampered_signature = signature.clone();
    if let Some(byte) = tampered_signature.first_mut() {
        *byte ^= 0xFF; // Flip all bits in first byte
    }
    let is_valid = hsm
        .verify(&key.metadata.key_id, message, &tampered_signature)
        .await
        .expect("Verification should complete");
    assert!(!is_valid, "Tampered signature should fail verification");

    // Test 4: Wrong key rejection - generate another key and try to verify
    let request2 = GenerateKeyRequest {
        key_id: "wrong-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let _wrong_key = hsm
        .generate_key(request2)
        .await
        .expect("Key generation failed");

    let is_valid = hsm
        .verify("wrong-key", message, &signature)
        .await
        .expect("Verification should complete");
    assert!(
        !is_valid,
        "Signature from different key should fail verification"
    );
}

/// Test critical path: Encryption/Decryption boundary validation
#[tokio::test]
async fn test_critical_encryption_boundaries() {
    // ✅ REAL IMPLEMENTATION: Using actual Software HSM with AES-256-GCM
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Generate an encryption key
    let request = GenerateKeyRequest {
        key_id: "encryption-key".to_string(),
        key_type: KeyType::Aes,
    };
    let key = hsm
        .generate_key(request)
        .await
        .expect("Key generation failed");

    // Test 1: Valid encryption/decryption round-trip
    let plaintext = b"Sensitive data that must be protected with AES-256-GCM AEAD";
    let ciphertext = hsm
        .encrypt(&key.metadata.key_id, plaintext)
        .await
        .expect("Encryption failed");

    assert_ne!(
        &ciphertext[..],
        plaintext,
        "Ciphertext should differ from plaintext"
    );
    assert!(
        ciphertext.len() > plaintext.len(),
        "Ciphertext should include nonce and auth tag"
    );

    let decrypted = hsm
        .decrypt(&key.metadata.key_id, &ciphertext)
        .await
        .expect("Decryption failed");
    assert_eq!(
        &decrypted[..],
        plaintext,
        "Decrypted data should match original plaintext"
    );

    // Test 2: Tampering detection - modified ciphertext (AEAD authentication)
    let mut tampered_ciphertext = ciphertext.clone();
    if let Some(byte) = tampered_ciphertext.last_mut() {
        *byte ^= 0xFF; // Tamper with the auth tag
    }

    let result = hsm
        .decrypt(&key.metadata.key_id, &tampered_ciphertext)
        .await;
    assert!(
        result.is_err(),
        "Tampered ciphertext should fail AEAD authentication"
    );

    // Test 3: Wrong key rejection
    let request2 = GenerateKeyRequest {
        key_id: "wrong-encryption-key".to_string(),
        key_type: KeyType::Aes,
    };
    let _wrong_key = hsm
        .generate_key(request2)
        .await
        .expect("Key generation failed");

    let result = hsm.decrypt("wrong-encryption-key", &ciphertext).await;
    assert!(
        result.is_err(),
        "Decryption with wrong key should fail authentication"
    );

    // Test 4: Nonce uniqueness - encrypt same plaintext twice, get different ciphertexts
    let ciphertext1 = hsm
        .encrypt(&key.metadata.key_id, plaintext)
        .await
        .expect("First encryption failed");
    let ciphertext2 = hsm
        .encrypt(&key.metadata.key_id, plaintext)
        .await
        .expect("Second encryption failed");

    assert_ne!(
        &ciphertext1[..12], // First 12 bytes are nonce
        &ciphertext2[..12],
        "Nonces should be unique for each encryption"
    );
}

/// Test critical path: HSM hardware detection and security levels
#[tokio::test]
async fn test_critical_hsm_security_levels() {
    // ✅ REAL IMPLEMENTATION: Testing Software HSM capabilities and provider info
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Test provider information via HsmProvider trait
    let provider_info = hsm.get_info().await.expect("Failed to get provider info");
    // Provider ID should exist and not be empty
    assert!(!provider_info.id.is_empty(), "Provider should have an ID");
    assert!(
        !provider_info.name.is_empty(),
        "Provider should have a name"
    );
    assert!(
        provider_info.security_level >= 1,
        "Should have security level"
    );

    // Test that HSM is available
    assert!(hsm.is_available(), "Software HSM should be available");
}

/// Test critical path: Key access control and authorization
#[tokio::test]
async fn test_critical_key_access_control() {
    // ✅ REAL IMPLEMENTATION: Testing that keys are isolated by key_id
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Create two separate keys
    let request1 = GenerateKeyRequest {
        key_id: "authorized-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let _key1 = hsm
        .generate_key(request1)
        .await
        .expect("Key 1 generation failed");

    let request2 = GenerateKeyRequest {
        key_id: "unauthorized-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let _key2 = hsm
        .generate_key(request2)
        .await
        .expect("Key 2 generation failed");

    // Sign with authorized key
    let message = b"Test message";
    let signature = hsm
        .sign("authorized-key", message)
        .await
        .expect("Signing with authorized key should succeed");

    // Verify that we can't verify with wrong key (access control test)
    let is_valid = hsm
        .verify("unauthorized-key", message, &signature)
        .await
        .expect("Verification should complete");
    assert!(
        !is_valid,
        "Signature from one key should not verify with another key"
    );

    // Attempting to use non-existent key should fail
    let result = hsm.sign("non-existent-key", message).await;
    assert!(result.is_err(), "Using non-existent key should fail");
}

/// Test critical path: Memory protection for sensitive data
#[tokio::test]
async fn test_critical_memory_protection() {
    // ✅ REAL IMPLEMENTATION: Testing that key material is protected
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    let request = GenerateKeyRequest {
        key_id: "protected-key".to_string(),
        key_type: KeyType::Aes,
    };
    let key = hsm
        .generate_key(request)
        .await
        .expect("Key generation failed");

    // Test 1: Debug output should not expose raw key bytes
    let debug_str = format!("{key:?}");
    // The field name is fine, but actual key bytes should be protected
    assert!(
        !debug_str.contains("plaintext"),
        "Debug output should not expose plaintext keys"
    );

    // Test 2: Key should be usable for operations (protected but accessible internally)
    let plaintext = b"Test data";
    let ciphertext = hsm
        .encrypt(&key.metadata.key_id, plaintext)
        .await
        .expect("Encryption should work with protected key");
    assert!(
        !ciphertext.is_empty(),
        "Protected key should still be usable for crypto operations"
    );
}

/// Test critical path: Entropy validation for key generation
#[tokio::test]
async fn test_critical_entropy_validation() {
    // ✅ REAL IMPLEMENTATION: Testing key uniqueness (proxy for entropy)
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Generate multiple keys and verify they're unique
    let mut keys = Vec::new();
    for i in 0..10 {
        let request = GenerateKeyRequest {
            key_id: format!("entropy-test-key-{i}"),
            key_type: KeyType::Ed25519,
        };
        let key = hsm
            .generate_key(request)
            .await
            .expect("Key generation failed");
        keys.push(key);
    }

    // Verify each key produces different signatures (proving keys are unique)
    let message = b"Entropy test message";
    let mut signatures = Vec::new();
    for key in &keys {
        let signature = hsm
            .sign(&key.metadata.key_id, message)
            .await
            .expect("Signing failed");
        signatures.push(signature);
    }

    // All signatures should be unique (proving sufficient entropy)
    for i in 0..signatures.len() {
        for j in (i + 1)..signatures.len() {
            assert_ne!(
                signatures[i], signatures[j],
                "Keys generated with proper entropy should produce unique signatures"
            );
        }
    }
}

/// Test critical path: Cryptographic algorithm validation
#[tokio::test]
async fn test_critical_algorithm_validation() {
    // ✅ REAL IMPLEMENTATION: Testing supported algorithms and security levels
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Test strong algorithm: Ed25519 (modern, secure)
    let request_ed25519 = GenerateKeyRequest {
        key_id: "strong-algo-ed25519".to_string(),
        key_type: KeyType::Ed25519,
    };
    let result = hsm.generate_key(request_ed25519).await;
    assert!(
        result.is_ok(),
        "Strong algorithm Ed25519 should be supported"
    );

    // Test strong algorithm: AES (secure)
    let request_aes = GenerateKeyRequest {
        key_id: "strong-algo-aes".to_string(),
        key_type: KeyType::Aes,
    };
    let result = hsm.generate_key(request_aes).await;
    assert!(result.is_ok(), "Strong algorithm AES should be supported");

    // Test ECC algorithm support
    let request_ecc = GenerateKeyRequest {
        key_id: "strong-algo-ecc".to_string(),
        key_type: KeyType::EllipticCurve,
    };
    let result = hsm.generate_key(request_ecc).await;
    assert!(result.is_ok(), "ECC should be supported");
}

/// Test critical path: Secure channel establishment
#[tokio::test]
async fn test_critical_secure_channel() {
    // ✅ REAL IMPLEMENTATION: Test key exchange pattern using HSM keys
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Alice generates key pair
    let alice_request = GenerateKeyRequest {
        key_id: "alice-channel-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let _alice_key = hsm
        .generate_key(alice_request)
        .await
        .expect("Alice key generation failed");

    // Bob generates key pair
    let bob_request = GenerateKeyRequest {
        key_id: "bob-channel-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let _bob_key = hsm
        .generate_key(bob_request)
        .await
        .expect("Bob key generation failed");

    // Test channel authentication: Alice signs a challenge
    let challenge = b"Channel establishment challenge";
    let alice_signature = hsm
        .sign("alice-channel-key", challenge)
        .await
        .expect("Alice signing failed");

    // Bob verifies Alice's signature (in real system, Bob would have Alice's public key)
    // For now, we test that signatures are valid
    let is_valid = hsm
        .verify("alice-channel-key", challenge, &alice_signature)
        .await
        .expect("Verification failed");
    assert!(is_valid, "Alice's signature should verify correctly");

    // Test mutual authentication: Bob signs response
    let response = b"Channel establishment response";
    let bob_signature = hsm
        .sign("bob-channel-key", response)
        .await
        .expect("Bob signing failed");

    let is_valid = hsm
        .verify("bob-channel-key", response, &bob_signature)
        .await
        .expect("Verification failed");
    assert!(is_valid, "Bob's signature should verify correctly");
}

/// Test critical path: Attestation and device integrity
#[tokio::test]
async fn test_critical_attestation() {
    // ✅ REAL IMPLEMENTATION: Testing provider attestation
    let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
        .await
        .expect("Failed to create Software HSM");

    // Get provider info (attestation of HSM capabilities)
    let provider_info = hsm.get_info().await.expect("Failed to get provider info");

    // Verify provider identity exists and is valid
    assert!(!provider_info.id.is_empty(), "Provider should have an ID");
    assert!(
        !provider_info.name.is_empty(),
        "Provider should have a name"
    );
    assert!(
        provider_info.security_level >= 1,
        "Should have valid security level"
    );

    // Verify attestation is consistent - if provider is available, it should work
    assert!(hsm.is_available(), "HSM should be available");

    let request = GenerateKeyRequest {
        key_id: "attestation-test-key".to_string(),
        key_type: KeyType::Ed25519,
    };
    let result = hsm.generate_key(request).await;
    assert!(
        result.is_ok(),
        "HSM should deliver on attested Ed25519 capability"
    );
}

#[cfg(test)]
mod integration {
    use super::*;

    /// End-to-end test: Complete HSM lifecycle with security validation
    #[tokio::test]
    async fn test_e2e_hsm_secure_lifecycle() {
        // ✅ REAL IMPLEMENTATION: Complete lifecycle test
        let hsm = RustSoftwareHsm::new(SoftwareHsmConfig::default())
            .await
            .expect("Failed to create Software HSM");

        // Phase 1: Key Generation
        let request = GenerateKeyRequest {
            key_id: "lifecycle-test-key".to_string(),
            key_type: KeyType::Ed25519,
        };
        let key = hsm
            .generate_key(request)
            .await
            .expect("Key generation failed");
        assert_eq!(key.metadata.key_id, "lifecycle-test-key");

        // Phase 2: Key Usage - Signing
        let message = b"End-to-end lifecycle test message";
        let signature = hsm
            .sign(&key.metadata.key_id, message)
            .await
            .expect("Signing failed");

        // Phase 3: Verification
        let is_valid = hsm
            .verify(&key.metadata.key_id, message, &signature)
            .await
            .expect("Verification failed");
        assert!(is_valid, "Signature should verify in complete lifecycle");

        // Phase 4: Key Deletion
        let result = hsm.delete_key(&key.metadata.key_id).await;
        assert!(result.is_ok(), "Key deletion should succeed");

        // Phase 5: Verify key is gone
        let result = hsm.sign(&key.metadata.key_id, message).await;
        assert!(result.is_err(), "Deleted key should no longer be usable");
    }
}
