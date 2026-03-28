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

#[cfg(test)]
mod tests {
    use super::route;
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    #[tokio::test]
    async fn route_x25519_generate_ephemeral() {
        let out = route("crypto.x25519_generate_ephemeral", None)
            .await
            .expect("route")
            .expect("x25519 gen");
        assert!(out.get("public_key").and_then(|x| x.as_str()).is_some());
        assert!(out.get("secret_key").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn route_x25519_derive_roundtrip() {
        let alice = route("crypto.x25519_generate_ephemeral", None)
            .await
            .expect("route")
            .expect("alice");
        let bob = route("crypto.x25519_generate_ephemeral", None)
            .await
            .expect("route")
            .expect("bob");
        let alice_secret = alice
            .get("secret_key")
            .and_then(|x| x.as_str())
            .expect("alice secret");
        let alice_pub = alice
            .get("public_key")
            .and_then(|x| x.as_str())
            .expect("alice pub");
        let bob_secret = bob
            .get("secret_key")
            .and_then(|x| x.as_str())
            .expect("bob secret");
        let bob_pub = bob
            .get("public_key")
            .and_then(|x| x.as_str())
            .expect("bob pub");

        let s1 = json!({
            "our_secret": alice_secret,
            "their_public": bob_pub,
        });
        let s2 = json!({
            "our_secret": bob_secret,
            "their_public": alice_pub,
        });
        let out1 = route("crypto.x25519_derive_secret", Some(&s1))
            .await
            .expect("route")
            .expect("derive 1");
        let out2 = route("crypto.x25519_derive_secret", Some(&s2))
            .await
            .expect("route")
            .expect("derive 2");
        assert_eq!(
            out1.get("shared_secret").and_then(|x| x.as_str()),
            out2.get("shared_secret").and_then(|x| x.as_str())
        );
    }

    #[tokio::test]
    async fn route_ecdh_p256_generate_and_derive() {
        let empty = json!({});
        let a = route("crypto.ecdh_p256_generate", Some(&empty))
            .await
            .expect("route")
            .expect("p256 gen a");
        let b = route("crypto.ecdh_p256_generate", Some(&empty))
            .await
            .expect("route")
            .expect("p256 gen b");
        let derive_a = json!({
            "private_key": a.get("private_key").and_then(|x| x.as_str()).expect("priv a"),
            "peer_public_key": b.get("public_key").and_then(|x| x.as_str()).expect("pub b"),
        });
        let derive_b = json!({
            "private_key": b.get("private_key").and_then(|x| x.as_str()).expect("priv b"),
            "peer_public_key": a.get("public_key").and_then(|x| x.as_str()).expect("pub a"),
        });
        let s1 = route("crypto.ecdh_p256_derive", Some(&derive_a))
            .await
            .expect("route")
            .expect("derive a");
        let s2 = route("crypto.ecdh_p256_derive", Some(&derive_b))
            .await
            .expect("route")
            .expect("derive b");
        assert_eq!(
            s1.get("shared_secret").and_then(|x| x.as_str()),
            s2.get("shared_secret").and_then(|x| x.as_str())
        );
    }

    #[tokio::test]
    async fn route_ecdh_p384_generate() {
        let empty = json!({});
        let out = route("crypto.ecdh_p384_generate", Some(&empty))
            .await
            .expect("route")
            .expect("p384 gen");
        assert_eq!(out.get("curve").and_then(|x| x.as_str()), Some("P-384"));
    }

    #[tokio::test]
    async fn route_chacha20_encrypt_decrypt_roundtrip() {
        let key = [7u8; 32];
        let key_b64 = BASE64.encode(key);
        let plain = b"hello chacha aead";
        let plain_b64 = BASE64.encode(plain);
        let enc_params = json!({
            "plaintext": plain_b64,
            "key": key_b64,
        });
        let enc = route("crypto.chacha20_poly1305_encrypt", Some(&enc_params))
            .await
            .expect("route")
            .expect("encrypt");
        let dec_params = json!({
            "ciphertext": enc.get("ciphertext").and_then(|x| x.as_str()).expect("ct"),
            "nonce": enc.get("nonce").and_then(|x| x.as_str()).expect("nonce"),
            "tag": enc.get("tag").and_then(|x| x.as_str()).expect("tag"),
            "key": key_b64,
        });
        let dec = route("crypto.chacha20_poly1305_decrypt", Some(&dec_params))
            .await
            .expect("route")
            .expect("decrypt");
        let pt = BASE64
            .decode(
                dec.get("plaintext")
                    .and_then(|x| x.as_str())
                    .expect("plaintext b64"),
            )
            .expect("decode plaintext");
        assert_eq!(pt, plain);
    }

    #[tokio::test]
    async fn route_aes256_gcm_roundtrip() {
        let key = [9u8; 32];
        let key_b64 = BASE64.encode(key);
        let plain = b"aes256 payload";
        let plain_b64 = BASE64.encode(plain);
        let enc_params = json!({
            "plaintext": plain_b64,
            "key": key_b64,
        });
        let enc = route("crypto.aes256_gcm_encrypt", Some(&enc_params))
            .await
            .expect("route")
            .expect("aes enc");
        let dec_params = json!({
            "ciphertext": enc.get("ciphertext").and_then(|x| x.as_str()).expect("ct"),
            "nonce": enc.get("nonce").and_then(|x| x.as_str()).expect("nonce"),
            "key": key_b64,
        });
        let dec = route("crypto.aes256_gcm_decrypt", Some(&dec_params))
            .await
            .expect("route")
            .expect("aes dec");
        let pt = BASE64
            .decode(
                dec.get("plaintext")
                    .and_then(|x| x.as_str())
                    .expect("plaintext b64"),
            )
            .expect("decode plaintext");
        assert_eq!(pt, plain);
    }

    #[tokio::test]
    async fn route_aes128_gcm_roundtrip() {
        let key = [3u8; 16];
        let key_b64 = BASE64.encode(key);
        let plain = b"aes128";
        let plain_b64 = BASE64.encode(plain);
        let enc_params = json!({
            "plaintext": plain_b64,
            "key": key_b64,
        });
        let enc = route("crypto.aes128_gcm_encrypt", Some(&enc_params))
            .await
            .expect("route")
            .expect("aes128 enc");
        let dec_params = json!({
            "ciphertext": enc.get("ciphertext").and_then(|x| x.as_str()).expect("ct"),
            "nonce": enc.get("nonce").and_then(|x| x.as_str()).expect("nonce"),
            "key": key_b64,
        });
        let dec = route("crypto.aes128_gcm_decrypt", Some(&dec_params))
            .await
            .expect("route")
            .expect("aes128 dec");
        let pt = BASE64
            .decode(
                dec.get("plaintext")
                    .and_then(|x| x.as_str())
                    .expect("plaintext b64"),
            )
            .expect("decode plaintext");
        assert_eq!(pt, plain);
    }

    #[tokio::test]
    async fn route_ecdh_p256_missing_params_errors() {
        let err = route("crypto.ecdh_p256_derive", None)
            .await
            .expect_err("missing params");
        assert!(err.contains("Missing parameters"));
    }

    #[tokio::test]
    async fn route_unknown_method_returns_none() {
        assert!(
            route("crypto.no_such_kex", None)
                .await
                .expect("route")
                .is_none()
        );
    }
}
