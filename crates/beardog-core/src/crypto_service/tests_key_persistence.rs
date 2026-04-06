// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tests for key persistence in crypto service
//!
//! Validates that public keys are properly stored and retrieved
//! for signature verification.

#[cfg(test)]
mod tests {
    use crate::crypto_service::implementation::BearDogCryptoService;
    use crate::crypto_service::r#trait::CryptoService;
    use beardog_types::crypto_service::{SignOptions, SignatureAlgorithm, VerifyOptions};

    fn create_test_service() -> BearDogCryptoService {
        let config = crate::crypto_service::types::CryptoServiceConfig {
            service_name: "test_service".to_string(),
            hsm_enabled: false,
            genetic_enabled: false,
            audit_enabled: false,
            max_data_size: 1024 * 1024,
        };
        BearDogCryptoService::new(config).expect("Failed to create crypto service")
    }

    #[tokio::test]
    async fn test_key_persistence_sign_and_verify() {
        let service = create_test_service();
        let data = b"test data to sign";
        let key_id = "test_signing_key";

        // Step 1: Sign data (this should store the public key)
        let signature = service
            .sign(
                data,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: key_id.to_string(),
                    context: None,
                },
            )
            .await
            .expect("Signing should succeed");

        assert_eq!(
            signature.signature.len(),
            64,
            "Ed25519 signature should be 64 bytes"
        );

        // Step 2: Retrieve the stored public key
        let public_key = service
            .get_public_key(key_id)
            .expect("Public key should be stored after signing");

        assert_eq!(
            public_key.len(),
            32,
            "Ed25519 public key should be 32 bytes"
        );

        // Step 3: Verify signature using the stored public key
        let verified = service
            .verify(
                data,
                &signature,
                VerifyOptions {
                    public_key,
                    context: None,
                },
            )
            .await
            .expect("Verification should succeed");

        assert!(
            verified,
            "Signature verification should pass with stored public key"
        );
    }

    #[tokio::test]
    async fn test_key_persistence_multiple_keys() {
        let service = create_test_service();
        let data1 = b"first message";
        let data2 = b"second message";
        let key_id1 = "key_1";
        let key_id2 = "key_2";

        // Sign with first key
        let sig1 = service
            .sign(
                data1,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: key_id1.to_string(),
                    context: None,
                },
            )
            .await
            .expect("Signing with key 1 should succeed");

        // Sign with second key
        let sig2 = service
            .sign(
                data2,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: key_id2.to_string(),
                    context: None,
                },
            )
            .await
            .expect("Signing with key 2 should succeed");

        // Retrieve both public keys
        let pub_key1 = service
            .get_public_key(key_id1)
            .expect("Public key 1 should be stored");
        let pub_key2 = service
            .get_public_key(key_id2)
            .expect("Public key 2 should be stored");

        // Keys should be different
        assert_ne!(
            pub_key1, pub_key2,
            "Different key IDs should generate different keys"
        );

        // Verify with correct keys
        let verified1 = service
            .verify(
                data1,
                &sig1,
                VerifyOptions {
                    public_key: pub_key1.clone(),
                    context: None,
                },
            )
            .await
            .expect("Verification 1 should succeed");

        let verified2 = service
            .verify(
                data2,
                &sig2,
                VerifyOptions {
                    public_key: pub_key2.clone(),
                    context: None,
                },
            )
            .await
            .expect("Verification 2 should succeed");

        assert!(verified1, "Signature 1 should verify with key 1");
        assert!(verified2, "Signature 2 should verify with key 2");

        // Cross-verification should fail
        let cross_verified = service
            .verify(
                data1,
                &sig1,
                VerifyOptions {
                    public_key: pub_key2, // Wrong key
                    context: None,
                },
            )
            .await
            .expect("Verification should not panic");

        assert!(!cross_verified, "Signature 1 should NOT verify with key 2");
    }

    #[tokio::test]
    async fn test_key_persistence_missing_key() {
        let service = create_test_service();
        let non_existent_key = "does_not_exist";

        // Attempt to retrieve a key that was never created
        let result = service.get_public_key(non_existent_key);

        assert!(
            result.is_err(),
            "Retrieving non-existent key should return error"
        );

        if let Err(e) = result {
            let error_msg = format!("{e}");
            assert!(
                error_msg.contains("Public key not found"),
                "Error message should indicate key not found"
            );
        }
    }

    #[tokio::test]
    async fn test_key_persistence_deterministic_keys() {
        let service = create_test_service();
        let data = b"test message";
        let key_id = "deterministic_key";

        // Sign twice with the same key_id
        let sig1 = service
            .sign(
                data,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: key_id.to_string(),
                    context: None,
                },
            )
            .await
            .expect("First signing should succeed");

        let sig2 = service
            .sign(
                data,
                SignatureAlgorithm::Ed25519,
                SignOptions {
                    key_id: key_id.to_string(),
                    context: None,
                },
            )
            .await
            .expect("Second signing should succeed");

        // Signatures should be identical (deterministic key derivation)
        assert_eq!(
            sig1.signature, sig2.signature,
            "Same key_id should produce same signature for same data"
        );

        // Public key should be the same
        let pub_key = service
            .get_public_key(key_id)
            .expect("Public key should be stored");

        // Both signatures should verify with the same public key
        let verified1 = service
            .verify(
                data,
                &sig1,
                VerifyOptions {
                    public_key: pub_key.clone(),
                    context: None,
                },
            )
            .await
            .expect("First verification should succeed");

        let verified2 = service
            .verify(
                data,
                &sig2,
                VerifyOptions {
                    public_key: pub_key,
                    context: None,
                },
            )
            .await
            .expect("Second verification should succeed");

        assert!(verified1, "First signature should verify");
        assert!(verified2, "Second signature should verify");
    }
}
