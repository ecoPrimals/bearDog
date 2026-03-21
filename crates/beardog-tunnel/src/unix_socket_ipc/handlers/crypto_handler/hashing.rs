// SPDX-License-Identifier: AGPL-3.0-only

//! Standalone hashing, cipher-aware hashing, and HMAC variants.

use crate::unix_socket_ipc::crypto_handlers_hashing::{
    handle_derive_onion_address, handle_sha1, handle_sha3_256, handle_sha256, handle_sha384,
    handle_sha512,
};
use crate::unix_socket_ipc::crypto_handlers_hmac::{
    handle_hmac_blake3, handle_hmac_sha384, handle_hmac_sha512,
};
use crate::unix_socket_ipc::handlers::crypto::{
    handle_blake3_hash, handle_hash_for_cipher, handle_hmac_sha256,
};
use tracing::info;

pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    match method {
        "crypto.blake3_hash" => {
            info!("🔍 Crypto: blake3_hash");
            Ok(Some(handle_blake3_hash(params).await?))
        }

        "crypto.hmac_sha256" => {
            info!("🔏 Crypto: hmac_sha256");
            Ok(Some(handle_hmac_sha256(params).await?))
        }

        "crypto.hash_for_cipher" => {
            info!("🎯 Crypto: hash_for_cipher (cipher-aware hashing for TLS 1.3)");
            Ok(Some(handle_hash_for_cipher(params).await?))
        }

        "crypto.sha256" => {
            info!("🔍 Crypto: sha256 (SHA-256 hashing)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(handle_sha256(params_ref).map_err(|e| e.to_string())?))
        }

        "crypto.sha384" => {
            info!("🔍 Crypto: sha384 (SHA-384 hashing)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(handle_sha384(params_ref).map_err(|e| e.to_string())?))
        }

        "crypto.sha512" => {
            info!("🔍 Crypto: sha512 (SHA-512 hashing)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(handle_sha512(params_ref).map_err(|e| e.to_string())?))
        }

        "crypto.sha1" => {
            info!("⚠️  Crypto: sha1 (LEGACY - Git compatibility only!)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(handle_sha1(params_ref).map_err(|e| e.to_string())?))
        }

        "crypto.sha3_256" => {
            info!("🔬 Crypto: sha3_256 (modern quantum-resistant hashing)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_sha3_256(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.hmac_sha384" => {
            info!("🔐 Crypto: hmac_sha384 (high-security MAC)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_hmac_sha384(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.hmac_sha512" => {
            info!("🔐 Crypto: hmac_sha512 (maximum-security MAC)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_hmac_sha512(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.hmac_blake3" => {
            info!("⚡ Crypto: hmac_blake3 (modern high-performance MAC)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_hmac_blake3(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "beardog.crypto.sha3_256" => {
            info!("🧅 Crypto: beardog.crypto.sha3_256 (Songbird Onion Service)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_sha3_256(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "beardog.crypto.derive_onion_address" => {
            info!("🧅 Crypto: beardog.crypto.derive_onion_address (Tor v3)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_derive_onion_address(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        _ => Ok(None),
    }
}
