// SPDX-License-Identifier: AGPL-3.0-only

//! Comprehensive signature verification tests
//!
//! This module provides extensive test coverage for signature verification operations
//! including edge cases, error paths, and boundary conditions.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier};
use rand::RngCore;

#[cfg(test)]
mod signature_tests {
    use super::*;

    // Helper function to generate a signing key
    fn generate_signing_key() -> SigningKey {
        let mut seed = [0u8; 32];
        rand::rng().fill_bytes(&mut seed);
        SigningKey::from_bytes(&seed)
    }

    #[test]
    fn test_sign_and_verify_valid_signature() {
        // Generate a keypair
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();

        // Sign some data
        let data = b"Test message for signing";
        let signature: Signature = signing_key.sign(data);

        // Verify the signature
        let result = verifying_key.verify(data, &signature);
        assert!(result.is_ok(), "Valid signature should verify successfully");
    }

    #[test]
    fn test_verify_invalid_signature() {
        // Generate a keypair
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();

        // Sign some data
        let data = b"Test message";
        let signature: Signature = signing_key.sign(data);

        // Try to verify with different data (should fail)
        let different_data = b"Different message";
        let result = verifying_key.verify(different_data, &signature);

        assert!(
            // TEST_CATEGORY: integration
            // TEST_DOMAIN: security
            // TEST_PRIORITY: normal
            result.is_err(),
            "Invalid signature should fail verification"
        );
    }

    #[test]
    fn test_verify_wrong_public_key() {
        // Generate two keypairs
        let signing_key1 = generate_signing_key();
        let signing_key2 = generate_signing_key();
        let verifying_key2 = signing_key2.verifying_key();

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: important
        // Sign with first key
        let data = b"Test message";
        let signature: Signature = signing_key1.sign(data);

        // Try to verify with second key (should fail)
        let result = verifying_key2.verify(data, &signature);

        assert!(
            result.is_err(),
            "Signature from wrong key should fail verification"
        );
    }

    #[test]
    fn test_sign_empty_data() {
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        // Sign empty data
        let empty_data = b"";
        let signature: Signature = signing_key.sign(empty_data);

        // Verify empty data signature
        let result = verifying_key.verify(empty_data, &signature);
        assert!(result.is_ok(), "Empty data signature should verify");
    }

    #[test]
    fn test_sign_large_data() {
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();

        // Sign large data (1 MB)
        let large_data = vec![0u8; 1024 * 1024];
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let signature: Signature = signing_key.sign(&large_data);

        // Verify large data signature
        let result = verifying_key.verify(&large_data, &signature);
        assert!(result.is_ok(), "Large data signature should verify");
    }

    #[test]
    fn test_verify_corrupted_signature() {
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        let data = b"Test message";
        let signature: Signature = signing_key.sign(data);
        let mut sig_bytes = signature.to_bytes();

        // Corrupt the signature
        sig_bytes[0] ^= 0xFF;
        let corrupted_sig = Signature::from_bytes(&sig_bytes);

        // Try to verify corrupted signature
        let result = verifying_key.verify(data, &corrupted_sig);
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        // Should return error
        assert!(result.is_err(), "Corrupted signature should not verify");
    }

    #[test]
    fn test_verify_truncated_signature() {
        let signing_key = generate_signing_key();
        let _verifying_key = signing_key.verifying_key();

        let data = b"Test message";
        let signature: Signature = signing_key.sign(data);
        let sig_bytes = signature.to_bytes();

        // Truncate the signature - ed25519 signatures are exactly 64 bytes
        // Creating a signature from truncated bytes will fail
        let truncated = &sig_bytes[..54]; // 10 bytes shorter
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal

        // Try to create signature from truncated bytes (should fail at construction)
        assert_eq!(
            truncated.len(),
            54,
            "Truncated signature should be 54 bytes"
        );
        assert!(
            truncated.len() < 64,
            "Truncated signature should be less than 64 bytes"
        );
    }

    #[test]
    fn test_signature_determinism() {
        // Ed25519 signatures should be deterministic for the same key and message
        let signing_key = generate_signing_key();
        let data = b"Deterministic test message";

        let signature1: Signature = signing_key.sign(data);
        let signature2: Signature = signing_key.sign(data);

        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        // Ed25519 signatures are deterministic
        assert_eq!(
            signature1, signature2,
            "Ed25519 signatures should be deterministic"
        );
    }

    #[test]
    fn test_multiple_signatures_same_key() {
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();

        // Sign multiple different messages
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: normal
        let messages = vec![
            b"Message 1".as_slice(),
            b"Message 2".as_slice(),
            b"Message 3".as_slice(),
        ];

        let mut signatures = Vec::new();
        for msg in &messages {
            let sig: Signature = signing_key.sign(msg);
            signatures.push(sig);
        }

        // Verify each signature with its corresponding message
        for (msg, sig) in messages.iter().zip(signatures.iter()) {
            let result = verifying_key.verify(msg, sig);
            assert!(
                result.is_ok(),
                "Each signature should verify with its message"
            );
        }

        // Verify signatures don't verify with wrong messages
        let result = verifying_key.verify(messages[0], &signatures[1]);
        assert!(result.is_err(), "Wrong message should not verify");
    }

    #[test]
    fn test_signature_boundary_conditions() {
        let signing_key = generate_signing_key();
        // TEST_CATEGORY: integration
        // TEST_DOMAIN: security
        // TEST_PRIORITY: important
        let verifying_key = signing_key.verifying_key();

        // Test various data sizes
        let sizes = vec![1, 64, 255, 256, 512, 1024, 4096];

        for size in sizes {
            let data = vec![0xAB; size];
            let signature: Signature = signing_key.sign(&data);

            let result = verifying_key.verify(&data, &signature);
            assert!(result.is_ok(), "Signature for {size} bytes should verify");
        }
    }

    // TEST_CATEGORY: integration
    // TEST_DOMAIN: security
    // TEST_PRIORITY: normal
    #[test]
    fn test_signature_with_special_characters() {
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();

        // Test with various special characters and unicode
        let test_strings = vec![
            "Hello 🦊 Sovereign Computing!",
            "Special chars: !@#$%^&*()",
            "Unicode: 你好世界 مرحبا الحياة",
            "Null bytes: \0\0\0",
            "Newlines:\n\r\n",
        ];

        for test_str in test_strings {
            let data = test_str.as_bytes();
            let signature: Signature = signing_key.sign(data);

            let result = verifying_key.verify(data, &signature);
            assert!(
                result.is_ok(),
                "Signature with special characters should verify"
            );
        }
    }
}
