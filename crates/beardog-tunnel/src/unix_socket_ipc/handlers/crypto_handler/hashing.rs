// SPDX-License-Identifier: AGPL-3.0-or-later
#![cfg_attr(test, allow(clippy::expect_used, clippy::unwrap_used))]

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

/// # Errors
///
/// Returns an error if hashing fails.
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

        "crypto.derive_onion_address" => {
            info!("🧅 Crypto: derive_onion_address (Tor v3, semantic primary)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_derive_onion_address(params_ref).map_err(|e| e.to_string())?,
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
            info!("🧅 Crypto: beardog.crypto.sha3_256 (backward-compat alias)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_sha3_256(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "beardog.crypto.derive_onion_address" => {
            info!("🧅 Crypto: beardog.crypto.derive_onion_address (backward-compat alias)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_derive_onion_address(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::route;
    use base64::Engine;
    use serde_json::json;

    fn b64(data: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(data)
    }

    #[tokio::test]
    async fn hashing_route_unknown_method_returns_none() {
        assert!(
            route("crypto.unknown_hash", None)
                .await
                .expect("route")
                .is_none()
        );
    }

    #[tokio::test]
    async fn hashing_route_sha256_requires_params() {
        let err = route("crypto.sha256", None)
            .await
            .expect_err("missing params");
        assert!(err.contains("Missing parameters") || err.contains("Missing"));
    }

    #[tokio::test]
    async fn hashing_route_sha256_success() {
        let params = json!({ "data": b64(b"abc") });
        let out = route("crypto.sha256", Some(&params))
            .await
            .expect("route")
            .expect("some");
        assert_eq!(out["algorithm"], "sha256");
        assert_eq!(out["output_bits"], 256);
    }

    #[tokio::test]
    async fn hashing_route_sha384_and_sha512() {
        let p = json!({ "data": b64(b"x") });
        let o384 = route("crypto.sha384", Some(&p))
            .await
            .expect("r")
            .expect("v");
        assert_eq!(o384["algorithm"], "sha384");
        let o512 = route("crypto.sha512", Some(&p))
            .await
            .expect("r")
            .expect("v");
        assert_eq!(o512["algorithm"], "sha512");
    }

    #[tokio::test]
    async fn hashing_route_sha1_and_sha3_256() {
        let p = json!({ "data": b64(b"legacy") });
        let o1 = route("crypto.sha1", Some(&p)).await.expect("r").expect("v");
        assert_eq!(o1["algorithm"], "sha1");
        let o3 = route("crypto.sha3_256", Some(&p))
            .await
            .expect("r")
            .expect("v");
        assert_eq!(o3["algorithm"], "sha3_256");
    }

    #[tokio::test]
    async fn hashing_route_blake3_and_hash_for_cipher() {
        let p_blake = json!({ "data": b64(b"msg") });
        let blake = route("crypto.blake3_hash", Some(&p_blake))
            .await
            .expect("r")
            .expect("v");
        assert_eq!(blake["algorithm"], "BLAKE3");

        let p_cipher = json!({ "data": b64(b"TLS"), "cipher_suite": 0x1301u64 });
        let hc = route("crypto.hash_for_cipher", Some(&p_cipher))
            .await
            .expect("r")
            .expect("v");
        assert_eq!(hc["cipher_suite"], 0x1301);
    }

    #[tokio::test]
    async fn hashing_route_hmac_sha256_requires_key() {
        let p = json!({ "data": b64(b"d") });
        let err = route("crypto.hmac_sha256", Some(&p))
            .await
            .expect_err("bad hmac params");
        assert!(!err.is_empty());
    }

    #[tokio::test]
    async fn hashing_route_beardog_aliases_match_sha3_and_onion() {
        let p = json!({ "data": b64(b"onion") });
        let a = route("beardog.crypto.sha3_256", Some(&p))
            .await
            .expect("r")
            .expect("v");
        let b = route("crypto.sha3_256", Some(&p))
            .await
            .expect("r")
            .expect("v");
        assert_eq!(a["hash"], b["hash"]);

        let onion = json!({ "public_key": b64(&[0u8; 32]) });
        let derived_sem = route("crypto.derive_onion_address", Some(&onion))
            .await
            .expect("r")
            .expect("v");
        let derived_bd = route("beardog.crypto.derive_onion_address", Some(&onion))
            .await
            .expect("r")
            .expect("v");
        assert_eq!(
            derived_sem.get("onion_address"),
            derived_bd.get("onion_address")
        );
        assert!(
            derived_sem
                .get("onion_address")
                .and_then(|v| v.as_str())
                .is_some_and(|s| s.ends_with(".onion"))
        );
    }
}
