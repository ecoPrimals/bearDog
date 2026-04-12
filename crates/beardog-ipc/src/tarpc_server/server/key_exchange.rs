// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key exchange (ECDH) helpers for the tarpc crypto server.

use crate::tarpc_types::{CryptoError, CryptoResult, KeyExchangeRequest, SharedSecret};

pub(super) async fn x25519_key_exchange(request: KeyExchangeRequest) -> CryptoResult<SharedSecret> {
    use x25519_dalek::{PublicKey, StaticSecret};

    let our_secret_bytes: [u8; 32] =
        request
            .our_private_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid X25519 private key length".to_string(),
            })?;

    let their_public_bytes: [u8; 32] =
        request
            .their_public_key
            .try_into()
            .map_err(|_| CryptoError {
                code: -32000,
                message: "Invalid X25519 public key length".to_string(),
            })?;

    let our_secret = StaticSecret::from(our_secret_bytes);
    let their_public = PublicKey::from(their_public_bytes);
    let shared = our_secret.diffie_hellman(&their_public);

    Ok(SharedSecret {
        secret: shared.as_bytes().to_vec(),
    })
}

pub(super) async fn ecdh_p256_key_exchange(
    request: KeyExchangeRequest,
) -> CryptoResult<SharedSecret> {
    use p256::{PublicKey, SecretKey, ecdh::diffie_hellman};

    let key_bytes: [u8; 32] = request
        .our_private_key
        .try_into()
        .map_err(|_| CryptoError {
            code: -32000,
            message: "Invalid P-256 private key length (expected 32 bytes)".to_string(),
        })?;

    let our_secret = SecretKey::from_bytes(&key_bytes.into()).map_err(|e| CryptoError {
        code: -32000,
        message: format!("Invalid P-256 private key: {e}"),
    })?;

    let their_public =
        PublicKey::from_sec1_bytes(&request.their_public_key).map_err(|e| CryptoError {
            code: -32000,
            message: format!("Invalid P-256 public key: {e}"),
        })?;

    let shared = diffie_hellman(our_secret.to_nonzero_scalar(), their_public.as_affine());

    Ok(SharedSecret {
        secret: shared.raw_secret_bytes().to_vec(),
    })
}
