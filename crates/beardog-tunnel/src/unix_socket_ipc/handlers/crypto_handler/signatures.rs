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
            info!("🔑 Crypto: ed25519_generate_keypair (Songbird Onion Identity)");
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
