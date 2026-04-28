// SPDX-License-Identifier: AGPL-3.0-or-later

//! Semantic `crypto.*` operations are primary; `beardog.crypto.*` names are backward-compat aliases.

use crate::unix_socket_ipc::crypto_handlers_hashing::handle_generate_onion_identity;
use crate::unix_socket_ipc::crypto_handlers_tor::{
    handle_tor_cell_decrypt, handle_tor_cell_encrypt, handle_tor_kdf,
    handle_tor_ntor_client_finish, handle_tor_ntor_client_init, handle_tor_ntor_server_respond,
};
use crate::unix_socket_ipc::handlers::crypto::{
    handle_blake3_hash, handle_chacha20_poly1305_decrypt, handle_chacha20_poly1305_encrypt,
    handle_ed25519_generate_keypair, handle_generate_keypair_with_hsm, handle_hmac_sha256,
    handle_public_key, handle_sign_ed25519, handle_verify_ed25519, handle_x25519_derive_secret,
    handle_x25519_generate_ephemeral,
};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_core::crypto_service::algorithms::hashing;
use serde_json::{Value, json};
use tracing::info;

/// # Errors
///
/// Returns an error if key generation fails in the underlying HSM provider.
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

        "crypto.public_key" => {
            info!("🔑 Crypto: public_key (retrieve Ed25519 public key for key_id)");
            Ok(Some(handle_public_key(params).await?))
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
            if params.is_some_and(|p| p.get("hsm_backend").is_some()) {
                info!("🔑 Crypto: generate_keypair (hsm_backend → generate_keypair_with_hsm)");
                Ok(Some(handle_generate_keypair_with_hsm(params).await?))
            } else {
                info!("🔑 Crypto: generate_keypair (semantic → x25519_generate_ephemeral)");
                Ok(Some(handle_x25519_generate_ephemeral(params).await?))
            }
        }

        "crypto.derive_secret" => {
            info!("🔐 Crypto: derive_secret (semantic → x25519_derive_secret)");
            Ok(Some(handle_x25519_derive_secret(params).await?))
        }

        "crypto.derive_purpose_key" => {
            info!("🔑 Crypto: derive_purpose_key (NUCLEUS two-tier key derivation)");
            Ok(Some(handle_derive_purpose_key(params).await?))
        }

        "crypto.sign_registration" => {
            info!("📋 Crypto: sign_registration (signed ipc.register payload)");
            Ok(Some(handle_sign_registration(params).await?))
        }

        "beardog.crypto.ed25519_generate_keypair" => {
            info!("🧅 Crypto: beardog.crypto.ed25519_generate_keypair (onion identity route)");
            Ok(Some(handle_ed25519_generate_keypair(params).await?))
        }

        "beardog.crypto.sign_ed25519" => {
            info!("🧅 Crypto: beardog.crypto.sign_ed25519 (onion service crypto)");
            Ok(Some(handle_sign_ed25519(params).await?))
        }

        "beardog.crypto.verify_ed25519" => {
            info!("🧅 Crypto: beardog.crypto.verify_ed25519 (onion service crypto)");
            Ok(Some(handle_verify_ed25519(params).await?))
        }

        "beardog.crypto.x25519_generate_ephemeral" => {
            info!("🧅 Crypto: beardog.crypto.x25519_generate_ephemeral (onion service crypto)");
            Ok(Some(handle_x25519_generate_ephemeral(params).await?))
        }

        "beardog.crypto.x25519_derive_secret" => {
            info!("🧅 Crypto: beardog.crypto.x25519_derive_secret (onion service crypto)");
            Ok(Some(handle_x25519_derive_secret(params).await?))
        }

        "beardog.crypto.chacha20_poly1305_encrypt" => {
            info!("🧅 Crypto: beardog.crypto.chacha20_poly1305_encrypt (onion service crypto)");
            Ok(Some(handle_chacha20_poly1305_encrypt(params).await?))
        }

        "beardog.crypto.chacha20_poly1305_decrypt" => {
            info!("🧅 Crypto: beardog.crypto.chacha20_poly1305_decrypt (onion service crypto)");
            Ok(Some(handle_chacha20_poly1305_decrypt(params).await?))
        }

        "beardog.crypto.hmac_sha256" => {
            info!("🧅 Crypto: beardog.crypto.hmac_sha256 (onion service crypto)");
            Ok(Some(handle_hmac_sha256(params).await?))
        }

        "beardog.crypto.blake3_hash" => {
            info!("🧅 Crypto: beardog.crypto.blake3_hash (onion service crypto)");
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

/// Derive a purpose-specific key from a parent key using the NUCLEUS convention.
///
/// Wire: `crypto.derive_purpose_key`
///
/// ```text
/// purpose_key = HMAC-SHA256(key, hex("purpose-v1:" + purpose))
/// ```
///
/// # Parameters
///
/// - `key`: Base64-encoded parent key (typically a family key)
/// - `purpose`: Purpose string (e.g. `"storage"`, `"dag"`, `"security"`)
///
/// # Returns
///
/// - `key`: Base64-encoded derived 32-byte purpose key
/// - `purpose`: Echo of the purpose string
/// - `method`: `"HMAC-SHA256-purpose-v1"`
async fn handle_derive_purpose_key(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.derive_purpose_key")?;

    let key_b64 = params
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: key (base64-encoded parent key)")?;

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: purpose")?;

    let key = BASE64
        .decode(key_b64)
        .map_err(|e| format!("Invalid base64 key: {e}"))?;

    let msg = hex::encode(format!("purpose-v1:{purpose}"));
    let msg_bytes = msg.as_bytes();

    let derived = hashing::hmac_sha256(&key, msg_bytes)
        .map_err(|e| format!("HMAC-SHA256 purpose derivation failed: {e}"))?;

    let derived_b64 = BASE64.encode(&derived);

    info!("✅ Derived purpose key for '{}' (32 bytes)", purpose);

    Ok(json!({
        "key": derived_b64,
        "purpose": purpose,
        "method": "HMAC-SHA256-purpose-v1",
    }))
}

/// Sign an `ipc.register` payload so consumers can verify authentic registrations.
///
/// Wire: `crypto.sign_registration`
///
/// Canonicalizes the registration fields (`primal_id`, `capabilities`, `endpoint`)
/// into a deterministic byte string and signs it with Ed25519.
///
/// # Parameters
///
/// - `primal_id`: Primal identifier (e.g. `"barracuda"`)
/// - `capabilities`: JSON array of capability strings
/// - `endpoint`: UDS endpoint path
/// - `key_id` (optional): Ed25519 key identifier (defaults to `"default_signing_key"`)
///
/// # Returns
///
/// - `signature`: Base64-encoded Ed25519 signature
/// - `public_key`: Base64-encoded Ed25519 public key (for verification)
/// - `canonical`: The canonical string that was signed
/// - `algorithm`: `"Ed25519"`
async fn handle_sign_registration(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.sign_registration")?;

    let primal_id = params
        .get("primal_id")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: primal_id")?;

    let capabilities = params
        .get("capabilities")
        .and_then(|v| v.as_array())
        .ok_or("Missing required parameter: capabilities (JSON array)")?;

    let endpoint = params
        .get("endpoint")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: endpoint")?;

    let key_id = params
        .get("key_id")
        .and_then(|v| v.as_str())
        .unwrap_or("default_signing_key");

    let mut caps: Vec<&str> = capabilities.iter().filter_map(|v| v.as_str()).collect();
    caps.sort_unstable();

    let canonical = format!(
        "ipc.register-v1:primal_id={},capabilities=[{}],endpoint={}",
        primal_id,
        caps.join(","),
        endpoint
    );

    let msg_b64 = BASE64.encode(canonical.as_bytes());
    let sign_params = json!({
        "message": msg_b64,
        "key_id": key_id,
        "purpose": "registration",
    });

    let sig_result = handle_sign_ed25519(Some(&sign_params)).await?;

    info!(
        "✅ Signed registration for '{}' ({} capabilities)",
        primal_id,
        caps.len()
    );

    Ok(json!({
        "signature": sig_result.get("signature").and_then(|v| v.as_str()),
        "public_key": sig_result.get("public_key").and_then(|v| v.as_str()),
        "canonical": canonical,
        "algorithm": "Ed25519",
    }))
}

#[cfg(test)]
mod tests {
    use super::route;
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD as BASE64;
    use serde_json::json;

    #[tokio::test]
    async fn route_crypto_hash_alias() {
        let params = json!({ "data": BASE64.encode(b"blake3-input") });
        let out = route("crypto.hash", Some(&params))
            .await
            .expect("route")
            .expect("hash");
        assert_eq!(
            out.get("algorithm").and_then(|x| x.as_str()),
            Some("BLAKE3")
        );
    }

    #[tokio::test]
    async fn route_crypto_hmac_alias() {
        let params = json!({
            "key": BASE64.encode(b"secret-hmac-key-123456789012"),
            "data": BASE64.encode(b"authenticated payload"),
        });
        let out = route("crypto.hmac", Some(&params))
            .await
            .expect("route")
            .expect("hmac");
        assert!(out.get("mac").and_then(|x| x.as_str()).is_some());
    }

    #[tokio::test]
    async fn route_crypto_sign_and_verify_alias() {
        let key_id = "aliases-alias-sign";
        let purpose = "general";
        let msg = BASE64.encode(b"message to sign");
        let sign_params = json!({
            "message": msg,
            "key_id": key_id,
            "purpose": purpose,
        });
        let sig = route("crypto.sign", Some(&sign_params))
            .await
            .expect("route")
            .expect("sign");
        let sig_b64 = sig.get("signature").and_then(|x| x.as_str()).expect("sig");
        let pk_b64 = sig
            .get("public_key")
            .and_then(|x| x.as_str())
            .expect("sign response must include public_key for IPC roundtrip");
        let verify_params = json!({
            "public_key": pk_b64,
            "message": msg,
            "signature": sig_b64,
        });
        let v = route("crypto.verify", Some(&verify_params))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_crypto_public_key_matches_sign_response() {
        let key_id = "pk-roundtrip-key";
        let purpose = "general";
        let pk_params = json!({ "key_id": key_id, "purpose": purpose });
        let pk_result = route("crypto.public_key", Some(&pk_params))
            .await
            .expect("route")
            .expect("public_key");
        let pk_b64 = pk_result
            .get("public_key")
            .and_then(|x| x.as_str())
            .expect("public_key field");
        assert_eq!(
            pk_result.get("algorithm").and_then(|x| x.as_str()),
            Some("Ed25519")
        );

        let msg = BASE64.encode(b"verify pk matches");
        let sign_params = json!({ "message": msg, "key_id": key_id, "purpose": purpose });
        let sig = route("crypto.sign", Some(&sign_params))
            .await
            .expect("route")
            .expect("sign");
        let sign_pk = sig
            .get("public_key")
            .and_then(|x| x.as_str())
            .expect("sign public_key");
        assert_eq!(
            pk_b64, sign_pk,
            "crypto.public_key must return the same key as crypto.sign"
        );

        let verify_params = json!({
            "public_key": pk_b64,
            "message": msg,
            "signature": sig.get("signature").and_then(|x| x.as_str()).expect("sig"),
        });
        let v = route("crypto.verify", Some(&verify_params))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_crypto_public_key_default_key_id() {
        let pk = route("crypto.public_key", None)
            .await
            .expect("route")
            .expect("public_key with defaults");
        assert_eq!(
            pk.get("key_id").and_then(|x| x.as_str()),
            Some("default_signing_key")
        );
        assert_eq!(
            pk.get("algorithm").and_then(|x| x.as_str()),
            Some("Ed25519")
        );
        let pk_bytes = BASE64
            .decode(pk.get("public_key").and_then(|x| x.as_str()).expect("pk"))
            .expect("valid base64");
        assert_eq!(pk_bytes.len(), 32);
    }

    #[tokio::test]
    async fn route_crypto_encrypt_decrypt_alias() {
        let key = [11u8; 32];
        let key_b64 = BASE64.encode(key);
        let plain = b"alias secret";
        let enc_params = json!({
            "plaintext": BASE64.encode(plain),
            "key": key_b64,
        });
        let enc = route("crypto.encrypt", Some(&enc_params))
            .await
            .expect("route")
            .expect("enc");
        let dec_params = json!({
            "ciphertext": enc.get("ciphertext").and_then(|x| x.as_str()).expect("ct"),
            "nonce": enc.get("nonce").and_then(|x| x.as_str()).expect("nonce"),
            "tag": enc.get("tag").and_then(|x| x.as_str()).expect("tag"),
            "key": key_b64,
        });
        let dec = route("crypto.decrypt", Some(&dec_params))
            .await
            .expect("route")
            .expect("dec");
        let pt = BASE64
            .decode(
                dec.get("plaintext")
                    .and_then(|x| x.as_str())
                    .expect("pt b64"),
            )
            .expect("decode");
        assert_eq!(pt, plain);
    }

    #[tokio::test]
    async fn route_crypto_generate_keypair_and_derive_secret_alias() {
        let a = route("crypto.generate_keypair", None)
            .await
            .expect("route")
            .expect("a");
        let b = route("crypto.generate_keypair", None)
            .await
            .expect("route")
            .expect("b");
        let p = json!({
            "our_secret": a.get("secret_key").and_then(|x| x.as_str()).expect("sa"),
            "their_public": b.get("public_key").and_then(|x| x.as_str()).expect("pb"),
        });
        let s1 = route("crypto.derive_secret", Some(&p))
            .await
            .expect("route")
            .expect("s1");
        let p2 = json!({
            "our_secret": b.get("secret_key").and_then(|x| x.as_str()).expect("sb"),
            "their_public": a.get("public_key").and_then(|x| x.as_str()).expect("pa"),
        });
        let s2 = route("crypto.derive_secret", Some(&p2))
            .await
            .expect("route")
            .expect("s2");
        assert_eq!(
            s1.get("shared_secret").and_then(|x| x.as_str()),
            s2.get("shared_secret").and_then(|x| x.as_str())
        );
    }

    #[tokio::test]
    async fn route_beardog_crypto_namespaced_helpers() {
        let b3 = json!({ "data": BASE64.encode(b"namespaced") });
        let h = route("beardog.crypto.blake3_hash", Some(&b3))
            .await
            .expect("route")
            .expect("blake3");
        assert!(h.get("hash").and_then(|x| x.as_str()).is_some());

        let hm = json!({
            "key": BASE64.encode(b"k".repeat(32)),
            "data": BASE64.encode(b"d"),
        });
        let mac = route("beardog.crypto.hmac_sha256", Some(&hm))
            .await
            .expect("route")
            .expect("hmac");
        assert!(mac.get("mac").and_then(|x| x.as_str()).is_some());

        let onion = route("beardog.crypto.generate_onion_identity", None)
            .await
            .expect("route")
            .expect("onion");
        assert!(onion.get("public_key").is_some() || onion.get("onion_address").is_some());
    }

    #[tokio::test]
    async fn route_crypto_derive_purpose_key() {
        let family_key = BASE64.encode(b"test-family-key-32-bytes-long!!");
        let params = json!({ "key": family_key, "purpose": "storage" });
        let out = route("crypto.derive_purpose_key", Some(&params))
            .await
            .expect("route")
            .expect("derive_purpose_key");
        assert_eq!(out["method"], "HMAC-SHA256-purpose-v1");
        assert_eq!(out["purpose"], "storage");
        let key_bytes = BASE64
            .decode(out["key"].as_str().expect("key"))
            .expect("valid base64");
        assert_eq!(key_bytes.len(), 32);
    }

    #[tokio::test]
    async fn route_crypto_derive_purpose_key_deterministic() {
        let family_key = BASE64.encode(b"deterministic-family-key-bytes!");
        let params = json!({ "key": family_key, "purpose": "dag" });
        let r1 = route("crypto.derive_purpose_key", Some(&params))
            .await
            .expect("route")
            .expect("first");
        let r2 = route("crypto.derive_purpose_key", Some(&params))
            .await
            .expect("route")
            .expect("second");
        assert_eq!(r1["key"], r2["key"]);
    }

    #[tokio::test]
    async fn route_crypto_derive_purpose_key_different_purposes_differ() {
        let family_key = BASE64.encode(b"shared-family-key-for-test!!!!!");
        let storage = route(
            "crypto.derive_purpose_key",
            Some(&json!({ "key": family_key, "purpose": "storage" })),
        )
        .await
        .expect("route")
        .expect("storage");
        let dag = route(
            "crypto.derive_purpose_key",
            Some(&json!({ "key": family_key, "purpose": "dag" })),
        )
        .await
        .expect("route")
        .expect("dag");
        assert_ne!(storage["key"], dag["key"]);
    }

    #[tokio::test]
    async fn route_crypto_sign_registration() {
        let params = json!({
            "primal_id": "barracuda",
            "capabilities": ["tensor", "math", "stats"],
            "endpoint": "unix:///run/user/1000/biomeos/barracuda-test.sock",
        });
        let out = route("crypto.sign_registration", Some(&params))
            .await
            .expect("route")
            .expect("sign_registration");
        assert_eq!(out["algorithm"], "Ed25519");
        assert!(out["signature"].as_str().is_some());
        assert!(out["public_key"].as_str().is_some());
        let canonical = out["canonical"].as_str().expect("canonical");
        assert!(canonical.starts_with("ipc.register-v1:primal_id=barracuda,"));
        assert!(canonical.contains("math,stats,tensor"));
    }

    #[tokio::test]
    async fn route_crypto_sign_registration_verify_roundtrip() {
        let params = json!({
            "primal_id": "nestgate",
            "capabilities": ["storage"],
            "endpoint": "unix:///tmp/nestgate.sock",
        });
        let reg = route("crypto.sign_registration", Some(&params))
            .await
            .expect("route")
            .expect("sign_registration");

        let canonical_b64 = BASE64.encode(reg["canonical"].as_str().expect("c").as_bytes());
        let verify_params = json!({
            "public_key": reg["public_key"].as_str().expect("pk"),
            "message": canonical_b64,
            "signature": reg["signature"].as_str().expect("sig"),
        });
        let v = route("crypto.verify", Some(&verify_params))
            .await
            .expect("route")
            .expect("verify");
        assert_eq!(v.get("valid").and_then(|x| x.as_bool()), Some(true));
    }

    #[tokio::test]
    async fn route_unknown_method_returns_none() {
        assert!(
            route("crypto.not_an_alias", Some(&json!({})))
                .await
                .expect("route")
                .is_none()
        );
    }
}
