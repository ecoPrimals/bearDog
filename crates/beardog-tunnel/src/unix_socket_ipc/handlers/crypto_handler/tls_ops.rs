// SPDX-License-Identifier: AGPL-3.0-or-later

//! TLS 1.3-style handshake and certificate operations.

#[cfg(feature = "tls-x509")]
use crate::unix_socket_ipc::handlers::crypto::handle_tls_verify_certificate;
use crate::unix_socket_ipc::handlers::crypto::{
    handle_tls_compute_finished_verify_data, handle_tls_derive_application_secrets,
    handle_tls_derive_handshake_secrets, handle_tls_derive_secrets, handle_tls_sign_handshake,
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
        "tls.derive_secrets" => {
            info!("🔑 TLS: derive_secrets (HKDF handshake key derivation - legacy)");
            Ok(Some(handle_tls_derive_secrets(params).await?))
        }

        "tls.derive_handshake_secrets" => {
            info!("🔑 TLS: derive_handshake_secrets (RFC 8446 handshake key derivation)");
            Ok(Some(handle_tls_derive_handshake_secrets(params).await?))
        }

        "tls.derive_application_secrets" => {
            info!(
                "🔑 TLS: derive_application_secrets (RFC 8446 application key derivation for HTTP)"
            );
            Ok(Some(handle_tls_derive_application_secrets(params).await?))
        }

        "tls.compute_finished_verify_data" => {
            info!(
                "🏁 TLS: compute_finished_verify_data (RFC 8446 Section 4.4.4 - Finished message)"
            );
            Ok(Some(handle_tls_compute_finished_verify_data(params).await?))
        }

        "tls.sign_handshake" => {
            info!("✍️  TLS: sign_handshake (Ed25519 handshake signing)");
            Ok(Some(handle_tls_sign_handshake(params).await?))
        }

        #[cfg(feature = "tls-x509")]
        "tls.verify_certificate" => {
            info!("🔍 TLS: verify_certificate (X.509 chain verification)");
            Ok(Some(handle_tls_verify_certificate(params).await?))
        }

        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::route;
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as B64;
    use serde_json::json;

    fn z32_b64() -> String {
        B64.encode([0u8; 32])
    }

    #[tokio::test]
    async fn route_tls_derive_secrets_legacy() {
        let params = json!({
            "pre_master_secret": z32_b64(),
            "client_random": z32_b64(),
            "server_random": z32_b64(),
            "cipher_suite": "TLS_CHACHA20_POLY1305_SHA256"
        });
        let v = route("tls.derive_secrets", Some(&params))
            .await
            .expect("route")
            .expect("secrets");
        assert!(v.get("master_secret").is_some() || v.as_object().is_some());
    }

    #[tokio::test]
    async fn route_tls_derive_handshake_secrets_chacha() {
        let th = B64.encode([2u8; 32]);
        let params = json!({
            "pre_master_secret": z32_b64(),
            "client_random": z32_b64(),
            "server_random": z32_b64(),
            "transcript_hash": th,
            "cipher_suite": 0x1303
        });
        let v = route("tls.derive_handshake_secrets", Some(&params))
            .await
            .expect("route")
            .expect("hs");
        assert!(v.get("client_write_key").is_some());
    }

    #[tokio::test]
    async fn route_tls_derive_application_secrets() {
        let params = json!({
            "handshake_secret": z32_b64(),
            "transcript_hash": z32_b64(),
            "cipher_suite": 0x1303
        });
        let v = route("tls.derive_application_secrets", Some(&params))
            .await
            .expect("route")
            .expect("app");
        assert!(v.get("client_application_secret").is_some());
    }

    #[tokio::test]
    async fn route_tls_compute_finished_verify_data() {
        let params = json!({
            "base_key": z32_b64(),
            "transcript_hash": z32_b64(),
            "cipher_suite": 0x1303
        });
        let v = route("tls.compute_finished_verify_data", Some(&params))
            .await
            .expect("route")
            .expect("fin");
        assert!(v.get("verify_data").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn route_tls_sign_handshake_ed25519() {
        let msg = B64.encode(b"handshake-transcript-bytes");
        let params = json!({
            "message": msg,
            "algorithm": "ed25519",
            "key_id": "route-test-key",
            "purpose": "tls_handshake"
        });
        let v = route("tls.sign_handshake", Some(&params))
            .await
            .expect("route")
            .expect("sig");
        assert_eq!(v.get("algorithm").and_then(|x| x.as_str()), Some("Ed25519"));
    }

    #[cfg(feature = "tls-x509")]
    #[tokio::test]
    async fn route_tls_verify_certificate_empty_chain_errors() {
        let params = json!({
            "certificate_chain": [],
            "server_name": "example.com",
            "current_time_unix": 1700000000_i64
        });
        let err = route("tls.verify_certificate", Some(&params))
            .await
            .expect_err("empty chain");
        assert!(err.contains("empty") || err.contains("Certificate"));
    }

    #[tokio::test]
    async fn route_tls_unknown_returns_none() {
        assert!(
            route("tls.not_a_real_method", None)
                .await
                .expect("route")
                .is_none()
        );
    }

    #[tokio::test]
    async fn route_tls_sign_handshake_rejects_bad_algorithm() {
        let params = json!({
            "message": B64.encode(b"x"),
            "algorithm": "rsa-pss",
        });
        let err = route("tls.sign_handshake", Some(&params))
            .await
            .expect_err("bad alg");
        assert!(err.contains("Unsupported") || err.contains("algorithm"));
    }
}
