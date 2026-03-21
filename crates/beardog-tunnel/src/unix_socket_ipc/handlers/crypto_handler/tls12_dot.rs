// SPDX-License-Identifier: AGPL-3.0-only

//! TLS 1.2 semantic method names (ECDHE, AES-GCM, PRF) for Songbird integration.

use crate::unix_socket_ipc::handlers::crypto::{
    handle_aes_128_gcm_decrypt, handle_aes_128_gcm_encrypt, handle_aes_256_gcm_decrypt,
    handle_aes_256_gcm_encrypt, handle_ecdhe_p256_compute_shared, handle_ecdhe_p256_generate,
    handle_ecdhe_p384_compute_shared, handle_ecdhe_p384_generate, handle_tls12_prf,
};
use tracing::info;

pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    match method {
        "crypto.ecdhe.p256.generate" => {
            info!("🔑 Crypto: ecdhe.p256.generate (TLS 1.2 P-256 keypair for Songbird)");
            Ok(Some(handle_ecdhe_p256_generate(params).await?))
        }

        "crypto.ecdhe.p256.compute_shared" => {
            info!("🤝 Crypto: ecdhe.p256.compute_shared (TLS 1.2 P-256 ECDH for Songbird)");
            Ok(Some(handle_ecdhe_p256_compute_shared(params).await?))
        }

        "crypto.ecdhe.p384.generate" => {
            info!("🔑 Crypto: ecdhe.p384.generate (TLS 1.2 P-384 keypair for Songbird)");
            Ok(Some(handle_ecdhe_p384_generate(params).await?))
        }

        "crypto.ecdhe.p384.compute_shared" => {
            info!("🤝 Crypto: ecdhe.p384.compute_shared (TLS 1.2 P-384 ECDH for Songbird)");
            Ok(Some(handle_ecdhe_p384_compute_shared(params).await?))
        }

        "crypto.aead.aes_128_gcm.encrypt" => {
            info!("🔒 Crypto: aead.aes_128_gcm.encrypt (TLS 1.2 AES-128-GCM for Songbird)");
            Ok(Some(handle_aes_128_gcm_encrypt(params).await?))
        }

        "crypto.aead.aes_128_gcm.decrypt" => {
            info!("🔓 Crypto: aead.aes_128_gcm.decrypt (TLS 1.2 AES-128-GCM for Songbird)");
            Ok(Some(handle_aes_128_gcm_decrypt(params).await?))
        }

        "crypto.aead.aes_256_gcm.encrypt" => {
            info!("🔒 Crypto: aead.aes_256_gcm.encrypt (TLS 1.2 AES-256-GCM for Songbird)");
            Ok(Some(handle_aes_256_gcm_encrypt(params).await?))
        }

        "crypto.aead.aes_256_gcm.decrypt" => {
            info!("🔓 Crypto: aead.aes_256_gcm.decrypt (TLS 1.2 AES-256-GCM for Songbird)");
            Ok(Some(handle_aes_256_gcm_decrypt(params).await?))
        }

        "crypto.kdf.tls12_prf" => {
            info!("🔑 Crypto: kdf.tls12_prf (TLS 1.2 PRF key expansion for Songbird)");
            Ok(Some(handle_tls12_prf(params).await?))
        }

        _ => Ok(None),
    }
}
