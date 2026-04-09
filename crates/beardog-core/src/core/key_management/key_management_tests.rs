// SPDX-License-Identifier: AGPL-3.0-or-later

#![expect(clippy::unwrap_used, reason = "test assertions")]

use super::*;
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SigningKey};
use tempfile::TempDir;

/// Exercises `types.rs` display, default, and serde paths for coverage (re-exported from `super`).
#[test]
fn key_management_types_defaults_and_displays() {
    assert_eq!(format!("{}", KeyType::default()), "Ed25519");
    assert_eq!(format!("{}", KeyUsage::default()), "sign");
    assert_eq!(format!("{}", KeyStorage::default()), "ephemeral");

    assert_eq!(format!("{}", KeyType::Rsa2048), "RSA-2048");
    assert_eq!(format!("{}", KeyType::Rsa4096), "RSA-4096");
    assert_eq!(format!("{}", KeyUsage::KeyAgreement), "key_agreement");

    let h = KeyStorage::Hsm("tok".to_string());
    assert_eq!(format!("{h}"), "hsm:tok");

    let meta = KeyMetadata {
        key_id: "cover".to_string(),
        key_type: KeyType::X25519,
        usage: vec![KeyUsage::Verify, KeyUsage::Decrypt],
        storage: KeyStorage::Ephemeral,
        created_at: chrono::Utc::now(),
        expires_at: None,
        description: None,
    };
    let json = serde_json::to_string(&meta).expect("serde");
    let back: KeyMetadata = serde_json::from_str(&json).expect("de");
    assert_eq!(back.key_type, KeyType::X25519);
}

#[tokio::test]
async fn test_generate_signing_key() {
    let key_store = KeyStore::new(None);

    let public_key = key_store
        .generate_signing_key(
            "test-key-1".to_string(),
            vec![KeyUsage::Sign, KeyUsage::Verify],
            KeyStorage::Ephemeral,
            Some("Test signing key".to_string()),
        )
        .await
        .expect("Key generation should succeed");

    assert_eq!(public_key.len(), PUBLIC_KEY_LENGTH);

    let metadata = key_store
        .get_metadata("test-key-1")
        .await
        .expect("Metadata should exist");
    assert_eq!(metadata.key_type, KeyType::Ed25519);
    assert_eq!(metadata.description, Some("Test signing key".to_string()));
}

#[tokio::test]
async fn test_generate_symmetric_key() {
    let key_store = KeyStore::new(None);

    key_store
        .generate_symmetric_key(
            "test-sym-1".to_string(),
            vec![KeyUsage::Encrypt, KeyUsage::Decrypt],
            KeyStorage::Ephemeral,
            Some("Test symmetric key".to_string()),
        )
        .await
        .expect("Key generation should succeed");

    let key = key_store
        .get_symmetric_key("test-sym-1")
        .await
        .expect("Key should exist");
    assert_eq!(key.len(), 32);
}

#[tokio::test]
async fn test_key_persistence() {
    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let key_store = KeyStore::new(Some(temp_dir.path().to_path_buf()));

    // Generate and persist a key
    key_store
        .generate_signing_key(
            "persistent-key".to_string(),
            vec![KeyUsage::Sign],
            KeyStorage::File(temp_dir.path().to_path_buf()),
            None,
        )
        .await
        .expect("Key generation should succeed");

    // Create new key store and load from storage
    let key_store2 = KeyStore::new(Some(temp_dir.path().to_path_buf()));
    let loaded_count = key_store2
        .load_from_storage()
        .await
        .expect("Load should succeed");

    assert_eq!(loaded_count, 1);

    let metadata = key_store2
        .get_metadata("persistent-key")
        .await
        .expect("Metadata should exist after load");
    assert_eq!(metadata.key_type, KeyType::Ed25519);
}

#[tokio::test]
async fn test_delete_key() {
    let key_store = KeyStore::new(None);

    key_store
        .generate_signing_key(
            "delete-me".to_string(),
            vec![KeyUsage::Sign],
            KeyStorage::Ephemeral,
            None,
        )
        .await
        .expect("Key generation should succeed");

    key_store
        .delete_key("delete-me")
        .await
        .expect("Delete should succeed");

    let result = key_store.get_metadata("delete-me").await;
    assert!(result.is_err(), "Key should not exist after deletion");
}

#[tokio::test]
async fn test_list_keys() {
    let key_store = KeyStore::new(None);

    key_store
        .generate_signing_key(
            "key1".to_string(),
            vec![KeyUsage::Sign],
            KeyStorage::Ephemeral,
            None,
        )
        .await
        .expect("Key generation should succeed");

    key_store
        .generate_signing_key(
            "key2".to_string(),
            vec![KeyUsage::Sign],
            KeyStorage::Ephemeral,
            None,
        )
        .await
        .expect("Key generation should succeed");

    let keys = key_store.list_keys().await.expect("List should succeed");
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
}

#[tokio::test]
async fn test_import_signing_key_rejects_bad_length() {
    let store = KeyStore::new(None);
    let err = store
        .import_signing_key(
            "bad".to_string(),
            &[0u8; 16],
            vec![KeyUsage::Sign],
            KeyStorage::Ephemeral,
            None,
        )
        .await
        .expect_err("wrong length");
    assert!(format!("{err}").contains("32"));
}

#[tokio::test]
async fn test_get_signing_key_missing() {
    let store = KeyStore::new(None);
    let err = store.get_signing_key("nope").await.expect_err("missing");
    assert!(format!("{err}").contains("nope"));
}

#[tokio::test]
async fn test_import_public_key_ed25519_and_reject_unsupported_type() {
    let store = KeyStore::new(None);
    let mut sk = [0u8; SECRET_KEY_LENGTH];
    use rand::RngCore;
    rand::rng().fill_bytes(&mut sk);
    let signing = SigningKey::from_bytes(&sk);
    let vk = signing.verifying_key();
    store
        .import_public_key(
            "pub-only".to_string(),
            vk.as_bytes(),
            KeyType::Ed25519,
            Some("peer".to_string()),
        )
        .await
        .expect("import pub");

    let err = store
        .import_public_key("rsa".to_string(), vk.as_bytes(), KeyType::Rsa2048, None)
        .await
        .expect_err("rsa not supported for import");
    assert!(format!("{err}").to_lowercase().contains("unsupported"));

    let err = store
        .import_public_key("bad-len".to_string(), &[0u8; 8], KeyType::Ed25519, None)
        .await
        .expect_err("short key");
    assert!(format!("{err}").contains("32"));
}

#[tokio::test]
async fn test_generate_signing_key_persists_when_storage_dir_set() {
    let temp = TempDir::new().expect("temp");
    let dir = temp.path().to_path_buf();
    let store = KeyStore::new(Some(dir.clone()));
    store
        .generate_signing_key(
            "file-sk".to_string(),
            vec![KeyUsage::Sign],
            KeyStorage::File(dir.clone()),
            None,
        )
        .await
        .expect("gen");

    assert!(dir.join("file-sk.key").exists());
    assert!(dir.join("file-sk.meta.json").exists());
}

#[tokio::test]
async fn test_load_from_storage_empty_dir_returns_zero() {
    let temp = TempDir::new().expect("temp");
    let store = KeyStore::new(Some(temp.path().to_path_buf()));
    let n = store.load_from_storage().await.expect("load");
    assert_eq!(n, 0);
}

#[tokio::test]
async fn test_delete_key_removes_persisted_files() {
    let temp = TempDir::new().expect("temp");
    let dir = temp.path().to_path_buf();
    let store = KeyStore::new(Some(dir.clone()));
    store
        .generate_symmetric_key(
            "sym-del".to_string(),
            vec![KeyUsage::Encrypt],
            KeyStorage::File(dir.clone()),
            None,
        )
        .await
        .expect("sym");

    assert!(dir.join("sym-del.key").exists());
    store.delete_key("sym-del").await.expect("delete");
    assert!(!dir.join("sym-del.key").exists());
}

#[tokio::test]
async fn test_generate_signing_key_file_storage_without_store_dir_fails() {
    let store = KeyStore::new(None);
    let err = store
        .generate_signing_key(
            "x".to_string(),
            vec![KeyUsage::Sign],
            KeyStorage::File(std::path::PathBuf::from("/tmp/unused")),
            None,
        )
        .await
        .expect_err("needs storage_dir");
    assert!(format!("{err}").to_lowercase().contains("storage"));
}

#[tokio::test]
async fn test_get_verifying_and_symmetric_and_metadata_missing() {
    let store = KeyStore::new(None);
    assert!(store.get_verifying_key("n").await.is_err());
    assert!(store.get_symmetric_key("n").await.is_err());
    assert!(store.get_metadata("n").await.is_err());
}

#[tokio::test]
async fn test_load_from_storage_skips_orphan_key_without_metadata() {
    let temp = TempDir::new().expect("temp dir for test");
    let dir = temp.path();
    std::fs::write(dir.join("lonely.key"), [7u8; 32]).expect("write orphan key file");
    let store = KeyStore::new(Some(dir.to_path_buf()));
    let n = store
        .load_from_storage()
        .await
        .expect("load_from_storage in test");
    assert_eq!(n, 0);
}

#[tokio::test]
async fn test_load_from_storage_skips_unsupported_key_type() {
    let temp = TempDir::new().expect("temp dir for test");
    let dir = temp.path();
    std::fs::write(dir.join("rsa.key"), b"dummy-key-material").expect("write rsa.key");
    let meta = KeyMetadata {
        key_id: "rsa".to_string(),
        key_type: KeyType::Rsa2048,
        usage: vec![KeyUsage::Sign],
        storage: KeyStorage::File(dir.to_path_buf()),
        created_at: chrono::Utc::now(),
        expires_at: None,
        description: None,
    };
    std::fs::write(
        dir.join("rsa.meta.json"),
        serde_json::to_string_pretty(&meta).expect("serialize KeyMetadata in test"),
    )
    .expect("write rsa.meta.json");
    let store = KeyStore::new(Some(dir.to_path_buf()));
    let n = store
        .load_from_storage()
        .await
        .expect("load_from_storage in test");
    assert_eq!(n, 0);
}

#[tokio::test]
async fn test_import_signing_key_matches_verifying_key() {
    let store = KeyStore::new(None);
    let mut sk_bytes = [0u8; SECRET_KEY_LENGTH];
    use rand::RngCore;
    rand::rng().fill_bytes(&mut sk_bytes);
    store
        .import_signing_key(
            "imported-ed25519".to_string(),
            &sk_bytes,
            vec![KeyUsage::Sign],
            KeyStorage::Ephemeral,
            None,
        )
        .await
        .expect("import");

    let vk = store
        .get_verifying_key("imported-ed25519")
        .await
        .expect("verifying key");
    let expected = SigningKey::from_bytes(&sk_bytes).verifying_key();
    assert_eq!(vk.as_bytes(), expected.as_bytes());
}
