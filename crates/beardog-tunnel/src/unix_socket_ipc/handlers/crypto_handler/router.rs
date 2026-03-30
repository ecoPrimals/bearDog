// SPDX-License-Identifier: AGPL-3.0-only

//! Dispatches JSON-RPC crypto methods to domain-specific route modules.

use crate::btsp_provider::BeardogBtspProvider;
use std::sync::Arc;

use super::{
    aliases_and_beardog, genetic, hashing, kex_aead, password_kdf, signatures, tls_ops, tls12_dot,
};

/// # Errors
///
/// Returns an error if hashing fails.
pub async fn dispatch(
    method: &str,
    params: Option<&serde_json::Value>,
    _btsp_provider: &Arc<BeardogBtspProvider>,
) -> Result<serde_json::Value, String> {
    if let Some(v) = signatures::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = kex_aead::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = hashing::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = password_kdf::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = tls_ops::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = tls12_dot::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = genetic::route(method, params).await? {
        return Ok(v);
    }
    if let Some(v) = aliases_and_beardog::route(method, params).await? {
        return Ok(v);
    }

    Err(format!("Unknown crypto method: {method}"))
}

#[cfg(test)]
mod tests {
    use super::dispatch;
    use crate::btsp_provider::BeardogBtspProvider;
    use crate::tunnel::hsm::HsmManager;
    use crate::unix_socket_ipc::handlers::crypto::utils::derive_key_from_id_for_tests;
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use beardog_core::crypto_service::algorithms::asymmetric;
    use beardog_genetics::ecosystem_evolution::engine::EcosystemGeneticEngine;
    use serde_json::json;
    use std::sync::Arc;

    async fn test_btsp() -> Arc<BeardogBtspProvider> {
        let hsm = Arc::new(HsmManager::new());
        let genetics =
            Arc::new(EcosystemGeneticEngine::new().expect("EcosystemGeneticEngine::new"));
        Arc::new(
            BeardogBtspProvider::new_for_testing(hsm, genetics)
                .await
                .expect("BeardogBtspProvider::new_for_testing"),
        )
    }

    #[tokio::test]
    async fn dispatch_routes_crypto_hash() {
        let btsp = test_btsp().await;
        let params = json!({
            "data": BASE64.encode(b"router-hash-test"),
        });
        let out = dispatch("crypto.hash", Some(&params), &btsp)
            .await
            .expect("dispatch crypto.hash");
        assert_eq!(
            out.get("algorithm").and_then(|x| x.as_str()),
            Some("BLAKE3")
        );
    }

    #[tokio::test]
    async fn dispatch_routes_password_kdf() {
        let btsp = test_btsp().await;
        let params = json!({ "password": "dispatch-pbkdf2", "salt": BASE64.encode(b"salt123456789012"), "iterations": 100000, "output_length": 32 });
        let out = dispatch("crypto.pbkdf2_sha256", Some(&params), &btsp)
            .await
            .expect("dispatch pbkdf2");
        assert!(out.get("derived_key").is_some());
    }

    #[tokio::test]
    async fn dispatch_routes_signatures_ed25519() {
        let btsp = test_btsp().await;
        let key_id = "router-dispatch-ed25519";
        let purpose = "general";
        let msg = BASE64.encode(b"router");
        let sign_p = json!({ "message": msg, "key_id": key_id, "purpose": purpose });
        let sig = dispatch("crypto.sign_ed25519", Some(&sign_p), &btsp)
            .await
            .expect("sign");
        let seed =
            derive_key_from_id_for_tests(key_id, purpose).expect("derive_key_from_id_for_tests");
        let (_sk, pk) =
            asymmetric::generate_ed25519_from_seed(&seed).expect("generate_ed25519_from_seed");
        let verify_p = json!({
            "public_key": BASE64.encode(pk),
            "message": msg,
            "signature": sig.get("signature").and_then(|x| x.as_str()).expect("sig"),
        });
        let v = dispatch("crypto.verify_ed25519", Some(&verify_p), &btsp)
            .await
            .expect("verify");
        assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn dispatch_unknown_method_errors() {
        let btsp = test_btsp().await;
        let err = dispatch("crypto.__not_registered__", None, &btsp)
            .await
            .expect_err("unknown method");
        assert!(err.contains("Unknown crypto method"));
    }

    #[tokio::test]
    async fn dispatch_crypto_derive_onion_address_semantic() {
        let btsp = test_btsp().await;
        let params = json!({
            "public_key": BASE64.encode([0u8; 32]),
        });
        let out = dispatch("crypto.derive_onion_address", Some(&params), &btsp)
            .await
            .expect("derive_onion_address");
        assert!(
            out.get("onion_address")
                .and_then(|v| v.as_str())
                .is_some_and(|s| s.ends_with(".onion"))
        );
    }
}
