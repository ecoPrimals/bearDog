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
    handle_blake3_hash, handle_hash_for_cipher, handle_hkdf_sha256, handle_hmac_sha256,
    handle_hmac_verify,
};
use tracing::info;

/// # Errors
///
/// Returns an error if hashing fails.
pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, super::super::HandlerError> {
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

        "crypto.hmac_verify" => {
            info!("🔐 Crypto: hmac_verify (constant-time HMAC verification)");
            Ok(Some(handle_hmac_verify(params).await?))
        }

        "crypto.hkdf_sha256" => {
            info!("🔑 Crypto: hkdf_sha256 (HKDF-SHA256 key derivation)");
            Ok(Some(handle_hkdf_sha256(params).await?))
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

    #[tokio::test]
    async fn hashing_route_hmac_verify_valid_mac() {
        let key = b64(b"my-secret-key");
        let data = b64(b"hello world");
        let compute_params = json!({ "key": key, "data": data });
        let mac_result = route("crypto.hmac_sha256", Some(&compute_params))
            .await
            .expect("route")
            .expect("hmac compute");
        let mac = mac_result["mac"].as_str().expect("mac field");

        let verify_params = json!({ "key": key, "data": data, "mac": mac });
        let out = route("crypto.hmac_verify", Some(&verify_params))
            .await
            .expect("route")
            .expect("hmac verify");
        assert_eq!(out["valid"], true);
        assert_eq!(out["algorithm"], "HMAC-SHA256");
    }

    #[tokio::test]
    async fn hashing_route_hmac_verify_rejects_wrong_mac() {
        let key = b64(b"my-secret-key");
        let data = b64(b"hello world");
        let wrong_mac = b64(&[0u8; 32]);
        let params = json!({ "key": key, "data": data, "mac": wrong_mac });
        let out = route("crypto.hmac_verify", Some(&params))
            .await
            .expect("route")
            .expect("hmac verify");
        assert_eq!(out["valid"], false);
    }

    #[tokio::test]
    async fn hashing_route_hmac_verify_missing_mac_errors() {
        let params = json!({ "key": b64(b"k"), "data": b64(b"d") });
        let err = route("crypto.hmac_verify", Some(&params))
            .await
            .expect_err("missing mac");
        assert!(err.contains("mac"));
    }

    #[tokio::test]
    async fn hashing_route_hkdf_sha256_basic() {
        let ikm = b64(b"input key material");
        let params = json!({ "ikm": ikm });
        let out = route("crypto.hkdf_sha256", Some(&params))
            .await
            .expect("route")
            .expect("hkdf");
        assert_eq!(out["algorithm"], "HKDF-SHA256");
        assert_eq!(out["length"], 32);
        let okm = out["okm"].as_str().expect("okm field");
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(okm)
            .expect("valid base64");
        assert_eq!(decoded.len(), 32);
    }

    #[tokio::test]
    async fn hashing_route_hkdf_sha256_with_salt_and_info() {
        let ikm = b64(b"shared secret");
        let salt = b64(b"random salt");
        let info = b64(b"btsp-v1-phase3");
        let params = json!({ "ikm": ikm, "salt": salt, "info": info, "length": 64 });
        let out = route("crypto.hkdf_sha256", Some(&params))
            .await
            .expect("route")
            .expect("hkdf");
        assert_eq!(out["length"], 64);
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(out["okm"].as_str().expect("okm"))
            .expect("valid base64");
        assert_eq!(decoded.len(), 64);
    }

    #[tokio::test]
    async fn hashing_route_hkdf_sha256_deterministic() {
        let params = json!({ "ikm": b64(b"key"), "salt": b64(b"s"), "info": b64(b"i") });
        let out1 = route("crypto.hkdf_sha256", Some(&params))
            .await
            .expect("r")
            .expect("v");
        let out2 = route("crypto.hkdf_sha256", Some(&params))
            .await
            .expect("r")
            .expect("v");
        assert_eq!(out1["okm"], out2["okm"]);
    }

    #[tokio::test]
    async fn hashing_route_hkdf_sha256_missing_ikm_errors() {
        let params = json!({ "salt": b64(b"s") });
        let err = route("crypto.hkdf_sha256", Some(&params))
            .await
            .expect_err("missing ikm");
        assert!(err.contains("ikm"));
    }
}
