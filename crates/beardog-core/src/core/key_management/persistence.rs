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
pub(crate) async fn persist_signing_key(
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
pub(crate) async fn persist_symmetric_key(
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
pub(crate) async fn delete_persisted_key(
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
pub(crate) async fn load_from_storage(store: &KeyStore) -> Result<usize, BearDogError> {
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
