// SPDX-License-Identifier: AGPL-3.0-only

//! Ed25519, ECDSA, and RSA signature routing.

use crate::unix_socket_ipc::crypto_handlers_ecdsa::{
    handle_sign_ecdsa_secp256r1, handle_sign_ecdsa_secp384r1, handle_verify_ecdsa_secp256r1,
    handle_verify_ecdsa_secp384r1,
};
use crate::unix_socket_ipc::crypto_handlers_rsa::{
    handle_sign_rsa_pkcs1_sha256, handle_sign_rsa_pss_sha256, handle_verify_rsa_pkcs1_sha256,
    handle_verify_rsa_pss_sha256,
};
use crate::unix_socket_ipc::handlers::crypto::{
    handle_ed25519_generate_keypair, handle_sign_ed25519, handle_verify_ed25519,
};
use tracing::info;

pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    match method {
        "crypto.ed25519_generate_keypair" => {
            info!("🔑 Crypto: ed25519_generate_keypair (onion identity route)");
            Ok(Some(handle_ed25519_generate_keypair(params).await?))
        }

        "crypto.sign_ed25519" => {
            info!("✍️  Crypto: sign_ed25519");
            Ok(Some(handle_sign_ed25519(params).await?))
        }

        "crypto.verify_ed25519" => {
            info!("✅ Crypto: verify_ed25519");
            Ok(Some(handle_verify_ed25519(params).await?))
        }

        "crypto.sign_ecdsa_secp256r1" => {
            info!("✍️  Crypto: sign_ecdsa_secp256r1 (ECDSA P-256 for TLS 1.3)");
            Ok(Some(handle_sign_ecdsa_secp256r1(params).await?))
        }

        "crypto.verify_ecdsa_secp256r1" => {
            info!("✅ Crypto: verify_ecdsa_secp256r1 (ECDSA P-256 for TLS 1.3)");
            Ok(Some(handle_verify_ecdsa_secp256r1(params).await?))
        }

        "crypto.sign_ecdsa_secp384r1" => {
            info!("✍️  Crypto: sign_ecdsa_secp384r1 (ECDSA P-384 for TLS 1.3)");
            Ok(Some(handle_sign_ecdsa_secp384r1(params).await?))
        }

        "crypto.verify_ecdsa_secp384r1" => {
            info!("✅ Crypto: verify_ecdsa_secp384r1 (ECDSA P-384 for TLS 1.3)");
            Ok(Some(handle_verify_ecdsa_secp384r1(params).await?))
        }

        "crypto.sign_rsa_pkcs1_sha256" => {
            info!("✍️  Crypto: sign_rsa_pkcs1_sha256 (RSA PKCS#1 v1.5 - legacy)");
            Ok(Some(handle_sign_rsa_pkcs1_sha256(params).await?))
        }

        "crypto.verify_rsa_pkcs1_sha256" => {
            info!("✅ Crypto: verify_rsa_pkcs1_sha256 (RSA PKCS#1 v1.5 - legacy)");
            Ok(Some(handle_verify_rsa_pkcs1_sha256(params).await?))
        }

        "crypto.sign_rsa_pss_sha256" => {
            info!("✍️  Crypto: sign_rsa_pss_sha256 (RSA-PSS - modern, recommended)");
            Ok(Some(handle_sign_rsa_pss_sha256(params).await?))
        }

        "crypto.verify_rsa_pss_sha256" => {
            info!("✅ Crypto: verify_rsa_pss_sha256 (RSA-PSS - modern, recommended)");
            Ok(Some(handle_verify_rsa_pss_sha256(params).await?))
        }

        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::route;
    use crate::unix_socket_ipc::handlers::crypto::utils::derive_key_from_id_for_tests;
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use beardog_core::crypto_service::algorithms::asymmetric;
    use serde_json::json;

    #[tokio::test]
    async fn route_ed25519_sign_verify_roundtrip() {
        let key_id = "signatures-route-ed25519";
        let purpose = "general";
        let msg = BASE64.encode(b"sign this");
        let sign_p = json!({ "message": msg, "key_id": key_id, "purpose": purpose });
        let sig = route("crypto.sign_ed25519", Some(&sign_p))
            .await
            .expect("route")
            .expect("sign");
        let sig_b64 = sig
            .get("signature")
            .and_then(|x| x.as_str())
            .expect("signature");
        let seed =
            derive_key_from_id_for_tests(key_id, purpose).expect("derive_key_from_id_for_tests");
        let (_sk, pk) =
            asymmetric::generate_ed25519_from_seed(&seed).expect("generate_ed25519_from_seed");
        let verify_p = json!({
            "public_key": BASE64.encode(pk),
            "message": msg,
            "signature": sig_b64,
        });
        let ok = route("crypto.verify_ed25519", Some(&verify_p))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(ok.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_ecdsa_p256_sign_verify() {
        let data_b64 = BASE64.encode(b"ecdsa payload");
        let sign_p = json!({ "data": data_b64 });
        let signed = route("crypto.sign_ecdsa_secp256r1", Some(&sign_p))
            .await
            .expect("route")
            .expect("signed");
        let pk = signed
            .get("public_key")
            .and_then(|x| x.as_str())
            .expect("pk");
        let sig = signed
            .get("signature")
            .and_then(|x| x.as_str())
            .expect("sig");
        let verify_p = json!({
            "data": data_b64,
            "signature": sig,
            "public_key": pk,
        });
        let v = route("crypto.verify_ecdsa_secp256r1", Some(&verify_p))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_ecdsa_p384_sign_verify() {
        let data_b64 = BASE64.encode(b"p384 data");
        let sign_p = json!({ "data": data_b64 });
        let signed = route("crypto.sign_ecdsa_secp384r1", Some(&sign_p))
            .await
            .expect("route")
            .expect("signed");
        let pk = signed
            .get("public_key")
            .and_then(|x| x.as_str())
            .expect("pk");
        let sig = signed
            .get("signature")
            .and_then(|x| x.as_str())
            .expect("sig");
        let verify_p = json!({
            "data": data_b64,
            "signature": sig,
            "public_key": pk,
        });
        let v = route("crypto.verify_ecdsa_secp384r1", Some(&verify_p))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_rsa_pss_sign_verify() {
        let data_b64 = BASE64.encode(b"rsa pss");
        let sign_p = json!({ "data": data_b64, "key_size": 2048 });
        let signed = route("crypto.sign_rsa_pss_sha256", Some(&sign_p))
            .await
            .expect("route")
            .expect("signed");
        let pem = signed
            .get("public_key_pem")
            .and_then(|x| x.as_str())
            .expect("pem");
        let sig = signed
            .get("signature")
            .and_then(|x| x.as_str())
            .expect("sig");
        let verify_p = json!({
            "data": data_b64,
            "signature": sig,
            "public_key_pem": pem,
        });
        let v = route("crypto.verify_rsa_pss_sha256", Some(&verify_p))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_rsa_pkcs1_sign_verify() {
        let data_b64 = BASE64.encode(b"rsa pkcs1");
        let sign_p = json!({ "data": data_b64, "key_size": 2048 });
        let signed = route("crypto.sign_rsa_pkcs1_sha256", Some(&sign_p))
            .await
            .expect("route")
            .expect("signed");
        let pem = signed
            .get("public_key_pem")
            .and_then(|x| x.as_str())
            .expect("pem");
        let sig = signed
            .get("signature")
            .and_then(|x| x.as_str())
            .expect("sig");
        let verify_p = json!({
            "data": data_b64,
            "signature": sig,
            "public_key_pem": pem,
        });
        let v = route("crypto.verify_rsa_pkcs1_sha256", Some(&verify_p))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_unknown_method_returns_none() {
        assert!(
            route("crypto.no_signature_here", None)
                .await
                .expect("route")
                .is_none()
        );
    }
}
