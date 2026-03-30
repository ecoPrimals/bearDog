// SPDX-License-Identifier: AGPL-3.0-only

//! TLS 1.2 semantic method names (ECDHE, AES-GCM, PRF) for handshake integration.

use crate::unix_socket_ipc::handlers::crypto::{
    handle_aes_128_gcm_decrypt, handle_aes_128_gcm_encrypt, handle_aes_256_gcm_decrypt,
    handle_aes_256_gcm_encrypt, handle_ecdhe_p256_compute_shared, handle_ecdhe_p256_generate,
    handle_ecdhe_p384_compute_shared, handle_ecdhe_p384_generate, handle_tls12_prf,
};
use tracing::info;

/// # Errors
///
/// Returns an error if decryption fails.
pub async fn route(
    method: &str,
    params: Option<&serde_json::Value>,
) -> Result<Option<serde_json::Value>, String> {
    match method {
        "crypto.ecdhe.p256.generate" => {
            info!(
                "🔑 Crypto: ecdhe.p256.generate (TLS 1.2 P-256 keypair for handshake integration)"
            );
            Ok(Some(handle_ecdhe_p256_generate(params).await?))
        }

        "crypto.ecdhe.p256.compute_shared" => {
            info!(
                "🤝 Crypto: ecdhe.p256.compute_shared (TLS 1.2 P-256 ECDH for handshake integration)"
            );
            Ok(Some(handle_ecdhe_p256_compute_shared(params).await?))
        }

        "crypto.ecdhe.p384.generate" => {
            info!(
                "🔑 Crypto: ecdhe.p384.generate (TLS 1.2 P-384 keypair for handshake integration)"
            );
            Ok(Some(handle_ecdhe_p384_generate(params).await?))
        }

        "crypto.ecdhe.p384.compute_shared" => {
            info!(
                "🤝 Crypto: ecdhe.p384.compute_shared (TLS 1.2 P-384 ECDH for handshake integration)"
            );
            Ok(Some(handle_ecdhe_p384_compute_shared(params).await?))
        }

        "crypto.aead.aes_128_gcm.encrypt" => {
            info!(
                "🔒 Crypto: aead.aes_128_gcm.encrypt (TLS 1.2 AES-128-GCM for handshake integration)"
            );
            Ok(Some(handle_aes_128_gcm_encrypt(params).await?))
        }

        "crypto.aead.aes_128_gcm.decrypt" => {
            info!(
                "🔓 Crypto: aead.aes_128_gcm.decrypt (TLS 1.2 AES-128-GCM for handshake integration)"
            );
            Ok(Some(handle_aes_128_gcm_decrypt(params).await?))
        }

        "crypto.aead.aes_256_gcm.encrypt" => {
            info!(
                "🔒 Crypto: aead.aes_256_gcm.encrypt (TLS 1.2 AES-256-GCM for handshake integration)"
            );
            Ok(Some(handle_aes_256_gcm_encrypt(params).await?))
        }

        "crypto.aead.aes_256_gcm.decrypt" => {
            info!(
                "🔓 Crypto: aead.aes_256_gcm.decrypt (TLS 1.2 AES-256-GCM for handshake integration)"
            );
            Ok(Some(handle_aes_256_gcm_decrypt(params).await?))
        }

        "crypto.kdf.tls12_prf" => {
            info!("🔑 Crypto: kdf.tls12_prf (TLS 1.2 PRF key expansion for handshake integration)");
            Ok(Some(handle_tls12_prf(params).await?))
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
    async fn route_ecdhe_p256_generate_none_params() {
        let v = route("crypto.ecdhe.p256.generate", None)
            .await
            .expect("route")
            .expect("some");
        assert_eq!(v.get("algorithm").and_then(|x| x.as_str()), Some("P-256"));
        assert!(v.get("public_key").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn route_ecdhe_p256_generate_with_purpose() {
        let params = json!({ "purpose": "tls12_route_test" });
        let v = route("crypto.ecdhe.p256.generate", Some(&params))
            .await
            .expect("route")
            .expect("some");
        assert!(v.get("secret_key").is_some());
    }

    #[tokio::test]
    async fn route_ecdhe_p256_compute_shared_roundtrip() {
        let a = route("crypto.ecdhe.p256.generate", None)
            .await
            .expect("route")
            .expect("a");
        let b = route("crypto.ecdhe.p256.generate", None)
            .await
            .expect("route")
            .expect("b");
        let params = json!({
            "our_secret": a.get("secret_key").and_then(|x| x.as_str()).expect("sk"),
            "their_public": b.get("public_key").and_then(|x| x.as_str()).expect("pk"),
        });
        let v = route("crypto.ecdhe.p256.compute_shared", Some(&params))
            .await
            .expect("route")
            .expect("shared");
        assert!(v.get("shared_secret").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn route_ecdhe_p384_generate_and_shared() {
        let a = route("crypto.ecdhe.p384.generate", None)
            .await
            .expect("route")
            .expect("a");
        let b = route("crypto.ecdhe.p384.generate", None)
            .await
            .expect("route")
            .expect("b");
        let params = json!({
            "our_secret": a.get("secret_key").and_then(|x| x.as_str()).expect("sk"),
            "their_public": b.get("public_key").and_then(|x| x.as_str()).expect("pk"),
        });
        let v = route("crypto.ecdhe.p384.compute_shared", Some(&params))
            .await
            .expect("route")
            .expect("shared");
        assert_eq!(v.get("algorithm").and_then(|x| x.as_str()), Some("P-384"));
    }

    #[tokio::test]
    async fn route_aes_128_gcm_encrypt_decrypt_roundtrip() {
        let key = B64.encode([7u8; 16]);
        let nonce = B64.encode([0u8; 12]);
        let pt = B64.encode(b"hello-tls12-aes128");
        let enc_params = json!({
            "key": key,
            "nonce": nonce,
            "plaintext": pt,
        });
        let enc = route("crypto.aead.aes_128_gcm.encrypt", Some(&enc_params))
            .await
            .expect("route")
            .expect("enc");
        let dec_params = json!({
            "key": key,
            "nonce": nonce,
            "ciphertext": enc.get("ciphertext").and_then(|x| x.as_str()).expect("ct"),
            "tag": enc.get("tag").and_then(|x| x.as_str()).expect("tag"),
        });
        let dec = route("crypto.aead.aes_128_gcm.decrypt", Some(&dec_params))
            .await
            .expect("route")
            .expect("dec");
        assert_eq!(
            dec.get("plaintext").and_then(|x| x.as_str()),
            Some(pt.as_str())
        );
    }

    #[tokio::test]
    async fn route_aes_256_gcm_encrypt_decrypt_roundtrip() {
        let key = B64.encode([9u8; 32]);
        let nonce = B64.encode([1u8; 12]);
        let pt = B64.encode(b"hello-tls12-aes256");
        let enc_params = json!({ "key": key, "nonce": nonce, "plaintext": pt });
        let enc = route("crypto.aead.aes_256_gcm.encrypt", Some(&enc_params))
            .await
            .expect("route")
            .expect("enc");
        let dec_params = json!({
            "key": key,
            "nonce": nonce,
            "ciphertext": enc.get("ciphertext").and_then(|x| x.as_str()).expect("ct"),
            "tag": enc.get("tag").and_then(|x| x.as_str()).expect("tag"),
        });
        let dec = route("crypto.aead.aes_256_gcm.decrypt", Some(&dec_params))
            .await
            .expect("route")
            .expect("dec");
        assert_eq!(
            dec.get("plaintext").and_then(|x| x.as_str()),
            Some(pt.as_str())
        );
    }

    #[tokio::test]
    async fn route_tls12_prf_sha256_and_sha384() {
        let secret = B64.encode([3u8; 32]);
        let seed = B64.encode([4u8; 64]);
        let p256 = json!({
            "secret": secret,
            "label": "key expansion",
            "seed": seed,
            "output_len": 32,
            "hash": "sha256"
        });
        let o1 = route("crypto.kdf.tls12_prf", Some(&p256))
            .await
            .expect("route")
            .expect("prf");
        assert!(o1.get("output").and_then(|x| x.as_str()).is_some());

        let p384 = json!({
            "secret": B64.encode([5u8; 48]),
            "label": "key expansion",
            "seed": seed,
            "output_len": 48,
            "hash": "sha384"
        });
        let o2 = route("crypto.kdf.tls12_prf", Some(&p384))
            .await
            .expect("route")
            .expect("prf384");
        assert!(o2.get("output").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn route_unknown_tls12_method_returns_none() {
        assert!(
            route("crypto.tls12.not_a_method", None)
                .await
                .expect("route")
                .is_none()
        );
    }

    #[tokio::test]
    async fn route_tls12_prf_rejects_bad_hash() {
        let params = json!({
            "secret": z32_b64(),
            "label": "x",
            "seed": z32_b64(),
            "output_len": 16,
            "hash": "md5"
        });
        let err = route("crypto.kdf.tls12_prf", Some(&params))
            .await
            .expect_err("bad hash");
        assert!(err.contains("Unsupported") || err.contains("hash"));
    }
}
