// SPDX-License-Identifier: AGPL-3.0-or-later

//! File-based key persistence: on-disk layout and load/save helpers.

use super::store::KeyStore;
use super::types::{KeyMetadata, KeyType};
use beardog_errors::BearDogError;
use std::path::Path;
use tokio::fs;
use zeroize::Zeroizing;

/// Persist an Ed25519 signing key and pretty-printed metadata JSON next to each other on disk.
///
/// Layout: `{storage_dir}/{key_id}.key` (raw secret bytes) and
/// `{storage_dir}/{key_id}.meta.json` (pretty-printed [`KeyMetadata`] JSON).
pub(super) async fn persist_signing_key(
    storage_dir: &Path,
    key_id: &str,
    signing_key_bytes: &[u8],
    metadata_json: &str,
) -> Result<(), BearDogError> {
    fs::create_dir_all(storage_dir).await.map_err(|e| {
        BearDogError::security(format!("Failed to create key storage directory: {e}"))
    })?;

    let key_path = storage_dir.join(format!("{key_id}.key"));
    fs::write(&key_path, signing_key_bytes)
        .await
        .map_err(|e| BearDogError::security(format!("Failed to write signing key: {e}")))?;

    let metadata_path = storage_dir.join(format!("{key_id}.meta.json"));
    fs::write(&metadata_path, metadata_json)
        .await
        .map_err(|e| BearDogError::security(format!("Failed to write key metadata: {e}")))?;

    Ok(())
}

/// Persist a symmetric key and pretty-printed metadata JSON (same file naming as signing keys).
pub(super) async fn persist_symmetric_key(
    storage_dir: &Path,
    key_id: &str,
    symmetric_key_bytes: &[u8],
    metadata_json: &str,
) -> Result<(), BearDogError> {
    fs::create_dir_all(storage_dir).await.map_err(|e| {
        BearDogError::security(format!("Failed to create key storage directory: {e}"))
    })?;

    let key_path = storage_dir.join(format!("{key_id}.key"));
    fs::write(&key_path, symmetric_key_bytes)
        .await
        .map_err(|e| BearDogError::security(format!("Failed to write symmetric key: {e}")))?;

    let metadata_path = storage_dir.join(format!("{key_id}.meta.json"));
    fs::write(&metadata_path, metadata_json)
        .await
        .map_err(|e| BearDogError::security(format!("Failed to write key metadata: {e}")))?;

    Ok(())
}

/// Remove `{key_id}.key` and `{key_id}.meta.json` under `storage_dir` if present.
pub(super) async fn delete_persisted_key(
    storage_dir: Option<&Path>,
    key_id: &str,
) -> Result<(), BearDogError> {
    if let Some(storage_dir) = storage_dir {
        let key_path = storage_dir.join(format!("{key_id}.key"));
        let metadata_path = storage_dir.join(format!("{key_id}.meta.json"));

        if key_path.exists() {
            fs::remove_file(&key_path)
                .await
                .map_err(|e| BearDogError::security(format!("Failed to delete key file: {e}")))?;
        }

        if metadata_path.exists() {
            fs::remove_file(&metadata_path).await.map_err(|e| {
                BearDogError::security(format!("Failed to delete metadata file: {e}"))
            })?;
        }
    }

    Ok(())
}

/// Scan `storage_dir` for `*.key` files with matching `*.meta.json`, then load keys into `store`.
pub(super) async fn load_from_storage(store: &KeyStore) -> Result<usize, BearDogError> {
    let storage_dir = store.storage_dir().ok_or_else(|| {
        BearDogError::security("Key storage directory not configured".to_string())
    })?;

    if !storage_dir.exists() {
        return Ok(0);
    }

    let mut entries = fs::read_dir(storage_dir).await.map_err(|e| {
        BearDogError::security(format!("Failed to read key storage directory: {e}"))
    })?;

    let mut loaded_count = 0;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| BearDogError::security(format!("Failed to read directory entry: {e}")))?
    {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("key") {
            let key_id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| BearDogError::security("Invalid key filename".to_string()))?;

            let metadata_path = storage_dir.join(format!("{key_id}.meta.json"));
            if !metadata_path.exists() {
                continue; // Skip if metadata is missing
            }

            let metadata_json = fs::read_to_string(&metadata_path)
                .await
                .map_err(|e| BearDogError::security(format!("Failed to read key metadata: {e}")))?;
            let metadata: KeyMetadata = serde_json::from_str(&metadata_json).map_err(|e| {
                BearDogError::security(format!("Failed to parse key metadata: {e}"))
            })?;

            match metadata.key_type {
                KeyType::Ed25519 => {
                    let key_bytes = fs::read(&path).await.map_err(|e| {
                        BearDogError::security(format!("Failed to read signing key: {e}"))
                    })?;
                    let KeyMetadata {
                        usage,
                        storage,
                        description,
                        ..
                    } = metadata;
                    store
                        .import_signing_key(
                            key_id.to_string(),
                            &key_bytes,
                            usage,
                            storage,
                            description,
                        )
                        .await?;
                    loaded_count += 1;
                }
                KeyType::Aes256Gcm => {
                    let key_bytes = Zeroizing::new(fs::read(&path).await.map_err(|e| {
                        BearDogError::security(format!("Failed to read symmetric key: {e}"))
                    })?);
                    store
                        .insert_symmetric_key_loaded(key_id.to_string(), key_bytes, metadata)
                        .await;
                    loaded_count += 1;
                }
                _ => {} // Skip unsupported key types
            }
        }
    }

    Ok(loaded_count)
}

#[cfg(test)]
mod persistence_tests {
    use super::super::{KeyMetadata, KeyStorage, KeyStore, KeyType, KeyUsage};
    use super::*;
    use ed25519_dalek::{SECRET_KEY_LENGTH, SigningKey};
    use tempfile::NamedTempFile;
    use tempfile::TempDir;

    #[tokio::test]
    async fn persist_signing_key_writes_key_and_metadata() {
        let dir = TempDir::new().unwrap();
        let sk = [9u8; SECRET_KEY_LENGTH];
        let meta = r#"{"key_id":"t","key_type":"Ed25519","usage":["sign"],"storage":{"Ephemeral":null},"created_at":"2024-06-01T00:00:00Z","expires_at":null,"description":null}"#;
        persist_signing_key(dir.path(), "t", &sk, meta)
            .await
            .unwrap();
        let key_path = dir.path().join("t.key");
        let meta_path = dir.path().join("t.meta.json");
        assert_eq!(tokio::fs::read(&key_path).await.unwrap(), sk);
        assert!(
            tokio::fs::read_to_string(&meta_path)
                .await
                .unwrap()
                .contains("Ed25519")
        );
    }

    #[tokio::test]
    async fn persist_symmetric_key_writes_material() {
        let dir = TempDir::new().unwrap();
        let bytes = vec![1u8; 32];
        let meta = r#"{"key_id":"s","key_type":"Aes256Gcm","usage":["encrypt"],"storage":{"Ephemeral":null},"created_at":"2024-06-01T00:00:00Z","expires_at":null,"description":null}"#;
        persist_symmetric_key(dir.path(), "s", &bytes, meta)
            .await
            .unwrap();
        assert_eq!(
            tokio::fs::read(dir.path().join("s.key")).await.unwrap(),
            bytes
        );
    }

    #[tokio::test]
    async fn delete_persisted_key_noop_without_storage_dir() {
        delete_persisted_key(None, "any").await.unwrap();
    }

    #[tokio::test]
    async fn delete_persisted_key_removes_existing_files() {
        let dir = TempDir::new().unwrap();
        tokio::fs::write(dir.path().join("x.key"), b"k")
            .await
            .unwrap();
        tokio::fs::write(dir.path().join("x.meta.json"), b"{}")
            .await
            .unwrap();
        delete_persisted_key(Some(dir.path()), "x").await.unwrap();
        assert!(!dir.path().join("x.key").exists());
        assert!(!dir.path().join("x.meta.json").exists());
    }

    #[tokio::test]
    async fn load_from_storage_errors_without_configured_directory() {
        let store = KeyStore::new(None);
        let err = load_from_storage(&store).await.unwrap_err();
        assert!(format!("{err}").contains("not configured"));
    }

    #[tokio::test]
    async fn load_from_storage_read_dir_fails_when_path_is_file() {
        let file = NamedTempFile::new().unwrap();
        let store = KeyStore::new(Some(file.path().to_path_buf()));
        let err = load_from_storage(&store).await.unwrap_err();
        assert!(format!("{err}").contains("read key storage"));
    }

    #[tokio::test]
    async fn load_from_storage_rejects_invalid_metadata_json() {
        let dir = TempDir::new().unwrap();
        let key_path = dir.path().join("bad.key");
        tokio::fs::write(&key_path, [3u8; SECRET_KEY_LENGTH])
            .await
            .unwrap();
        tokio::fs::write(dir.path().join("bad.meta.json"), "not-json")
            .await
            .unwrap();
        let store = KeyStore::new(Some(dir.path().to_path_buf()));
        let err = load_from_storage(&store).await.unwrap_err();
        assert!(format!("{err}").contains("parse"));
    }

    #[tokio::test]
    async fn load_from_storage_fails_on_short_ed25519_key_material() {
        let dir = TempDir::new().unwrap();
        tokio::fs::write(dir.path().join("short.key"), [0u8; 8])
            .await
            .unwrap();
        let meta = KeyMetadata {
            key_id: "short".to_string(),
            key_type: KeyType::Ed25519,
            usage: vec![KeyUsage::Sign],
            storage: KeyStorage::Ephemeral,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description: None,
        };
        tokio::fs::write(
            dir.path().join("short.meta.json"),
            serde_json::to_string_pretty(&meta).unwrap(),
        )
        .await
        .unwrap();
        let store = KeyStore::new(Some(dir.path().to_path_buf()));
        let err = load_from_storage(&store).await.unwrap_err();
        assert!(format!("{err}").contains("32") || format!("{err}").contains("length"));
    }

    #[tokio::test]
    async fn load_from_storage_skips_x25519_keys() {
        let dir = TempDir::new().unwrap();
        tokio::fs::write(dir.path().join("x.key"), b"material")
            .await
            .unwrap();
        let meta = KeyMetadata {
            key_id: "x".to_string(),
            key_type: KeyType::X25519,
            usage: vec![KeyUsage::KeyAgreement],
            storage: KeyStorage::Ephemeral,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description: None,
        };
        tokio::fs::write(
            dir.path().join("x.meta.json"),
            serde_json::to_string_pretty(&meta).unwrap(),
        )
        .await
        .unwrap();
        let store = KeyStore::new(Some(dir.path().to_path_buf()));
        let n = load_from_storage(&store).await.unwrap();
        assert_eq!(n, 0);
    }

    #[tokio::test]
    async fn load_from_storage_loads_aes_symmetric_key() {
        let dir = TempDir::new().unwrap();
        let key_bytes = [7u8; 32];
        tokio::fs::write(dir.path().join("sym.key"), key_bytes)
            .await
            .unwrap();
        let meta = KeyMetadata {
            key_id: "sym".to_string(),
            key_type: KeyType::Aes256Gcm,
            usage: vec![KeyUsage::Encrypt],
            storage: KeyStorage::Ephemeral,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description: Some("sym".into()),
        };
        tokio::fs::write(
            dir.path().join("sym.meta.json"),
            serde_json::to_string_pretty(&meta).unwrap(),
        )
        .await
        .unwrap();
        let store = KeyStore::new(Some(dir.path().to_path_buf()));
        let n = load_from_storage(&store).await.unwrap();
        assert_eq!(n, 1);
        let got = store.get_symmetric_key("sym").await.unwrap();
        assert_eq!(&*got, key_bytes.as_slice());
    }

    #[tokio::test]
    async fn load_from_storage_returns_zero_when_directory_missing() {
        let dir = TempDir::new().unwrap();
        let missing = dir.path().join("no_such_subdir");
        let store = KeyStore::new(Some(missing));
        let n = load_from_storage(&store).await.unwrap();
        assert_eq!(n, 0);
    }

    #[tokio::test]
    async fn load_from_storage_loads_valid_ed25519_from_disk() {
        let dir = TempDir::new().unwrap();
        let sk = [11u8; SECRET_KEY_LENGTH];
        let meta = KeyMetadata {
            key_id: "edload".to_string(),
            key_type: KeyType::Ed25519,
            usage: vec![KeyUsage::Sign],
            storage: KeyStorage::Ephemeral,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description: None,
        };
        let meta_json = serde_json::to_string_pretty(&meta).unwrap();
        persist_signing_key(dir.path(), "edload", &sk, &meta_json)
            .await
            .unwrap();
        let store = KeyStore::new(Some(dir.path().to_path_buf()));
        let n = load_from_storage(&store).await.unwrap();
        assert_eq!(n, 1);
        let vk = store.get_verifying_key("edload").await.unwrap();
        assert_eq!(
            vk.as_bytes(),
            SigningKey::from_bytes(&sk).verifying_key().as_bytes()
        );
    }

    #[tokio::test]
    async fn persist_signing_key_fails_when_storage_path_is_file() {
        let file = NamedTempFile::new().unwrap();
        let err = persist_signing_key(file.path(), "x", &[0u8; SECRET_KEY_LENGTH], "{}")
            .await
            .unwrap_err();
        assert!(
            format!("{err}").to_lowercase().contains("storage")
                || format!("{err}").to_lowercase().contains("directory")
                || format!("{err}").to_lowercase().contains("not a directory")
        );
    }

    #[tokio::test]
    async fn persist_symmetric_key_fails_when_storage_path_is_file() {
        let file = NamedTempFile::new().unwrap();
        let err = persist_symmetric_key(file.path(), "x", &[0u8; 32], "{}")
            .await
            .unwrap_err();
        assert!(
            format!("{err}").contains("storage")
                || format!("{err}").contains("directory")
                || format!("{err}").contains("create")
        );
    }

    #[tokio::test]
    async fn delete_persisted_key_removes_only_existing_sidecar_files() {
        let dir = TempDir::new().unwrap();
        tokio::fs::write(dir.path().join("solo.key"), b"k")
            .await
            .unwrap();
        delete_persisted_key(Some(dir.path()), "solo")
            .await
            .unwrap();
        assert!(!dir.path().join("solo.key").exists());
        tokio::fs::write(dir.path().join("meta_only.meta.json"), b"{}")
            .await
            .unwrap();
        delete_persisted_key(Some(dir.path()), "meta_only")
            .await
            .unwrap();
        assert!(!dir.path().join("meta_only.meta.json").exists());
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn load_from_storage_fails_when_metadata_unreadable() {
        use std::os::unix::fs::PermissionsExt;

        let dir = TempDir::new().unwrap();
        let key_path = dir.path().join("k.key");
        tokio::fs::write(&key_path, [4u8; SECRET_KEY_LENGTH])
            .await
            .unwrap();
        let meta = KeyMetadata {
            key_id: "k".to_string(),
            key_type: KeyType::Ed25519,
            usage: vec![KeyUsage::Sign],
            storage: KeyStorage::Ephemeral,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description: None,
        };
        let meta_path = dir.path().join("k.meta.json");
        tokio::fs::write(&meta_path, serde_json::to_string_pretty(&meta).unwrap())
            .await
            .unwrap();
        std::fs::set_permissions(&meta_path, std::fs::Permissions::from_mode(0o000)).unwrap();
        let store = KeyStore::new(Some(dir.path().to_path_buf()));
        let err = load_from_storage(&store).await.unwrap_err();
        assert!(format!("{err}").to_lowercase().contains("metadata"));
        let _ = std::fs::set_permissions(&meta_path, std::fs::Permissions::from_mode(0o644));
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn load_from_storage_fails_when_ed25519_key_file_unreadable() {
        use std::os::unix::fs::PermissionsExt;

        let dir = TempDir::new().unwrap();
        let key_path = dir.path().join("k.key");
        tokio::fs::write(&key_path, [4u8; SECRET_KEY_LENGTH])
            .await
            .unwrap();
        let meta = KeyMetadata {
            key_id: "k".to_string(),
            key_type: KeyType::Ed25519,
            usage: vec![KeyUsage::Sign],
            storage: KeyStorage::Ephemeral,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description: None,
        };
        tokio::fs::write(
            dir.path().join("k.meta.json"),
            serde_json::to_string_pretty(&meta).unwrap(),
        )
        .await
        .unwrap();
        std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o000)).unwrap();
        let store = KeyStore::new(Some(dir.path().to_path_buf()));
        let err = load_from_storage(&store).await.unwrap_err();
        assert!(format!("{err}").to_lowercase().contains("signing"));
        let _ = std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o644));
    }

    #[tokio::test]
    #[cfg(unix)]
    async fn load_from_storage_rejects_non_utf8_key_filename_stem() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt;

        let dir = TempDir::new().unwrap();
        let bad_name = OsString::from_vec(vec![0xff, 0xfe, b'.', b'k', b'e', b'y']);
        let bad_path = dir.path().join(bad_name);
        std::fs::write(&bad_path, [1u8; SECRET_KEY_LENGTH]).unwrap();
        let store = KeyStore::new(Some(dir.path().to_path_buf()));
        let err = load_from_storage(&store).await.unwrap_err();
        assert!(format!("{err}").to_lowercase().contains("filename"));
        let _ = std::fs::remove_file(bad_path);
    }

    #[tokio::test]
    async fn load_from_storage_symmetric_read_failure_surfaces_error() {
        let dir = TempDir::new().unwrap();
        let key_path = dir.path().join("sym2.key");
        tokio::fs::write(&key_path, [5u8; 32]).await.unwrap();
        let meta = KeyMetadata {
            key_id: "sym2".to_string(),
            key_type: KeyType::Aes256Gcm,
            usage: vec![KeyUsage::Encrypt],
            storage: KeyStorage::Ephemeral,
            created_at: chrono::Utc::now(),
            expires_at: None,
            description: None,
        };
        tokio::fs::write(
            dir.path().join("sym2.meta.json"),
            serde_json::to_string_pretty(&meta).unwrap(),
        )
        .await
        .unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o000)).unwrap();
            let store = KeyStore::new(Some(dir.path().to_path_buf()));
            let err = load_from_storage(&store).await.unwrap_err();
            assert!(format!("{err}").to_lowercase().contains("symmetric"));
            let _ = std::fs::set_permissions(&key_path, std::fs::Permissions::from_mode(0o644));
        }
        #[cfg(not(unix))]
        {
            let store = KeyStore::new(Some(dir.path().to_path_buf()));
            let n = load_from_storage(&store).await.unwrap();
            assert_eq!(n, 1);
        }
    }
}
