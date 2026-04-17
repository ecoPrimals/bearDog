// SPDX-License-Identifier: AGPL-3.0-or-later
//! Signature tests (ECDSA P-256).

#![cfg(test)]

use super::*;

fn create_test_keypair_p256() -> (Vec<u8>, Vec<u8>) {
    use p256::SecretKey;
    use p256::ecdsa::SigningKey;

    let secret_key = SecretKey::from_bytes((&[0x42; 32]).into()).unwrap();
    let signing_key = SigningKey::from(secret_key);
    let verifying_key = signing_key.verifying_key();

    (
        signing_key.to_bytes().to_vec(),
        verifying_key.to_sec1_bytes().to_vec(),
    )
}

#[tokio::test]
async fn test_ecdsa_p256_sign_verify_round_trip() {
    let provider = RustCryptoProvider::new();
    let (private_key, public_key) = create_test_keypair_p256();
    let message = b"ECDSA P-256 test message";

    let alg = SignatureAlgorithm::EcdsaP256 {
        hash: HashAlgorithm::Sha256,
    };
    let sign_opts = SigningOptions::default();

    // Sign
    let signature = provider
        .sign(alg.clone(), &private_key, message, &sign_opts)
        .await
        .unwrap();
    assert_eq!(signature.algorithm, "ECDSA-P256-SHA256");

    // Verify
    let verify_opts = VerificationOptions::default();
    let valid = provider
        .verify(alg, &public_key, message, &signature, &verify_opts)
        .await
        .unwrap();
    assert!(valid);
}

#[tokio::test]
async fn test_ecdsa_p256_verify_invalid_signature() {
    let provider = RustCryptoProvider::new();
    let (_, public_key) = create_test_keypair_p256();
    let message = b"Test";

    let invalid_signature = Signature {
        algorithm: "ECDSA-P256-SHA256".to_string(),
        signature: vec![0x00; 64],
    };

    let alg = SignatureAlgorithm::EcdsaP256 {
        hash: HashAlgorithm::Sha256,
    };
    let opts = VerificationOptions::default();
    // Invalid signature format causes an error, not Ok(false)
    let result = provider
        .verify(alg, &public_key, message, &invalid_signature, &opts)
        .await;
    assert!(result.is_err());
}
