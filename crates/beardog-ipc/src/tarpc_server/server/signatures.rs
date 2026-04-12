// SPDX-License-Identifier: AGPL-3.0-or-later

//! Signature helpers for the tarpc crypto server.

use crate::tarpc_types::{CryptoError, CryptoResult, SignRequest, SignResponse, VerifyRequest};

pub(super) async fn sign_ed25519(request: SignRequest) -> CryptoResult<SignResponse> {
    use ed25519_dalek::{Signature, Signer, SigningKey};

    let key_bytes: [u8; 32] = request.private_key.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid Ed25519 private key length".to_string(),
    })?;

    let signing_key = SigningKey::from_bytes(&key_bytes);
    let signature: Signature = signing_key.sign(&request.data);

    Ok(SignResponse {
        signature: signature.to_bytes().to_vec(),
    })
}

pub(super) async fn verify_ed25519(request: VerifyRequest) -> CryptoResult<bool> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};

    let key_bytes: [u8; 32] = request.public_key.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid Ed25519 public key length".to_string(),
    })?;

    let sig_bytes: [u8; 64] = request.signature.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid Ed25519 signature length".to_string(),
    })?;

    let verifying_key = VerifyingKey::from_bytes(&key_bytes).map_err(|e| CryptoError {
        code: -32000,
        message: format!("Invalid public key: {e}"),
    })?;

    let signature = Signature::from_bytes(&sig_bytes);
    Ok(verifying_key.verify(&request.data, &signature).is_ok())
}

pub(super) async fn sign_ecdsa_p256(request: SignRequest) -> CryptoResult<SignResponse> {
    use p256::ecdsa::{Signature, SigningKey, signature::Signer};

    let key_bytes: [u8; 32] = request.private_key.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid P-256 private key length (expected 32 bytes)".to_string(),
    })?;

    let signing_key = SigningKey::from_bytes(&key_bytes.into()).map_err(|e| CryptoError {
        code: -32000,
        message: format!("Invalid P-256 private key: {e}"),
    })?;

    let signature: Signature = signing_key.sign(&request.data);

    Ok(SignResponse {
        signature: signature.to_der().as_bytes().to_vec(),
    })
}

pub(super) async fn sign_ecdsa_p384(request: SignRequest) -> CryptoResult<SignResponse> {
    use p384::ecdsa::{Signature, SigningKey, signature::Signer};

    let key_bytes: [u8; 48] = request.private_key.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid P-384 private key length (expected 48 bytes)".to_string(),
    })?;

    let signing_key = SigningKey::from_bytes(&key_bytes.into()).map_err(|e| CryptoError {
        code: -32000,
        message: format!("Invalid P-384 private key: {e}"),
    })?;

    let signature: Signature = signing_key.sign(&request.data);

    Ok(SignResponse {
        signature: signature.to_der().as_bytes().to_vec(),
    })
}
