// SPDX-License-Identifier: AGPL-3.0-only

//! Key exchange (X25519, ECDH) and AEAD (ChaCha20-Poly1305, AES-GCM).

use crate::unix_socket_ipc::crypto_handlers_aes_gcm::{
    handle_aes128_gcm_decrypt, handle_aes128_gcm_encrypt, handle_aes256_gcm_decrypt,
    handle_aes256_gcm_encrypt,
};
use crate::unix_socket_ipc::crypto_handlers_ecdh::{
    handle_ecdh_p256_derive, handle_ecdh_p256_generate, handle_ecdh_p384_derive,
    handle_ecdh_p384_generate,
};
use crate::unix_socket_ipc::handlers::crypto::{
    handle_chacha20_poly1305_decrypt, handle_chacha20_poly1305_encrypt,
    handle_x25519_derive_secret, handle_x25519_generate_ephemeral,
};
use tracing::info;

pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    match method {
        "crypto.x25519_generate_ephemeral" => {
            info!("🔑 Crypto: x25519_generate_ephemeral");
            Ok(Some(handle_x25519_generate_ephemeral(params).await?))
        }

        "crypto.x25519_derive_secret" => {
            info!("🤝 Crypto: x25519_derive_secret (ECDH key exchange)");
            Ok(Some(handle_x25519_derive_secret(params).await?))
        }

        "crypto.ecdh_p256_generate" => {
            info!("🔑 Crypto: ecdh_p256_generate (P-256 keypair for TLS 1.3)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_ecdh_p256_generate(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.ecdh_p256_derive" => {
            info!("🤝 Crypto: ecdh_p256_derive (P-256 ECDH key exchange for TLS 1.3)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_ecdh_p256_derive(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.ecdh_p384_generate" => {
            info!("🔑 Crypto: ecdh_p384_generate (P-384 keypair for TLS 1.3)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_ecdh_p384_generate(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.ecdh_p384_derive" => {
            info!("🤝 Crypto: ecdh_p384_derive (P-384 ECDH key exchange for TLS 1.3)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_ecdh_p384_derive(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.chacha20_poly1305_encrypt" => {
            info!("🔒 Crypto: chacha20_poly1305_encrypt (AEAD)");
            Ok(Some(handle_chacha20_poly1305_encrypt(params).await?))
        }

        "crypto.chacha20_poly1305_decrypt" => {
            info!("🔓 Crypto: chacha20_poly1305_decrypt (AEAD)");
            Ok(Some(handle_chacha20_poly1305_decrypt(params).await?))
        }

        "crypto.aes256_gcm_encrypt" => {
            info!("🔒 Crypto: aes256_gcm_encrypt (90%+ of HTTPS!)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_aes256_gcm_encrypt(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.aes256_gcm_decrypt" => {
            info!("🔓 Crypto: aes256_gcm_decrypt (90%+ of HTTPS!)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_aes256_gcm_decrypt(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.aes128_gcm_encrypt" => {
            info!("🔒 Crypto: aes128_gcm_encrypt (80%+ of HTTPS!)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_aes128_gcm_encrypt(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        "crypto.aes128_gcm_decrypt" => {
            info!("🔓 Crypto: aes128_gcm_decrypt (80%+ of HTTPS!)");
            let params_ref = params.ok_or_else(|| "Missing parameters".to_string())?;
            Ok(Some(
                handle_aes128_gcm_decrypt(params_ref).map_err(|e| e.to_string())?,
            ))
        }

        _ => Ok(None),
    }
}
