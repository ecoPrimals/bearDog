// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;
use base64::engine::general_purpose::STANDARD as BASE64;
use serde_json::json;

// ========================================================================
// ED25519 SIGNATURE TESTS
// ========================================================================

#[tokio::test]
async fn test_ed25519_sign_basic() {
    let message = BASE64.encode(b"Hello, BearDog!");
    let params = json!({ "message": message });

    let result = handle_sign_ed25519(Some(&params)).await;
    assert!(result.is_ok());

    let value = result.expect("ed25519 sign should succeed");
    assert_eq!(value["algorithm"], "Ed25519");
    assert!(value["signature"].is_string());
    assert!(value["key_id"].is_string());
    assert!(value["public_key"].is_string());

    let sig_str = value["signature"]
        .as_str()
        .expect("signature should be a string");
    let signature = BASE64
        .decode(sig_str)
        .expect("signature should be valid base64");
    assert_eq!(signature.len(), 64);

    let pk_str = value["public_key"]
        .as_str()
        .expect("public_key should be a string");
    let pk = BASE64
        .decode(pk_str)
        .expect("public_key should be valid base64");
    assert_eq!(pk.len(), 32);
}

#[tokio::test]
async fn test_ed25519_sign_with_key_id() {
    let message = BASE64.encode(b"Signed message");
    let params = json!({ "message": message, "key_id": "my-custom-key" });

    let result = handle_sign_ed25519(Some(&params)).await;
    assert!(result.is_ok());

    let value = result.expect("ed25519 sign with key_id should succeed");
    assert_eq!(value["key_id"], "my-custom-key");
}

#[tokio::test]
async fn test_ed25519_sign_deterministic_same_key() {
    let message = BASE64.encode(b"Same message");
    let params = json!({ "message": message, "key_id": "test-key-1" });

    let result1 = handle_sign_ed25519(Some(&params))
        .await
        .expect("first deterministic sign should succeed");
    let result2 = handle_sign_ed25519(Some(&params))
        .await
        .expect("second deterministic sign should succeed");

    // Same key, same message = same signature
    assert_eq!(result1["signature"], result2["signature"]);
}

#[tokio::test]
async fn test_ed25519_sign_different_keys_different_signatures() {
    let message = BASE64.encode(b"Same message");

    let params1 = json!({ "message": message, "key_id": "key-a" });
    let params2 = json!({ "message": message, "key_id": "key-b" });

    let result1 = handle_sign_ed25519(Some(&params1))
        .await
        .expect("sign with key-a should succeed");
    let result2 = handle_sign_ed25519(Some(&params2))
        .await
        .expect("sign with key-b should succeed");

    // Different keys = different signatures
    assert_ne!(result1["signature"], result2["signature"]);
}

#[tokio::test]
async fn test_ed25519_sign_missing_message() {
    let params = json!({ "key_id": "some-key" });
    let result = handle_sign_ed25519(Some(&params)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("message"));
}

#[tokio::test]
async fn test_ed25519_sign_invalid_base64() {
    let params = json!({ "message": "!!!invalid-base64!!!" });
    let result = handle_sign_ed25519(Some(&params)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Invalid base64"));
}

// ========================================================================
// ED25519 VERIFICATION TESTS
// ========================================================================

#[tokio::test]
async fn test_ed25519_verify_missing_params() {
    let message = BASE64.encode(b"msg");
    let signature = BASE64.encode(&[0u8; 64]);

    let params = json!({ "message": message, "signature": signature });
    let result = handle_verify_ed25519(Some(&params)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("public_key"));
}

#[tokio::test]
async fn test_ed25519_verify_missing_signature() {
    let message = BASE64.encode(b"msg");
    let public_key = BASE64.encode(&[0u8; 32]);

    let params = json!({ "message": message, "public_key": public_key });
    let result = handle_verify_ed25519(Some(&params)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("signature"));
}

// ========================================================================
// X25519 KEY EXCHANGE TESTS
// ========================================================================

#[tokio::test]
async fn test_x25519_generate_ephemeral() {
    let result = handle_x25519_generate_ephemeral(None).await;
    assert!(result.is_ok());

    let value = result.expect("x25519 ephemeral should succeed");
    assert_eq!(value["algorithm"], "X25519");
    assert!(value["public_key"].is_string());
    assert!(value["secret_key"].is_string());

    // Verify key sizes (32 bytes each)
    let pk_str = value["public_key"]
        .as_str()
        .expect("public_key should be a string");
    let sk_str = value["secret_key"]
        .as_str()
        .expect("secret_key should be a string");
    let public_key = BASE64.decode(pk_str).expect("public_key base64");
    let secret_key = BASE64.decode(sk_str).expect("secret_key base64");
    assert_eq!(public_key.len(), 32);
    assert_eq!(secret_key.len(), 32);
}

#[tokio::test]
async fn test_x25519_generate_unique_keys() {
    let result1 = handle_x25519_generate_ephemeral(None)
        .await
        .expect("first x25519 ephemeral");
    let result2 = handle_x25519_generate_ephemeral(None)
        .await
        .expect("second x25519 ephemeral");

    // Each generation should produce unique keys
    assert_ne!(result1["public_key"], result2["public_key"]);
    assert_ne!(result1["secret_key"], result2["secret_key"]);
}

#[tokio::test]
async fn test_x25519_key_exchange_roundtrip() {
    // Alice generates her keypair
    let alice_keypair = handle_x25519_generate_ephemeral(None)
        .await
        .expect("alice x25519 keypair");

    // Bob generates his keypair
    let bob_keypair = handle_x25519_generate_ephemeral(None)
        .await
        .expect("bob x25519 keypair");

    // Alice derives shared secret with Bob's public key
    let alice_params = json!({
        "our_secret": alice_keypair["secret_key"],
        "their_public": bob_keypair["public_key"]
    });
    let alice_secret = handle_x25519_derive_secret(Some(&alice_params))
        .await
        .expect("Alice derive failed");

    // Bob derives shared secret with Alice's public key
    let bob_params = json!({
        "our_secret": bob_keypair["secret_key"],
        "their_public": alice_keypair["public_key"]
    });
    let bob_secret = handle_x25519_derive_secret(Some(&bob_params))
        .await
        .expect("Bob derive failed");

    // Both should arrive at the same shared secret
    assert_eq!(alice_secret["shared_secret"], bob_secret["shared_secret"]);
    assert_eq!(alice_secret["algorithm"], "X25519");
}

#[tokio::test]
async fn test_x25519_derive_missing_our_secret() {
    let their_public = BASE64.encode(&[0u8; 32]);
    let params = json!({ "their_public": their_public });

    let result = handle_x25519_derive_secret(Some(&params)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("our_secret"));
}

#[tokio::test]
async fn test_x25519_derive_missing_their_public() {
    let our_secret = BASE64.encode(&[0u8; 32]);
    let params = json!({ "our_secret": our_secret });

    let result = handle_x25519_derive_secret(Some(&params)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("their_public"));
}

#[tokio::test]
async fn test_x25519_derive_wrong_key_length() {
    let our_secret = BASE64.encode(&[0u8; 16]); // Wrong: 16 bytes
    let their_public = BASE64.encode(&[0u8; 32]);
    let params = json!({ "our_secret": our_secret, "their_public": their_public });

    let result = handle_x25519_derive_secret(Some(&params)).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("32 bytes"));
}

#[tokio::test]
async fn test_sign_ed25519_missing_params() {
    let r = handle_sign_ed25519(None).await;
    assert!(r.is_err());
}

#[tokio::test]
async fn test_verify_ed25519_invalid_message_base64() {
    let params = json!({
        "message": "@@@",
        "signature": BASE64.encode(&[1u8; 64]),
        "public_key": BASE64.encode(&[2u8; 32]),
    });
    let r = handle_verify_ed25519(Some(&params)).await;
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("base64"));
}

#[tokio::test]
async fn test_verify_ed25519_invalid_signature_base64() {
    let params = json!({
        "message": BASE64.encode(b"m"),
        "signature": "not-b64",
        "public_key": BASE64.encode(&[2u8; 32]),
    });
    let r = handle_verify_ed25519(Some(&params)).await;
    assert!(r.is_err());
}

#[tokio::test]
async fn test_verify_ed25519_invalid_public_key_base64() {
    let params = json!({
        "message": BASE64.encode(b"m"),
        "signature": BASE64.encode(&[1u8; 64]),
        "public_key": "???",
    });
    let r = handle_verify_ed25519(Some(&params)).await;
    assert!(r.is_err());
}

#[tokio::test]
async fn test_x25519_derive_invalid_base64_secret() {
    let params = json!({
        "our_secret": "bad",
        "their_public": BASE64.encode(&[0u8; 32]),
    });
    let r = handle_x25519_derive_secret(Some(&params)).await;
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("our_secret"));
}

#[tokio::test]
async fn test_ed25519_generate_keypair_with_purpose() {
    let params = json!({ "purpose": "birdsong" });
    let r = handle_ed25519_generate_keypair(Some(&params))
        .await
        .expect("kp");
    assert_eq!(r["algorithm"], "Ed25519");
}

#[tokio::test]
async fn test_ed25519_sign_verify_roundtrip() {
    let msg = BASE64.encode(b"roundtrip");
    let sign_p = json!({ "message": msg, "key_id": "rt-key", "purpose": "t" });
    let signed = handle_sign_ed25519(Some(&sign_p)).await.expect("sign");
    let sig = signed["signature"].as_str().expect("sig");
    let pk = signed["public_key"]
        .as_str()
        .expect("public_key in sign response");
    let verify_p = json!({
        "message": msg,
        "signature": sig,
        "public_key": pk,
    });
    let v = handle_verify_ed25519(Some(&verify_p))
        .await
        .expect("verify");
    assert_eq!(
        v["valid"], true,
        "sign→verify roundtrip via IPC response fields"
    );
}

// ========================================================================
// PUBLIC KEY RETRIEVAL TESTS
// ========================================================================

#[tokio::test]
async fn test_public_key_default() {
    let r = handle_public_key(None).await.expect("public_key");
    assert_eq!(r["algorithm"], "Ed25519");
    assert_eq!(r["key_id"], "default_signing_key");
    let pk = BASE64
        .decode(r["public_key"].as_str().expect("pk str"))
        .expect("valid base64");
    assert_eq!(pk.len(), 32);
}

#[tokio::test]
async fn test_public_key_matches_sign() {
    let params = json!({ "key_id": "pk-match-test", "purpose": "general" });
    let pk_resp = handle_public_key(Some(&params)).await.expect("pk");
    let pk_b64 = pk_resp["public_key"].as_str().expect("pk str");

    let msg = BASE64.encode(b"pk match");
    let sign_p = json!({ "message": msg, "key_id": "pk-match-test", "purpose": "general" });
    let signed = handle_sign_ed25519(Some(&sign_p)).await.expect("sign");
    assert_eq!(signed["public_key"].as_str().expect("sign pk"), pk_b64);

    let verify_p = json!({
        "message": msg,
        "signature": signed["signature"].as_str().expect("sig"),
        "public_key": pk_b64,
    });
    let v = handle_verify_ed25519(Some(&verify_p))
        .await
        .expect("verify");
    assert_eq!(
        v["valid"], true,
        "public_key from crypto.public_key must verify signatures from crypto.sign"
    );
}

// ========================================================================
// BD-01: ENCODING HINT TESTS (WireWitnessRef compatibility)
// ========================================================================

fn to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

#[tokio::test]
async fn test_verify_ed25519_encoding_default_is_base64() {
    use beardog_core::crypto_service::algorithms::asymmetric;
    let msg_bytes = b"default encoding";
    let seed = super::super::utils::derive_key_from_id("enc-default", "t").expect("seed");
    let (sk, pk) = asymmetric::generate_ed25519_from_seed(&seed).expect("kp");
    let sig = asymmetric::sign_ed25519(msg_bytes, &sk).expect("sign");

    let params = json!({
        "message": BASE64.encode(msg_bytes),
        "signature": BASE64.encode(&sig),
        "public_key": BASE64.encode(pk),
    });
    let v = handle_verify_ed25519(Some(&params)).await.expect("verify");
    assert_eq!(v["valid"], true);
}

#[tokio::test]
async fn test_verify_ed25519_encoding_hex() {
    use beardog_core::crypto_service::algorithms::asymmetric;
    let msg_bytes = b"hex witness evidence";
    let seed = super::super::utils::derive_key_from_id("enc-hex", "t").expect("seed");
    let (sk, pk) = asymmetric::generate_ed25519_from_seed(&seed).expect("kp");
    let sig = asymmetric::sign_ed25519(msg_bytes, &sk).expect("sign");

    let params = json!({
        "message": to_hex(msg_bytes),
        "signature": to_hex(&sig),
        "public_key": to_hex(&pk),
        "encoding": "hex",
    });
    let v = handle_verify_ed25519(Some(&params))
        .await
        .expect("verify with hex");
    assert_eq!(v["valid"], true);
}

#[tokio::test]
async fn test_verify_ed25519_encoding_hex_0x_prefix() {
    use beardog_core::crypto_service::algorithms::asymmetric;
    let msg_bytes = b"0x prefix";
    let seed = super::super::utils::derive_key_from_id("enc-hex-0x", "t").expect("seed");
    let (sk, pk) = asymmetric::generate_ed25519_from_seed(&seed).expect("kp");
    let sig = asymmetric::sign_ed25519(msg_bytes, &sk).expect("sign");

    let params = json!({
        "message": format!("0x{}", to_hex(msg_bytes)),
        "signature": format!("0x{}", to_hex(&sig)),
        "public_key": format!("0x{}", to_hex(&pk)),
        "encoding": "hex",
    });
    let v = handle_verify_ed25519(Some(&params))
        .await
        .expect("verify with 0x hex");
    assert_eq!(v["valid"], true);
}

#[tokio::test]
async fn test_verify_ed25519_encoding_base64url() {
    use base64::engine::general_purpose::URL_SAFE;
    use beardog_core::crypto_service::algorithms::asymmetric;
    let msg_bytes = b"base64url witness";
    let seed = super::super::utils::derive_key_from_id("enc-b64url", "t").expect("seed");
    let (sk, pk) = asymmetric::generate_ed25519_from_seed(&seed).expect("kp");
    let sig = asymmetric::sign_ed25519(msg_bytes, &sk).expect("sign");

    let params = json!({
        "message": URL_SAFE.encode(msg_bytes),
        "signature": URL_SAFE.encode(&sig),
        "public_key": URL_SAFE.encode(pk),
        "encoding": "base64url",
    });
    let v = handle_verify_ed25519(Some(&params))
        .await
        .expect("verify with base64url");
    assert_eq!(v["valid"], true);
}

#[tokio::test]
async fn test_verify_ed25519_encoding_unsupported() {
    let params = json!({
        "message": "whatever",
        "signature": "whatever",
        "public_key": "whatever",
        "encoding": "brotli",
    });
    let r = handle_verify_ed25519(Some(&params)).await;
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("Unsupported encoding"));
}

#[tokio::test]
async fn test_verify_ed25519_encoding_invalid_hex() {
    let params = json!({
        "message": "zzzz",
        "signature": BASE64.encode(&[0u8; 64]),
        "public_key": BASE64.encode(&[0u8; 32]),
        "encoding": "hex",
    });
    let r = handle_verify_ed25519(Some(&params)).await;
    assert!(r.is_err());
    assert!(r.unwrap_err().contains("hex"));
}

#[tokio::test]
async fn test_verify_ed25519_backwards_compat_no_encoding_field() {
    use beardog_core::crypto_service::algorithms::asymmetric;
    let msg_bytes = b"no encoding field";
    let seed = super::super::utils::derive_key_from_id("compat", "t").expect("seed");
    let (sk, pk) = asymmetric::generate_ed25519_from_seed(&seed).expect("kp");
    let sig = asymmetric::sign_ed25519(msg_bytes, &sk).expect("sign");

    let params = json!({
        "message": BASE64.encode(msg_bytes),
        "signature": BASE64.encode(&sig),
        "public_key": BASE64.encode(pk),
    });
    let v = handle_verify_ed25519(Some(&params))
        .await
        .expect("verify without encoding");
    assert_eq!(
        v["valid"], true,
        "must stay backwards-compatible with base64-only callers"
    );
}

// ========================================================================
// BD-01 RESOLUTION: PER-FIELD ENCODING HINT TESTS
// ========================================================================

fn to_hex_local(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

#[tokio::test]
async fn test_verify_ed25519_per_field_encoding_mixed() {
    use base64::engine::general_purpose::URL_SAFE;
    use beardog_core::crypto_service::algorithms::asymmetric;

    let msg_bytes = b"mixed encoding witness";
    let seed = super::super::utils::derive_key_from_id("per-field-mix", "t").expect("seed");
    let (sk, pk) = asymmetric::generate_ed25519_from_seed(&seed).expect("kp");
    let sig = asymmetric::sign_ed25519(msg_bytes, &sk).expect("sign");

    let params = json!({
        "message": BASE64.encode(msg_bytes),
        "signature": to_hex_local(&sig),
        "public_key": URL_SAFE.encode(pk),
        "message_encoding": "base64",
        "signature_encoding": "hex",
        "public_key_encoding": "base64url",
    });
    let v = handle_verify_ed25519(Some(&params))
        .await
        .expect("per-field mixed encoding verify");
    assert_eq!(
        v["valid"], true,
        "per-field encoding must decode each field independently"
    );
}

#[tokio::test]
async fn test_verify_ed25519_per_field_overrides_default() {
    use beardog_core::crypto_service::algorithms::asymmetric;

    let msg_bytes = b"override default";
    let seed = super::super::utils::derive_key_from_id("per-field-override", "t").expect("seed");
    let (sk, pk) = asymmetric::generate_ed25519_from_seed(&seed).expect("kp");
    let sig = asymmetric::sign_ed25519(msg_bytes, &sk).expect("sign");

    let params = json!({
        "message": to_hex_local(msg_bytes),
        "signature": BASE64.encode(&sig),
        "public_key": BASE64.encode(pk),
        "encoding": "base64",
        "message_encoding": "hex",
    });
    let v = handle_verify_ed25519(Some(&params))
        .await
        .expect("per-field override");
    assert_eq!(
        v["valid"], true,
        "message_encoding=hex should override encoding=base64 for message only"
    );
}

#[tokio::test]
async fn test_verify_ed25519_per_field_invalid_encoding_error() {
    let params = json!({
        "message": "whatever",
        "signature": "whatever",
        "public_key": "whatever",
        "encoding": "base64",
        "signature_encoding": "brotli",
    });
    let r = handle_verify_ed25519(Some(&params)).await;
    assert!(r.is_err());
    assert!(
        r.unwrap_err().contains("Unsupported encoding"),
        "per-field encoding error should surface"
    );
}

// ========================================================================
// DID:KEY DERIVATION TESTS
// ========================================================================

#[tokio::test]
async fn test_did_from_key_basic() {
    let result = handle_did_from_key(None).await;
    assert!(result.is_ok());

    let value = result.expect("did_from_key should succeed");
    let did = value["did"].as_str().expect("did should be a string");

    assert!(
        did.starts_with("did:key:z6Mk"),
        "Ed25519 did:key should start with z6Mk"
    );
    assert_eq!(value["algorithm"], "Ed25519");
    assert_eq!(value["key_id"], "default_signing_key");
    assert!(value["public_key"].is_string());
}

#[tokio::test]
async fn test_did_from_key_deterministic() {
    let params = json!({ "key_id": "test_key", "purpose": "signing" });

    let r1 = handle_did_from_key(Some(&params))
        .await
        .expect("first call");
    let r2 = handle_did_from_key(Some(&params))
        .await
        .expect("second call");

    assert_eq!(
        r1["did"], r2["did"],
        "same key_id+purpose must produce same DID"
    );
    assert_eq!(r1["public_key"], r2["public_key"]);
}

#[tokio::test]
async fn test_did_from_key_different_purposes_produce_different_dids() {
    let p1 = json!({ "key_id": "k", "purpose": "signing" });
    let p2 = json!({ "key_id": "k", "purpose": "auth" });

    let r1 = handle_did_from_key(Some(&p1))
        .await
        .expect("purpose=signing");
    let r2 = handle_did_from_key(Some(&p2)).await.expect("purpose=auth");

    assert_ne!(
        r1["did"], r2["did"],
        "different purposes should produce different DIDs"
    );
}

#[tokio::test]
async fn test_did_from_key_matches_public_key_output() {
    let params = json!({ "key_id": "cross_check" });

    let did_result = handle_did_from_key(Some(&params))
        .await
        .expect("did_from_key");
    let pk_result = handle_public_key(Some(&params)).await.expect("public_key");

    assert_eq!(
        did_result["public_key"], pk_result["public_key"],
        "did_from_key and public_key should return the same public key"
    );
}
