// SPDX-License-Identifier: AGPL-3.0-or-later

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
