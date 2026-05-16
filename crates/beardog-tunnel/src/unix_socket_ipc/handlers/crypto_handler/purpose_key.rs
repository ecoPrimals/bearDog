// SPDX-License-Identifier: AGPL-3.0-or-later

//! NUCLEUS purpose-key operations and signed registration.
//!
//! Implements the NUCLEUS two-tier crypto model:
//! - `crypto.derive_purpose_key` — derive purpose-specific keys from a parent key
//! - `crypto.derive_public_key` — derive Ed25519 public key for a purpose from `FAMILY_SEED`
//! - `crypto.sign_registration` — sign `ipc.register` payloads for verifiable registrations
//! - `crypto.encrypt` / `crypto.decrypt` with `purpose` param — purpose-key envelope operations

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use beardog_core::crypto_service::algorithms::hashing;
use chacha20poly1305::{
    ChaCha20Poly1305,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use serde_json::{Value, json};
use tracing::info;

use crate::unix_socket_ipc::handlers::crypto::handle_sign_ed25519;
use ed25519_dalek::SigningKey;

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
pub async fn handle_derive_purpose_key(params: Option<&Value>) -> Result<Value, String> {
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

/// Derive a purpose-specific Ed25519 public key from `FAMILY_SEED`.
///
/// Wire: `crypto.derive_public_key`
///
/// Combines purpose-key derivation with Ed25519 keypair generation:
/// 1. `purpose_key = HMAC-SHA256(FAMILY_SEED, hex("purpose-v1:" + purpose))`
/// 2. `signing_key = Ed25519::from_bytes(purpose_key)`
/// 3. Return the verifying (public) key.
///
/// # Parameters
///
/// - `purpose`: Purpose string (e.g. `"coordination"`, `"storage"`, `"inference"`)
///
/// # Returns
///
/// - `public_key`: Base64-encoded Ed25519 public key (32 bytes)
/// - `algorithm`: `"Ed25519"`
/// - `purpose`: Echo of the purpose string
/// - `derivation`: `"HMAC-SHA256-purpose-v1 → Ed25519"`
pub async fn handle_derive_public_key(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or(
        "Missing params for crypto.derive_public_key — expected: {\"purpose\": \"<string>\"}",
    )?;

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: purpose (e.g. \"coordination\", \"storage\")")?;

    let purpose_key = resolve_purpose_key(purpose)?;

    let signing_key = SigningKey::from_bytes(&purpose_key);
    let public_key = signing_key.verifying_key();
    let public_key_b64 = BASE64.encode(public_key.as_bytes());

    info!(
        "🔑 Derived public key for purpose '{}' (Ed25519, 32 bytes)",
        purpose
    );

    Ok(json!({
        "public_key": public_key_b64,
        "algorithm": "Ed25519",
        "purpose": purpose,
        "derivation": "HMAC-SHA256-purpose-v1 → Ed25519",
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
/// - `primal_id`: Primal identifier
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
pub async fn handle_sign_registration(params: Option<&Value>) -> Result<Value, String> {
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

/// Resolve a NUCLEUS purpose key from `FAMILY_SEED` / `BEARDOG_FAMILY_SEED`.
///
/// Same HMAC-SHA256 convention as `handle_derive_purpose_key`:
/// `purpose_key = HMAC-SHA256(family_seed, hex("purpose-v1:" + purpose))`
fn resolve_purpose_key(purpose: &str) -> Result<[u8; 32], String> {
    let family_seed = load_family_seed()?;

    let msg = hex::encode(format!("purpose-v1:{purpose}"));
    let derived = hashing::hmac_sha256(&family_seed, msg.as_bytes())
        .map_err(|e| format!("HMAC-SHA256 purpose derivation failed: {e}"))?;

    let key: [u8; 32] = derived
        .as_slice()
        .try_into()
        .map_err(|_| "Purpose key derivation produced unexpected length".to_string())?;

    Ok(key)
}

fn load_family_seed() -> Result<Vec<u8>, String> {
    if let Ok(seed) = beardog_errors::process_env::var("BEARDOG_FAMILY_SEED")
        && !seed.is_empty()
    {
        return Ok(seed.into_bytes());
    }
    if let Ok(seed) = beardog_errors::process_env::var("FAMILY_SEED")
        && !seed.is_empty()
    {
        return Ok(seed.into_bytes());
    }
    Err(
        "Purpose-based encrypt/decrypt requires FAMILY_SEED or BEARDOG_FAMILY_SEED env var"
            .to_string(),
    )
}

/// Encrypt with a NUCLEUS purpose key, returning the standard envelope.
///
/// Wire: `crypto.encrypt` with `purpose` param
///
/// # Parameters
///
/// - `data`: Base64-encoded plaintext
/// - `purpose`: Purpose string (e.g. `"storage"`, `"inference"`)
///
/// # Returns
///
/// NUCLEUS standard envelope: `{"v":1,"ct":"<b64>","n":"<b64>","alg":"chacha20-poly1305"}`
pub async fn handle_purpose_encrypt(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.encrypt with purpose")?;

    let data_b64 = params
        .get("data")
        .and_then(|v| v.as_str())
        .or_else(|| params.get("plaintext").and_then(|v| v.as_str()))
        .ok_or("Missing required parameter: data (base64-encoded plaintext)")?;

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: purpose")?;

    let plaintext = BASE64
        .decode(data_b64)
        .map_err(|e| format!("Invalid base64 data: {e}"))?;

    let key = resolve_purpose_key(purpose)?;
    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(OsRng);

    // AEAD output = ciphertext || tag (standard NUCLEUS envelope format)
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_ref())
        .map_err(|e| format!("ChaCha20-Poly1305 encryption failed: {e}"))?;

    info!(
        "✅ Purpose-key encrypt: purpose='{}', {} bytes → {} bytes",
        purpose,
        plaintext.len(),
        ciphertext.len()
    );

    Ok(json!({
        "v": 1,
        "ct": BASE64.encode(&ciphertext),
        "n": BASE64.encode(nonce),
        "alg": "chacha20-poly1305",
        "purpose": purpose,
    }))
}

/// Decrypt a NUCLEUS purpose-key envelope.
///
/// Wire: `crypto.decrypt` with `purpose` param
///
/// # Parameters
///
/// Accepts either the NUCLEUS envelope or flat params:
/// - `purpose`: Purpose string (required)
/// - `ct` or `ciphertext`: Base64-encoded ciphertext
/// - `n` or `nonce`: Base64-encoded nonce
///
/// # Returns
///
/// - `plaintext`: Base64-encoded decrypted data
/// - `algorithm`: `"chacha20-poly1305"`
pub async fn handle_purpose_decrypt(params: Option<&Value>) -> Result<Value, String> {
    let params = params.ok_or("Missing params for crypto.decrypt with purpose")?;

    let purpose = params
        .get("purpose")
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: purpose")?;

    let ct_b64 = params
        .get("ct")
        .or_else(|| params.get("ciphertext"))
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: ct (base64-encoded ciphertext)")?;

    let nonce_b64 = params
        .get("n")
        .or_else(|| params.get("nonce"))
        .and_then(|v| v.as_str())
        .ok_or("Missing required parameter: n (base64-encoded nonce)")?;

    let ciphertext = BASE64
        .decode(ct_b64)
        .map_err(|e| format!("Invalid base64 ciphertext: {e}"))?;

    let nonce_bytes = BASE64
        .decode(nonce_b64)
        .map_err(|e| format!("Invalid base64 nonce: {e}"))?;

    if nonce_bytes.len() != 12 {
        return Err(format!(
            "Invalid nonce length: expected 12, got {}",
            nonce_bytes.len()
        ));
    }

    let key = resolve_purpose_key(purpose)?;
    let cipher = ChaCha20Poly1305::new(&key.into());
    let nonce = chacha20poly1305::Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("ChaCha20-Poly1305 decryption failed: {e}"))?;

    info!(
        "✅ Purpose-key decrypt: purpose='{}', {} bytes → {} bytes",
        purpose,
        ciphertext.len(),
        plaintext.len()
    );

    Ok(json!({
        "plaintext": BASE64.encode(&plaintext),
        "algorithm": "chacha20-poly1305",
        "purpose": purpose,
    }))
}

/// Compute a stable, non-reversible fingerprint of the primal's identity seed.
///
/// Wire: `crypto.seed_fingerprint`
///
/// The fingerprint is `BLAKE3(HMAC-SHA256(family_seed, "seed-fingerprint-v1"))`,
/// truncated to 16 bytes and hex-encoded. This allows Tower atomic validation
/// to verify that a primal's seed is consistent without exposing the seed itself.
///
/// # Parameters
///
/// - `algorithm` (optional): fingerprint hash algorithm, default `"blake3"`
///
/// # Returns
///
/// - `fingerprint`: hex-encoded seed fingerprint (32 chars)
/// - `algorithm`: `"BLAKE3-over-HMAC-SHA256"`
/// - `version`: `"seed-fingerprint-v1"`
///
/// # Errors
///
/// Returns an error if `FAMILY_SEED` / `BEARDOG_FAMILY_SEED` is not set.
pub async fn handle_seed_fingerprint(params: Option<&Value>) -> Result<Value, String> {
    let _ = params;

    let seed = load_family_seed()?;

    let hmac_output = hashing::hmac_sha256(&seed, b"seed-fingerprint-v1")
        .map_err(|e| format!("HMAC for seed fingerprint failed: {e}"))?;

    let fingerprint_full = blake3::hash(&hmac_output);
    let fingerprint_hex = hex::encode(&fingerprint_full.as_bytes()[..16]);

    info!("Seed fingerprint computed (32 hex chars)");

    Ok(json!({
        "fingerprint": fingerprint_hex,
        "algorithm": "BLAKE3-over-HMAC-SHA256",
        "version": "seed-fingerprint-v1",
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unix_socket_ipc::handlers::crypto_handler::aliases_and_beardog::route;
    use serial_test::serial;

    #[tokio::test]
    async fn derive_purpose_key_basic() {
        let family_key = BASE64.encode(b"test-family-key-32-bytes-long!!");
        let params = json!({ "key": family_key, "purpose": "storage" });
        let out = handle_derive_purpose_key(Some(&params))
            .await
            .expect("derive_purpose_key");
        assert_eq!(out["method"], "HMAC-SHA256-purpose-v1");
        assert_eq!(out["purpose"], "storage");
        let key_bytes = BASE64
            .decode(out["key"].as_str().expect("key"))
            .expect("valid base64");
        assert_eq!(key_bytes.len(), 32);
    }

    #[tokio::test]
    async fn derive_purpose_key_deterministic() {
        let family_key = BASE64.encode(b"deterministic-family-key-bytes!");
        let params = json!({ "key": family_key, "purpose": "dag" });
        let r1 = handle_derive_purpose_key(Some(&params))
            .await
            .expect("first");
        let r2 = handle_derive_purpose_key(Some(&params))
            .await
            .expect("second");
        assert_eq!(r1["key"], r2["key"]);
    }

    #[tokio::test]
    async fn derive_purpose_key_different_purposes_differ() {
        let family_key = BASE64.encode(b"shared-family-key-for-test!!!!!");
        let storage =
            handle_derive_purpose_key(Some(&json!({ "key": family_key, "purpose": "storage" })))
                .await
                .expect("storage");
        let dag = handle_derive_purpose_key(Some(&json!({ "key": family_key, "purpose": "dag" })))
            .await
            .expect("dag");
        assert_ne!(storage["key"], dag["key"]);
    }

    #[tokio::test]
    async fn sign_registration_basic() {
        let params = json!({
            "primal_id": "test-primal",
            "capabilities": ["tensor", "math", "stats"],
            "endpoint": "unix:///run/user/1000/biomeos/test.sock",
        });
        let out = handle_sign_registration(Some(&params))
            .await
            .expect("sign_registration");
        assert_eq!(out["algorithm"], "Ed25519");
        assert!(out["signature"].as_str().is_some());
        assert!(out["public_key"].as_str().is_some());
        let canonical = out["canonical"].as_str().expect("canonical");
        assert!(canonical.starts_with("ipc.register-v1:primal_id=test-primal,"));
        assert!(canonical.contains("math,stats,tensor"));
    }

    #[tokio::test]
    #[serial]
    async fn purpose_encrypt_decrypt_roundtrip() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "test-family-seed-for-purpose-keys");

        let enc = handle_purpose_encrypt(Some(&json!({
            "data": BASE64.encode(b"secret storage payload"),
            "purpose": "storage",
        })))
        .await
        .expect("purpose encrypt");

        assert_eq!(enc["v"], 1);
        assert_eq!(enc["alg"], "chacha20-poly1305");
        assert_eq!(enc["purpose"], "storage");

        let dec = handle_purpose_decrypt(Some(&json!({
            "ct": enc["ct"].as_str().expect("ct"),
            "n": enc["n"].as_str().expect("n"),
            "purpose": "storage",
        })))
        .await
        .expect("purpose decrypt");

        let plaintext = BASE64
            .decode(dec["plaintext"].as_str().expect("pt"))
            .expect("valid base64");
        assert_eq!(plaintext, b"secret storage payload");

        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn purpose_encrypt_different_purposes_differ() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "purpose-isolation-test-seed!!");

        let storage = handle_purpose_encrypt(Some(&json!({
            "data": BASE64.encode(b"same data"), "purpose": "storage"
        })))
        .await
        .expect("storage");

        let inference = handle_purpose_encrypt(Some(&json!({
            "data": BASE64.encode(b"same data"), "purpose": "inference"
        })))
        .await
        .expect("inference");

        assert_ne!(storage["ct"], inference["ct"]);

        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn purpose_encrypt_without_family_seed_fails() {
        beardog_errors::process_env::remove_var("FAMILY_SEED");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_SEED");

        let result = handle_purpose_encrypt(Some(&json!({
            "data": BASE64.encode(b"test"), "purpose": "storage"
        })))
        .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("FAMILY_SEED"));
    }

    #[tokio::test]
    #[serial]
    async fn purpose_routing_via_route_fn() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "routing-test-seed-material!!!!");

        let enc = route(
            "crypto.encrypt",
            Some(&json!({ "data": BASE64.encode(b"routed"), "purpose": "storage" })),
        )
        .await
        .expect("route")
        .expect("purpose encrypt via route");
        assert_eq!(enc["v"], 1);

        let dec = route(
            "crypto.decrypt",
            Some(&json!({
                "ct": enc["ct"].as_str().expect("ct"),
                "n": enc["n"].as_str().expect("n"),
                "purpose": "storage",
            })),
        )
        .await
        .expect("route")
        .expect("purpose decrypt via route");

        let pt = BASE64
            .decode(dec["plaintext"].as_str().expect("pt"))
            .expect("b64");
        assert_eq!(pt, b"routed");

        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn derive_public_key_basic() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "derive-pubkey-test-seed-material!");
        let params = json!({ "purpose": "coordination" });
        let out = handle_derive_public_key(Some(&params))
            .await
            .expect("derive_public_key");
        assert_eq!(out["algorithm"], "Ed25519");
        assert_eq!(out["purpose"], "coordination");
        assert_eq!(out["derivation"], "HMAC-SHA256-purpose-v1 → Ed25519");
        let pk_bytes = BASE64
            .decode(out["public_key"].as_str().expect("public_key"))
            .expect("valid base64");
        assert_eq!(pk_bytes.len(), 32);
        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn derive_public_key_deterministic() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "deterministic-pubkey-seed!!!!!!!!");
        let p = json!({ "purpose": "storage" });
        let r1 = handle_derive_public_key(Some(&p)).await.expect("first");
        let r2 = handle_derive_public_key(Some(&p)).await.expect("second");
        assert_eq!(r1["public_key"], r2["public_key"]);
        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn derive_public_key_different_purposes_differ() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "diffpurp-seed-material-for-test!!");
        let coordination = handle_derive_public_key(Some(&json!({ "purpose": "coordination" })))
            .await
            .expect("coordination");
        let storage = handle_derive_public_key(Some(&json!({ "purpose": "storage" })))
            .await
            .expect("storage");
        assert_ne!(coordination["public_key"], storage["public_key"]);
        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn derive_public_key_no_seed_fails() {
        beardog_errors::process_env::remove_var("FAMILY_SEED");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_SEED");
        let err = handle_derive_public_key(Some(&json!({ "purpose": "coordination" })))
            .await
            .expect_err("should fail without seed");
        assert!(err.contains("FAMILY_SEED"));
    }

    #[tokio::test]
    async fn derive_public_key_missing_purpose_fails() {
        let err = handle_derive_public_key(Some(&json!({})))
            .await
            .expect_err("should fail without purpose");
        assert!(err.contains("purpose"));
    }

    #[tokio::test]
    async fn derive_public_key_missing_params_fails() {
        let err = handle_derive_public_key(None)
            .await
            .expect_err("should fail without params");
        assert!(err.contains("Missing params"));
    }

    #[tokio::test]
    #[serial]
    async fn derive_public_key_routes_via_aliases() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "routing-pubkey-test-seed-material!");
        let out = route(
            "crypto.derive_public_key",
            Some(&json!({ "purpose": "coordination" })),
        )
        .await
        .expect("route")
        .expect("derive_public_key via route");
        assert_eq!(out["algorithm"], "Ed25519");
        assert_eq!(out["purpose"], "coordination");
        let pk_bytes = BASE64
            .decode(out["public_key"].as_str().expect("pk"))
            .expect("b64");
        assert_eq!(pk_bytes.len(), 32);
        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    // ── crypto.seed_fingerprint tests ────────────────────────────────────

    #[tokio::test]
    #[serial]
    async fn seed_fingerprint_returns_valid_hex() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "fingerprint-test-seed!!");
        let out = handle_seed_fingerprint(None).await.expect("fingerprint");
        let fp = out["fingerprint"].as_str().expect("fingerprint");
        assert_eq!(fp.len(), 32, "16 bytes = 32 hex chars");
        assert!(fp.chars().all(|c| c.is_ascii_hexdigit()));
        assert_eq!(out["algorithm"], "BLAKE3-over-HMAC-SHA256");
        assert_eq!(out["version"], "seed-fingerprint-v1");
        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn seed_fingerprint_is_deterministic() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "determinism-test-seed!!");
        let fp1 = handle_seed_fingerprint(None).await.expect("1")["fingerprint"]
            .as_str()
            .expect("fp")
            .to_owned();
        let fp2 = handle_seed_fingerprint(None).await.expect("2")["fingerprint"]
            .as_str()
            .expect("fp")
            .to_owned();
        assert_eq!(fp1, fp2);
        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn seed_fingerprint_differs_per_seed() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "seed-alpha-for-test!!!");
        let fp_a = handle_seed_fingerprint(None).await.expect("a")["fingerprint"]
            .as_str()
            .expect("fp")
            .to_owned();
        beardog_errors::process_env::set_var("FAMILY_SEED", "seed-bravo-for-test!!!");
        let fp_b = handle_seed_fingerprint(None).await.expect("b")["fingerprint"]
            .as_str()
            .expect("fp")
            .to_owned();
        assert_ne!(fp_a, fp_b);
        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }

    #[tokio::test]
    #[serial]
    async fn seed_fingerprint_fails_without_seed() {
        beardog_errors::process_env::remove_var("FAMILY_SEED");
        beardog_errors::process_env::remove_var("BEARDOG_FAMILY_SEED");
        let err = handle_seed_fingerprint(None).await.expect_err("no seed");
        assert!(err.contains("FAMILY_SEED"));
    }

    #[tokio::test]
    #[serial]
    async fn seed_fingerprint_routes_via_aliases() {
        beardog_errors::process_env::set_var("FAMILY_SEED", "route-fp-test-seed-material!");
        let out = route("crypto.seed_fingerprint", None)
            .await
            .expect("route")
            .expect("fingerprint via route");
        assert_eq!(out["algorithm"], "BLAKE3-over-HMAC-SHA256");
        let fp = out["fingerprint"].as_str().expect("fp");
        assert_eq!(fp.len(), 32);
        beardog_errors::process_env::remove_var("FAMILY_SEED");
    }
}
