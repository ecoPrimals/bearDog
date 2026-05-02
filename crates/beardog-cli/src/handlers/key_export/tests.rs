// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::handlers::key_store;
use crate::handlers::key_store::StoredKey;

use super::crypto::{decrypt_key_material, encrypt_key_material};
use super::export::{handle_key_export, handle_key_export_with_home};
use super::import::{handle_key_import, handle_key_import_with_home};
use super::types::ExportedKey;

use beardog_errors::process_env;
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
fn test_decrypt_key_material_missing_salt_key() {
    let enc = encrypt_key_material("e30=", "pw").expect("encrypt fixture for missing-salt test");
    let v: serde_json::Value =
        serde_json::from_str(&enc).expect("parse encrypted package JSON in test");
    let bad = serde_json::json!({
        "nonce": v["nonce"],
        "ciphertext": v["ciphertext"],
    });
    assert!(decrypt_key_material(&bad.to_string(), "pw").is_err());
}

#[test]
fn test_decrypt_key_material_missing_nonce_key() {
    let enc = encrypt_key_material("e30=", "pw").expect("encrypt fixture for missing-nonce test");
    let v: serde_json::Value =
        serde_json::from_str(&enc).expect("parse encrypted package JSON in test");
    let bad = serde_json::json!({
        "salt": v["salt"],
        "ciphertext": v["ciphertext"],
    });
    assert!(decrypt_key_material(&bad.to_string(), "pw").is_err());
}

#[test]
fn test_decrypt_key_material_missing_ciphertext_key() {
    let enc =
        encrypt_key_material("e30=", "pw").expect("encrypt fixture for missing-ciphertext test");
    let v: serde_json::Value =
        serde_json::from_str(&enc).expect("parse encrypted package JSON in test");
    let bad = serde_json::json!({
        "salt": v["salt"],
        "nonce": v["nonce"],
    });
    assert!(decrypt_key_material(&bad.to_string(), "pw").is_err());
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

#[tokio::test]
async fn test_import_with_home_includes_expires_in_details() {
    let dir = TempDir::new().expect("temp dir for expires import test");
    let p = dir.path().join("with-expires.json");
    let exported = ExportedKey {
        key_id: "exp-k".to_string(),
        algorithm: "aes-256-gcm".to_string(),
        parent: None,
        generation: 0,
        created_at: Utc::now().to_rfc3339(),
        context: None,
        expires_at: Some("2035-12-31T23:59:59Z".to_string()),
        usage: None,
        purpose: None,
        metadata: std::collections::HashMap::new(),
        key_material: key_store::base64_encode(b"01234567890123456789012345678901"),
        encrypted: false,
        version: "1.0".to_string(),
    };
    std::fs::write(
        &p,
        serde_json::to_string_pretty(&exported).expect("serialize export with expires"),
    )
    .expect("write export file");

    handle_key_import_with_home(
        p.to_str().expect("with-expires path utf8"),
        None,
        false,
        false,
        dir.path(),
    )
    .await
    .expect("import key with expires_at");

    let loaded = key_store::load_key_from_home("exp-k", dir.path()).expect("load exp-k");
    assert_eq!(loaded.expires_at.as_deref(), Some("2035-12-31T23:59:59Z"));
}

#[cfg(unix)]
#[tokio::test]
async fn test_export_with_home_fails_when_output_dir_unwritable() {
    use std::os::unix::fs::PermissionsExt;

    let dir = TempDir::new().expect("temp dir for unwritable export test");
    let key = StoredKey {
        key_id: "ro-test".to_string(),
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
    key_store::save_key_to_home(&key, dir.path()).expect("save ro-test key");

    let blocked = dir.path().join("blocked");
    std::fs::create_dir(&blocked).expect("create blocked export dir");
    std::fs::set_permissions(&blocked, std::fs::Permissions::from_mode(0o500))
        .expect("chmod blocked dir read+execute only");

    let out = blocked.join("out.json");
    let r = handle_key_export_with_home(
        "ro-test",
        out.to_str().expect("out path utf8"),
        false,
        dir.path(),
    )
    .await;

    assert!(
        r.is_err(),
        "expected fs::write to fail under unwritable parent: {r:?}"
    );
}

#[tokio::test]
#[serial_test::serial]
async fn test_handle_key_export_errors_when_home_unset() {
    let _guard = crate::__cli_test_env::HOME
        .lock()
        .expect("cli HOME env test lock poisoned");

    let old_home = std::env::var_os("HOME");
    process_env::remove_var("HOME");

    let err = handle_key_export("any-key", "/tmp/out.json", false)
        .await
        .expect_err("handle_key_export without HOME");
    assert!(
        err.to_string().contains("HOME") || err.to_string().contains("home"),
        "{err}"
    );

    match old_home {
        Some(h) => process_env::set_var("HOME", h.as_os_str()),
        None => process_env::remove_var("HOME"),
    }
}

#[tokio::test]
#[serial_test::serial]
async fn test_handle_key_import_errors_when_home_unset() {
    let _guard = crate::__cli_test_env::HOME
        .lock()
        .expect("cli HOME env test lock poisoned");

    let old_home = std::env::var_os("HOME");
    process_env::remove_var("HOME");

    let err = handle_key_import("/tmp/in.json", None, false)
        .await
        .expect_err("handle_key_import without HOME");
    assert!(
        err.to_string().contains("HOME") || err.to_string().contains("home"),
        "{err}"
    );

    match old_home {
        Some(h) => process_env::set_var("HOME", h.as_os_str()),
        None => process_env::remove_var("HOME"),
    }
}

#[tokio::test]
#[serial_test::serial]
async fn test_handle_key_export_roundtrip_uses_default_home_env() {
    let dir = TempDir::new().expect("temp home for handle_key_export env test");
    let dst = TempDir::new().expect("temp home for handle_key_import env test");

    let _guard = crate::__cli_test_env::HOME
        .lock()
        .expect("cli HOME env test lock poisoned");

    let old_home = std::env::var_os("HOME");
    process_env::set_var("HOME", dir.path().as_os_str());

    let key = StoredKey {
        key_id: "env-roundtrip".to_string(),
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
    key_store::save_key_to_home(&key, dir.path()).expect("save key under HOME");

    let out = dir.path().join("exported-env.json");
    handle_key_export("env-roundtrip", out.to_str().expect("utf8"), false)
        .await
        .expect("handle_key_export with HOME");

    process_env::set_var("HOME", dst.path().as_os_str());

    handle_key_import(out.to_str().expect("utf8"), None, false)
        .await
        .expect("handle_key_import with HOME");

    let loaded = key_store::load_key_from_home("env-roundtrip", dst.path()).expect("load imported");
    assert_eq!(loaded.key_material_b64, key.key_material_b64);

    match old_home {
        Some(h) => process_env::set_var("HOME", h.as_os_str()),
        None => process_env::remove_var("HOME"),
    }
}

#[tokio::test]
async fn test_export_with_home_errors_when_output_path_is_directory() {
    let dir = TempDir::new().expect("temp dir for directory-as-output test");
    let key = StoredKey {
        key_id: "dir-out-key".to_string(),
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
    key_store::save_key_to_home(&key, dir.path()).expect("save key");

    let r = handle_key_export_with_home(
        "dir-out-key",
        dir.path().to_str().expect("utf8"),
        false,
        dir.path(),
    )
    .await;
    assert!(r.is_err(), "fs::write to a directory path must fail: {r:?}");
}

#[cfg(unix)]
#[tokio::test]
async fn test_import_with_home_errors_when_input_path_is_directory() {
    let dir = TempDir::new().expect("temp dir for directory-as-input test");
    let dst = TempDir::new().expect("dest home");
    let r = handle_key_import_with_home(
        dir.path().to_str().expect("utf8"),
        None,
        false,
        false,
        dst.path(),
    )
    .await;
    assert!(
        r.is_err(),
        "reading a directory as a key file must fail: {r:?}"
    );
}

#[test]
fn test_decrypt_key_material_salt_must_be_string() {
    let enc = encrypt_key_material("e30=", "pw").expect("encrypt fixture");
    let mut v: serde_json::Value = serde_json::from_str(&enc).expect("parse package");
    v["salt"] = serde_json::json!(["not", "a", "string"]);
    assert!(decrypt_key_material(&v.to_string(), "pw").is_err());
}

#[test]
fn test_decrypt_key_material_nonce_must_be_string() {
    let enc = encrypt_key_material("e30=", "pw").expect("encrypt fixture");
    let mut v: serde_json::Value = serde_json::from_str(&enc).expect("parse package");
    v["nonce"] = serde_json::json!(42);
    assert!(decrypt_key_material(&v.to_string(), "pw").is_err());
}

#[test]
fn test_decrypt_key_material_ciphertext_must_be_string() {
    let enc = encrypt_key_material("e30=", "pw").expect("encrypt fixture");
    let mut v: serde_json::Value = serde_json::from_str(&enc).expect("parse package");
    v["ciphertext"] = serde_json::json!({});
    assert!(decrypt_key_material(&v.to_string(), "pw").is_err());
}

#[test]
fn test_decrypt_key_material_salt_empty_string_errors() {
    let enc = encrypt_key_material("e30=", "pw").expect("encrypt fixture");
    let mut v: serde_json::Value = serde_json::from_str(&enc).expect("parse package");
    v["salt"] = serde_json::Value::String(String::new());
    assert!(decrypt_key_material(&v.to_string(), "pw").is_err());
}
