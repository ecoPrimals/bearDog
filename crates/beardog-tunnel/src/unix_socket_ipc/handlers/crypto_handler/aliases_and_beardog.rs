// SPDX-License-Identifier: AGPL-3.0-only

//! Semantic aliases, `beardog.crypto.*` namespaced helpers, onion identity, and Tor protocol ops.

use crate::unix_socket_ipc::crypto_handlers_hashing::handle_generate_onion_identity;
use crate::unix_socket_ipc::crypto_handlers_tor::{
    handle_tor_cell_decrypt, handle_tor_cell_encrypt, handle_tor_kdf,
    handle_tor_ntor_client_finish, handle_tor_ntor_client_init, handle_tor_ntor_server_respond,
};
use crate::unix_socket_ipc::handlers::crypto::{
    handle_blake3_hash, handle_chacha20_poly1305_decrypt, handle_chacha20_poly1305_encrypt,
    handle_ed25519_generate_keypair, handle_hmac_sha256, handle_sign_ed25519,
    handle_verify_ed25519, handle_x25519_derive_secret, handle_x25519_generate_ephemeral,
};
use tracing::info;

pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    match method {
        "crypto.hash" => {
            info!("🔐 Crypto: hash (semantic → blake3_hash)");
            Ok(Some(handle_blake3_hash(params).await?))
        }

        "crypto.hmac" => {
            info!("🔐 Crypto: hmac (semantic → hmac_sha256)");
            Ok(Some(handle_hmac_sha256(params).await?))
        }

        "crypto.sign" => {
            info!("✍️  Crypto: sign (semantic → sign_ed25519)");
            Ok(Some(handle_sign_ed25519(params).await?))
        }

        "crypto.verify" => {
            info!("✅ Crypto: verify (semantic → verify_ed25519)");
            Ok(Some(handle_verify_ed25519(params).await?))
        }

        "crypto.encrypt" => {
            info!("🔒 Crypto: encrypt (semantic → chacha20_poly1305_encrypt)");
            Ok(Some(handle_chacha20_poly1305_encrypt(params).await?))
        }

        "crypto.decrypt" => {
            info!("🔓 Crypto: decrypt (semantic → chacha20_poly1305_decrypt)");
            Ok(Some(handle_chacha20_poly1305_decrypt(params).await?))
        }

        "crypto.generate_keypair" => {
            info!("🔑 Crypto: generate_keypair (semantic → x25519_generate_ephemeral)");
            Ok(Some(handle_x25519_generate_ephemeral(params).await?))
        }

        "crypto.derive_secret" => {
            info!("🔐 Crypto: derive_secret (semantic → x25519_derive_secret)");
            Ok(Some(handle_x25519_derive_secret(params).await?))
        }

        "beardog.crypto.ed25519_generate_keypair" => {
            info!("🧅 Crypto: beardog.crypto.ed25519_generate_keypair (Songbird Onion Identity)");
            Ok(Some(handle_ed25519_generate_keypair(params).await?))
        }

        "beardog.crypto.sign_ed25519" => {
            info!("🧅 Crypto: beardog.crypto.sign_ed25519 (Songbird Onion Service)");
            Ok(Some(handle_sign_ed25519(params).await?))
        }

        "beardog.crypto.verify_ed25519" => {
            info!("🧅 Crypto: beardog.crypto.verify_ed25519 (Songbird Onion Service)");
            Ok(Some(handle_verify_ed25519(params).await?))
        }

        "beardog.crypto.x25519_generate_ephemeral" => {
            info!("🧅 Crypto: beardog.crypto.x25519_generate_ephemeral (Songbird Onion Service)");
            Ok(Some(handle_x25519_generate_ephemeral(params).await?))
        }

        "beardog.crypto.x25519_derive_secret" => {
            info!("🧅 Crypto: beardog.crypto.x25519_derive_secret (Songbird Onion Service)");
            Ok(Some(handle_x25519_derive_secret(params).await?))
        }

        "beardog.crypto.chacha20_poly1305_encrypt" => {
            info!("🧅 Crypto: beardog.crypto.chacha20_poly1305_encrypt (Songbird Onion Service)");
            Ok(Some(handle_chacha20_poly1305_encrypt(params).await?))
        }

        "beardog.crypto.chacha20_poly1305_decrypt" => {
            info!("🧅 Crypto: beardog.crypto.chacha20_poly1305_decrypt (Songbird Onion Service)");
            Ok(Some(handle_chacha20_poly1305_decrypt(params).await?))
        }

        "beardog.crypto.hmac_sha256" => {
            info!("🧅 Crypto: beardog.crypto.hmac_sha256 (Songbird Onion Service)");
            Ok(Some(handle_hmac_sha256(params).await?))
        }

        "beardog.crypto.blake3_hash" => {
            info!("🧅 Crypto: beardog.crypto.blake3_hash (Songbird Onion Service)");
            Ok(Some(handle_blake3_hash(params).await?))
        }

        "beardog.crypto.generate_onion_identity" => {
            info!("🧅 Crypto: beardog.crypto.generate_onion_identity (Tor v3)");
            Ok(Some(handle_generate_onion_identity(params).await?))
        }

        "beardog.crypto.tor_ntor_client_init" => {
            info!("🧅 Crypto: tor_ntor_client_init (Tor ntor handshake - client init)");
            Ok(Some(
                handle_tor_ntor_client_init(params)
                    .await
                    .map_err(|e| e.to_string())?,
            ))
        }

        "beardog.crypto.tor_ntor_client_finish" => {
            info!("🧅 Crypto: tor_ntor_client_finish (Tor ntor handshake - client finish)");
            Ok(Some(
                handle_tor_ntor_client_finish(params)
                    .await
                    .map_err(|e| e.to_string())?,
            ))
        }

        "beardog.crypto.tor_ntor_server_respond" => {
            info!("🧅 Crypto: tor_ntor_server_respond (Tor ntor handshake - server)");
            Ok(Some(
                handle_tor_ntor_server_respond(params)
                    .await
                    .map_err(|e| e.to_string())?,
            ))
        }

        "beardog.crypto.tor_cell_encrypt" => {
            info!("🧅 Crypto: tor_cell_encrypt (Tor relay cell encryption)");
            Ok(Some(
                handle_tor_cell_encrypt(params)
                    .await
                    .map_err(|e| e.to_string())?,
            ))
        }

        "beardog.crypto.tor_cell_decrypt" => {
            info!("🧅 Crypto: tor_cell_decrypt (Tor relay cell decryption)");
            Ok(Some(
                handle_tor_cell_decrypt(params)
                    .await
                    .map_err(|e| e.to_string())?,
            ))
        }

        "beardog.crypto.tor_kdf" => {
            info!("🧅 Crypto: tor_kdf (Tor key derivation)");
            Ok(Some(
                handle_tor_kdf(params).await.map_err(|e| e.to_string())?,
            ))
        }

        _ => Ok(None),
    }
}
