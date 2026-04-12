// SPDX-License-Identifier: AGPL-3.0-or-later

//! Hashing and HMAC helpers for the tarpc crypto server.

use crate::tarpc_types::{CryptoError, CryptoResult, HashResponse, HmacRequest};

pub(super) async fn blake3_hash(data: Vec<u8>) -> CryptoResult<HashResponse> {
    let hash = blake3::hash(&data);
    Ok(HashResponse {
        hash: hash.as_bytes().to_vec(),
    })
}

pub(super) async fn sha256_hash(data: Vec<u8>) -> CryptoResult<HashResponse> {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    hasher.update(&data);
    let result = hasher.finalize();

    Ok(HashResponse {
        hash: result.to_vec(),
    })
}

pub(super) async fn hmac_sha256(request: HmacRequest) -> CryptoResult<HashResponse> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(&request.key).map_err(|e| CryptoError {
        code: -32000,
        message: format!("Invalid HMAC key: {e}"),
    })?;

    mac.update(&request.data);
    let result = mac.finalize();

    Ok(HashResponse {
        hash: result.into_bytes().to_vec(),
    })
}
