// SPDX-License-Identifier: AGPL-3.0-or-later

//! AEAD (ChaCha20-Poly1305, AES-GCM) helpers for the tarpc crypto server.

use crate::tarpc_types::{
    CryptoError, CryptoResult, DecryptRequest, DecryptResponse, EncryptRequest, EncryptResponse,
};

pub(super) async fn chacha20_poly1305_encrypt(
    request: EncryptRequest,
) -> CryptoResult<EncryptResponse> {
    use chacha20poly1305::{
        ChaCha20Poly1305, Nonce,
        aead::{Aead, KeyInit},
    };
    use rand::RngCore;

    let key: [u8; 32] = request.key.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid key length (expected 32 bytes)".to_string(),
    })?;

    let cipher = ChaCha20Poly1305::new(&key.into());

    let nonce_bytes: [u8; 12] = if let Some(n) = request.nonce {
        n.try_into().map_err(|_| CryptoError {
            code: -32000,
            message: "Invalid nonce length (expected 12 bytes)".to_string(),
        })?
    } else {
        let mut n = [0u8; 12];
        rand::rng().fill_bytes(&mut n);
        n
    };

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, request.plaintext.as_ref())
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("Encryption failed: {e}"),
        })?;

    let (ct, tag) = ciphertext.split_at(ciphertext.len() - 16);

    Ok(EncryptResponse {
        ciphertext: ct.to_vec(),
        nonce: nonce_bytes.to_vec(),
        tag: tag.to_vec(),
    })
}

pub(super) async fn chacha20_poly1305_decrypt(
    request: DecryptRequest,
) -> CryptoResult<DecryptResponse> {
    use chacha20poly1305::{
        ChaCha20Poly1305, Nonce,
        aead::{Aead, KeyInit},
    };

    let key: [u8; 32] = request.key.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid key length (expected 32 bytes)".to_string(),
    })?;

    let nonce_bytes: [u8; 12] = request.nonce.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid nonce length (expected 12 bytes)".to_string(),
    })?;

    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = Nonce::from_slice(&nonce_bytes);

    let mut ciphertext_with_tag = request.ciphertext;
    ciphertext_with_tag.extend_from_slice(&request.tag);

    let plaintext = cipher
        .decrypt(nonce, ciphertext_with_tag.as_ref())
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("Decryption failed: {e}"),
        })?;

    Ok(DecryptResponse { plaintext })
}

pub(super) async fn aes256_gcm_encrypt(request: EncryptRequest) -> CryptoResult<EncryptResponse> {
    use aes_gcm::{
        Aes256Gcm, Nonce,
        aead::{Aead, KeyInit},
    };
    use rand::RngCore;

    let key: [u8; 32] = request.key.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid key length (expected 32 bytes)".to_string(),
    })?;

    let cipher = Aes256Gcm::new(&key.into());

    let nonce_bytes: [u8; 12] = if let Some(n) = request.nonce {
        n.try_into().map_err(|_| CryptoError {
            code: -32000,
            message: "Invalid nonce length (expected 12 bytes)".to_string(),
        })?
    } else {
        let mut n = [0u8; 12];
        rand::rng().fill_bytes(&mut n);
        n
    };

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, request.plaintext.as_ref())
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("Encryption failed: {e}"),
        })?;

    let (ct, tag) = ciphertext.split_at(ciphertext.len() - 16);

    Ok(EncryptResponse {
        ciphertext: ct.to_vec(),
        nonce: nonce_bytes.to_vec(),
        tag: tag.to_vec(),
    })
}

pub(super) async fn aes256_gcm_decrypt(request: DecryptRequest) -> CryptoResult<DecryptResponse> {
    use aes_gcm::{
        Aes256Gcm, Nonce,
        aead::{Aead, KeyInit},
    };

    let key: [u8; 32] = request.key.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid key length (expected 32 bytes)".to_string(),
    })?;

    let nonce_bytes: [u8; 12] = request.nonce.try_into().map_err(|_| CryptoError {
        code: -32000,
        message: "Invalid nonce length (expected 12 bytes)".to_string(),
    })?;

    let cipher = Aes256Gcm::new(&key.into());
    let nonce = Nonce::from_slice(&nonce_bytes);

    let mut ciphertext_with_tag = request.ciphertext;
    ciphertext_with_tag.extend_from_slice(&request.tag);

    let plaintext = cipher
        .decrypt(nonce, ciphertext_with_tag.as_ref())
        .map_err(|e| CryptoError {
            code: -32000,
            message: format!("Decryption failed: {e}"),
        })?;

    Ok(DecryptResponse { plaintext })
}
