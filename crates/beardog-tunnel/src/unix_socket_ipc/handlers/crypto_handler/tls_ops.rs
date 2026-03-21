// SPDX-License-Identifier: AGPL-3.0-only

//! TLS 1.3-style handshake and certificate operations.

use crate::unix_socket_ipc::handlers::crypto::{
    handle_tls_compute_finished_verify_data, handle_tls_derive_application_secrets,
    handle_tls_derive_handshake_secrets, handle_tls_derive_secrets, handle_tls_sign_handshake,
    handle_tls_verify_certificate,
};
use tracing::info;

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

        "tls.verify_certificate" => {
            info!("🔍 TLS: verify_certificate (X.509 chain verification)");
            Ok(Some(handle_tls_verify_certificate(params).await?))
        }

        _ => Ok(None),
    }
}
