// SPDX-License-Identifier: AGPL-3.0-only

use crate::handlers::key_store;
use crate::handlers::key_store::StoredKey;

use super::crypto::{decrypt_key_material, encrypt_key_material};
use super::export::handle_key_export_with_home;
use super::import::handle_key_import_with_home;
use super::types::ExportedKey;

use chrono::Utc;
use tempfile::TempDir;

#[tokio::test]
async fn test_export_import_with_home_roundtrip() {
    let src = TempDir::new().expect("create temp dir for test");
    let dst = TempDir::new().expect("create temp dir for test");
    let key = StoredKey {
        key_id: "export-key-1".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        hsm_name: "soft".to_string(),
        key_material_b64: key_store::base64_encode(b"01234567890123456789012345678901"),
        created_at: Utc::now().to_rfc3339(),
        generation: 0,
        parent_key_id: None,
        derivation_purpose: None,
        children: vec![],
        lineage: None,
        expires_at: None,
        usage: None,
        purpose: Some("unit-test".to_string()),
    };
    key_store::save_key_to_home(&key, src.path()).expect("save test key to store");

    let out = src.path().join("exported.json");
    handle_key_export_with_home(
        "export-key-1",
        out.to_str().expect("export path is valid UTF-8"),
        false,
        src.path(),
    )
    .await
    .expect("export key in test");

    handle_key_import_with_home(
        out.to_str().expect("export path is valid UTF-8"),
        None,
        false,
        false,
        dst.path(),
    )
    .await
    .expect("import key in test");

    let loaded = key_store::load_key_from_home("export-key-1", dst.path())
        .expect("load imported key in test");
    assert_eq!(loaded.algorithm, key.algorithm);
    assert_eq!(loaded.key_material_b64, key.key_material_b64);
}

#[tokio::test]
async fn test_import_with_home_override_id() {
    let src = TempDir::new().expect("create temp dir for test");
    let dst = TempDir::new().expect("create temp dir for test");
    let key = StoredKey {
        key_id: "orig-id".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        hsm_name: "soft".to_string(),
        key_material_b64: key_store::base64_encode(b"01234567890123456789012345678901"),
        created_at: Utc::now().to_rfc3339(),
        generation: 0,
        parent_key_id: None,
        derivation_purpose: None,
        children: vec![],
        lineage: None,
        expires_at: None,
        usage: None,
        purpose: None,
    };
    key_store::save_key_to_home(&key, src.path()).expect("save test key to store");
    let out = src.path().join("exported.json");
    handle_key_export_with_home(
        "orig-id",
        out.to_str().expect("export path is valid UTF-8"),
        false,
        src.path(),
    )
    .await
    .expect("export key in test");

    handle_key_import_with_home(
        out.to_str().expect("export path is valid UTF-8"),
        Some("renamed-id"),
        false,
        false,
        dst.path(),
    )
    .await
    .expect("import with key id override in test");

    assert!(key_store::load_key_from_home("renamed-id", dst.path()).is_ok());
}

#[tokio::test]
async fn test_import_with_home_invalid_json_fails() {
    let dir = TempDir::new().expect("create temp dir for test");
    let bad = dir.path().join("bad.json");
    std::fs::write(&bad, "{").expect("write invalid JSON fixture");
    assert!(
        handle_key_import_with_home(
            bad.to_str().expect("bad.json path is valid UTF-8"),
            None,
            false,
            false,
            dir.path(),
        )
        .await
        .is_err()
    );
}

#[tokio::test]
async fn test_import_with_home_encrypted_without_decrypt_flag() {
    let dir = TempDir::new().expect("create temp dir for test");
    let p = dir.path().join("enc.json");
    let exported = ExportedKey {
        key_id: "k".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        parent: None,
        generation: 0,
        created_at: "t".to_string(),
        context: None,
        expires_at: None,
        usage: None,
        purpose: None,
        metadata: std::collections::HashMap::new(),
        key_material: "x".to_string(),
        encrypted: true,
        version: "1.0".to_string(),
    };
    std::fs::write(
        &p,
        serde_json::to_string(&exported).expect("serialize encrypted export fixture"),
    )
    .expect("write encrypted export fixture");
    let r = handle_key_import_with_home(
        p.to_str().expect("enc.json path is valid UTF-8"),
        None,
        false,
        false,
        dir.path(),
    )
    .await;
    assert!(r.is_err());
}

#[test]
fn test_encrypt_decrypt_key_material() {
    let original = "SGVsbG8gV29ybGQh"; // "Hello World!" in base64
    let password = "test-password-123";

    // Encrypt
    let encrypted = encrypt_key_material(original, password).expect("encrypt test key material");
    assert_ne!(encrypted, original);
    assert!(encrypted.contains("salt"));
    assert!(encrypted.contains("nonce"));
    assert!(encrypted.contains("ciphertext"));

    // Decrypt
    let decrypted = decrypt_key_material(&encrypted, password).expect("decrypt test key material");
    assert_eq!(decrypted, original);
}

#[test]
fn test_decrypt_wrong_password_fails() {
    let original = "SGVsbG8gV29ybGQh";
    let password = "correct-password";
    let wrong_password = "wrong-password";

    let encrypted =
        encrypt_key_material(original, password).expect("encrypt for wrong-password test");
    let result = decrypt_key_material(&encrypted, wrong_password);

    assert!(result.is_err());
}

#[test]
fn test_exported_key_serialization() {
    let exported = ExportedKey {
        key_id: "test-key".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        parent: Some("master-key".to_string()),
        generation: 1,
        created_at: "2025-12-18T00:00:00Z".to_string(),
        context: Some("student-1".to_string()),
        expires_at: None,
        usage: Some("all".to_string()),
        purpose: Some("testing".to_string()),
        metadata: std::collections::HashMap::new(),
        key_material: "base64encodedkey".to_string(),
        encrypted: false,
        version: "1.0".to_string(),
    };

    let json = serde_json::to_string(&exported).expect("serialize ExportedKey in test");
    let deserialized: ExportedKey =
        serde_json::from_str(&json).expect("round-trip ExportedKey JSON in test");

    assert_eq!(deserialized.key_id, "test-key");
    assert_eq!(deserialized.algorithm, "aes-256-gcm");
    assert_eq!(deserialized.generation, 1);
}

#[test]
fn test_exported_key_serde_default_version() {
    let json = r#"{"key_id":"k","algorithm":"aes-256-gcm","generation":0,"created_at":"t","key_material":"e30="}"#;
    let e: ExportedKey = serde_json::from_str(json).expect("parse ExportedKey with serde defaults");
    assert_eq!(e.version, "1.0");
    assert!(!e.encrypted);
}

#[test]
fn test_decrypt_key_material_invalid_json() {
    let r = decrypt_key_material("not json", "pw");
    assert!(r.is_err());
}

#[test]
fn test_decrypt_key_material_missing_fields() {
    let r = decrypt_key_material(r#"{"salt":"x"}"#, "pw");
    assert!(r.is_err());
}

#[test]
fn test_decrypt_key_material_invalid_salt_b64() {
    let bad = r#"{"salt":"not-valid-salt-string","nonce":"AAAA","ciphertext":"AAAA"}"#;
    assert!(decrypt_key_material(bad, "pw").is_err());
}

#[test]
fn test_encrypt_key_material_invalid_base64() {
    assert!(encrypt_key_material("@@@not-base64@@@", "pw").is_err());
}

#[test]
fn test_decrypt_key_material_invalid_nonce_length() {
    use base64::Engine;
    use base64::engine::general_purpose::STANDARD;
    let enc = encrypt_key_material("e30=", "pw").expect("encrypt minimal payload for test");
    let mut v: serde_json::Value =
        serde_json::from_str(&enc).expect("parse encrypted package JSON in test");
    v["nonce"] = serde_json::Value::String(STANDARD.encode([1u8, 2, 3]));
    let bad = v.to_string();
    assert!(decrypt_key_material(&bad, "pw").is_err());
}

#[test]
fn test_decrypt_key_material_invalid_ciphertext_b64() {
    let enc = encrypt_key_material("e30=", "pw").expect("encrypt minimal payload for test");
    let mut v: serde_json::Value =
        serde_json::from_str(&enc).expect("parse encrypted package JSON in test");
    v["ciphertext"] = serde_json::Value::String("not-valid-b64!!!".to_string());
    assert!(decrypt_key_material(&v.to_string(), "pw").is_err());
}

#[tokio::test]
async fn test_export_with_home_includes_parent_in_output() {
    let src = TempDir::new().expect("create temp dir for test");
    let key = StoredKey {
        key_id: "with-parent".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        hsm_name: "soft".to_string(),
        key_material_b64: key_store::base64_encode(b"01234567890123456789012345678901"),
        created_at: Utc::now().to_rfc3339(),
        generation: 1,
        parent_key_id: Some("master-x".to_string()),
        derivation_purpose: Some("ctx".to_string()),
        children: vec![],
        lineage: None,
        expires_at: None,
        usage: None,
        purpose: Some("p".to_string()),
    };
    key_store::save_key_to_home(&key, src.path()).expect("save test key to store");
    let out = src.path().join("out.json");
    handle_key_export_with_home(
        "with-parent",
        out.to_str().expect("export output path is valid UTF-8"),
        false,
        src.path(),
    )
    .await
    .expect("export with parent in test");
    let json = std::fs::read_to_string(&out).expect("read exported JSON in test");
    assert!(json.contains("master-x"));
}

#[tokio::test]
async fn test_import_with_home_version_mismatch_warns_and_imports() {
    let dir = TempDir::new().expect("create temp dir for test");
    let p = dir.path().join("v2.json");
    let exported = ExportedKey {
        key_id: "ver-key".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        parent: None,
        generation: 0,
        created_at: Utc::now().to_rfc3339(),
        context: None,
        expires_at: None,
        usage: None,
        purpose: None,
        metadata: std::collections::HashMap::new(),
        key_material: key_store::base64_encode(b"01234567890123456789012345678901"),
        encrypted: false,
        version: "2.0".to_string(),
    };
    std::fs::write(
        &p,
        serde_json::to_string_pretty(&exported).expect("serialize version-mismatch fixture"),
    )
    .expect("write version-mismatch export file");
    handle_key_import_with_home(
        p.to_str().expect("v2.json path is valid UTF-8"),
        None,
        false,
        false,
        dir.path(),
    )
    .await
    .expect("import version-mismatch key in test");
    let loaded = key_store::load_key_from_home("ver-key", dir.path())
        .expect("load imported ver-key in test");
    assert_eq!(loaded.algorithm, "aes-256-gcm");
}

#[tokio::test]
async fn test_import_with_home_allow_overwrite_replaces_existing() {
    let dir = TempDir::new().expect("create temp dir for test");
    let old = StoredKey {
        key_id: "dup".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        hsm_name: "a".to_string(),
        key_material_b64: key_store::base64_encode(b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"),
        created_at: Utc::now().to_rfc3339(),
        generation: 0,
        parent_key_id: None,
        derivation_purpose: None,
        children: vec![],
        lineage: None,
        expires_at: None,
        usage: None,
        purpose: None,
    };
    key_store::save_key_to_home(&old, dir.path()).expect("save pre-existing dup key");

    let p = dir.path().join("new.json");
    let exported = ExportedKey {
        key_id: "dup".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        parent: Some("root".to_string()),
        generation: 1,
        created_at: Utc::now().to_rfc3339(),
        context: None,
        expires_at: None,
        usage: None,
        purpose: Some("imported".to_string()),
        metadata: std::collections::HashMap::new(),
        key_material: key_store::base64_encode(b"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"),
        encrypted: false,
        version: "1.0".to_string(),
    };
    std::fs::write(
        &p,
        serde_json::to_string_pretty(&exported).expect("serialize overwrite import fixture"),
    )
    .expect("write overwrite import file");

    handle_key_import_with_home(
        p.to_str().expect("new.json path is valid UTF-8"),
        None,
        false,
        true,
        dir.path(),
    )
    .await
    .expect("import with allow_overwrite in test");
    let loaded =
        key_store::load_key_from_home("dup", dir.path()).expect("load overwritten dup key");
    assert_eq!(loaded.generation, 1);
    assert_eq!(loaded.parent_key_id.as_deref(), Some("root"));
    assert_eq!(loaded.key_material_b64, exported.key_material);
}

#[tokio::test]
async fn test_import_with_home_parent_triggers_lineage_next_step_message() {
    let src = TempDir::new().expect("create temp dir for test");
    let dst = TempDir::new().expect("create temp dir for test");
    let key = StoredKey {
        key_id: "lineage-k".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        hsm_name: "soft".to_string(),
        key_material_b64: key_store::base64_encode(b"01234567890123456789012345678901"),
        created_at: Utc::now().to_rfc3339(),
        generation: 1,
        parent_key_id: Some("root-k".to_string()),
        derivation_purpose: None,
        children: vec![],
        lineage: None,
        expires_at: None,
        usage: None,
        purpose: None,
    };
    key_store::save_key_to_home(&key, src.path()).expect("save lineage test key");
    let out = src.path().join("exp.json");
    handle_key_export_with_home(
        "lineage-k",
        out.to_str().expect("exp.json path is valid UTF-8"),
        false,
        src.path(),
    )
    .await
    .expect("export lineage key in test");

    handle_key_import_with_home(
        out.to_str().expect("exp.json path is valid UTF-8"),
        None,
        false,
        false,
        dst.path(),
    )
    .await
    .expect("import lineage key in test");
    let loaded =
        key_store::load_key_from_home("lineage-k", dst.path()).expect("load imported lineage key");
    assert_eq!(loaded.parent_key_id.as_deref(), Some("root-k"));
}

#[tokio::test]
async fn test_export_with_home_missing_key_fails() {
    let dir = TempDir::new().expect("create temp dir for test");
    let out = dir.path().join("out.json");
    let r = handle_key_export_with_home(
        "no-such-key",
        out.to_str().expect("out.json path is valid UTF-8"),
        false,
        dir.path(),
    )
    .await;
    assert!(r.is_err());
}

#[tokio::test]
async fn test_import_with_home_missing_input_file() {
    let dir = TempDir::new().expect("create temp dir for test");
    let r =
        handle_key_import_with_home("/nonexistent/path/key.json", None, false, false, dir.path())
            .await;
    assert!(r.is_err());
}
