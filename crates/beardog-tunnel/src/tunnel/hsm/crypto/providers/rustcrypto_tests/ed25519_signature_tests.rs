// SPDX-License-Identifier: AGPL-3.0-or-later
//! Signature tests (Ed25519).

#![cfg(test)]

use super::*;

fn create_test_keypair_ed25519() -> (Vec<u8>, Vec<u8>) {
    use ed25519_dalek::SigningKey;
    // Create a signing key from seed bytes
    let mut seed = [0u8; 32];
    seed[0] = 0x42;
    seed[31] = 0x24; // Add some variation to make valid key
    let signing_key = SigningKey::from_bytes(&seed);
    let verifying_key = signing_key.verifying_key();
    (
        signing_key.to_bytes().to_vec(),
        verifying_key.to_bytes().to_vec(),
    )
}

#[tokio::test]
async fn test_ed25519_sign_verify_round_trip() {
    let provider = RustCryptoProvider::new();
    let (private_key, _public_key) = create_test_keypair_ed25519();
    let message = b"Test message for Ed25519";

    let alg = SignatureAlgorithm::Ed25519;
    let sign_opts = SigningOptions::default();

    // Sign
    let signature = provider
        .sign(alg.clone(), &private_key, message, &sign_opts)
        .await
        .unwrap();
    assert_eq!(signature.algorithm, "Ed25519");
    assert_eq!(signature.signature.len(), 64);

    // Verify with private key (should derive public key automatically)
    let verify_opts = VerificationOptions::default();
    let valid = provider
        .verify(alg, &private_key, message, &signature, &verify_opts)
        .await
        .unwrap();
    assert!(valid);
}

#[tokio::test]
async fn test_ed25519_verify_invalid_signature() {
    let provider = RustCryptoProvider::new();
    let (_, public_key) = create_test_keypair_ed25519();
    let message = b"Test message";

    let invalid_signature = Signature {
        algorithm: "Ed25519".to_string(),
        signature: vec![0x00; 64], // Invalid signature
    };

    let alg = SignatureAlgorithm::Ed25519;
    let opts = VerificationOptions::default();
    let valid = provider
        .verify(alg, &public_key, message, &invalid_signature, &opts)
        .await
        .unwrap();
    assert!(!valid);
}

#[tokio::test]
async fn test_ed25519_verify_wrong_message() {
    let provider = RustCryptoProvider::new();
    let (private_key, public_key) = create_test_keypair_ed25519();
    let message1 = b"Original message";
    let message2 = b"Different message";

    let alg = SignatureAlgorithm::Ed25519;
    let sign_opts = SigningOptions::default();
    let signature = provider
        .sign(alg.clone(), &private_key, message1, &sign_opts)
        .await
        .unwrap();

    let verify_opts = VerificationOptions::default();
    let valid = provider
        .verify(alg, &public_key, message2, &signature, &verify_opts)
        .await
        .unwrap();
    assert!(!valid);
}

#[tokio::test]
async fn test_ed25519_sign_invalid_key_length() {
    let provider = RustCryptoProvider::new();
    let bad_key = vec![0x42; 16]; // Wrong size (should be 32)
    let message = b"Test";

    let alg = SignatureAlgorithm::Ed25519;
    let opts = SigningOptions::default();
    let result = provider.sign(alg, &bad_key, message, &opts).await;
    assert!(result.is_err());
}
