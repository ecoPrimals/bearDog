// SPDX-License-Identifier: AGPL-3.0-or-later

//! List keys in the local store and show key metadata (`info`).

use crate::handlers::key_store;
use beardog_errors::BearDogError;
use std::path::Path;

/// Handle key list command
///
/// # Errors
///
/// Returns an error if the key home directory cannot be resolved or keys cannot be listed.
pub async fn handle_key_list(hsm_filter: Option<&str>, _verbose: bool) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_list_with_home(hsm_filter, _verbose, &home).await
}

/// Same as [`handle_key_list`] but with an explicit home directory for the key store (tests / DI).
///
/// # Errors
///
/// Returns an error if keys cannot be listed from the given home.
pub async fn handle_key_list_with_home(
    hsm_filter: Option<&str>,
    _verbose: bool,
    home: impl AsRef<Path>,
) -> Result<(), BearDogError> {
    println!("🔑 Available Keys");
    println!("================");
    println!();

    let keys = key_store::list_keys_from_home(home)?;

    let filtered_keys: Vec<_> = if let Some(filter) = hsm_filter {
        println!("📌 Filtering by HSM: {filter}");
        println!();
        keys.into_iter()
            .filter(|k| k.hsm_name.to_lowercase().contains(&filter.to_lowercase()))
            .collect()
    } else {
        keys
    };

    if filtered_keys.is_empty() {
        println!("No keys found.");
        println!();
        println!("💡 Generate a key:");
        println!("   beardog key generate --key-id my-key --algorithm aes256-gcm");
        return Ok(());
    }

    println!("Found {} key(s):", filtered_keys.len());
    println!();

    for key in filtered_keys {
        println!("📋 Key: {}", key.key_id);
        println!("   Algorithm: {}", key.algorithm);
        println!("   HSM: {}", key.hsm_name);
        println!("   Created: {}", key.created_at);
        println!();
    }

    Ok(())
}

/// Handle key info command — load and display key metadata from the local key
/// store.
///
/// # Errors
///
/// Returns an error if the key cannot be found or read from the store.
pub async fn handle_key_info(key_id: &str) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_info_with_home(key_id, home).await
}

/// Same as [`handle_key_info`] but with an explicit home directory (tests / DI).
///
/// # Errors
///
/// Returns an error if the key cannot be found or read from the store.
pub async fn handle_key_info_with_home(
    key_id: &str,
    home: impl AsRef<Path>,
) -> Result<(), BearDogError> {
    let stored = key_store::load_key_from_home(key_id, home)?;

    println!("🔍 Key Information");
    println!("=================");
    println!();
    println!("  Key ID:      {}", stored.key_id);
    println!("  Algorithm:   {}", stored.algorithm);
    println!("  HSM:         {}", stored.hsm_name);
    println!("  Created:     {}", stored.created_at);
    println!("  Generation:  {}", stored.generation);

    if let Some(ref parent) = stored.parent_key_id {
        println!("  Parent key:  {parent}");
    }
    if let Some(ref purpose) = stored.derivation_purpose {
        println!("  Derived for: {purpose}");
    }
    if !stored.children.is_empty() {
        println!("  Children:    {}", stored.children.join(", "));
    }

    println!();
    Ok(())
}

#[cfg(test)]
mod list_tests {
    use super::{handle_key_info, handle_key_info_with_home, handle_key_list_with_home};
    use crate::handlers::key_store;
    use chrono::Utc;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_handle_key_list_empty_store() {
        let dir = TempDir::new().expect("create temp directory for empty key list test");
        handle_key_list_with_home(None, false, dir.path())
            .await
            .expect("list keys in empty store");
    }

    #[tokio::test]
    async fn test_handle_key_list_with_filter_and_keys() {
        let dir = TempDir::new().expect("create temp directory for filtered key list test");
        let home = dir.path();

        let alpha = key_store::StoredKey {
            key_id: "alpha".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "AlphaHSM-software".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[1u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        let beta = key_store::StoredKey {
            hsm_name: "BetaHSM-hardware".to_string(),
            key_id: "beta".to_string(),
            ..alpha.clone()
        };
        key_store::save_key_to_home(&alpha, home).expect("save alpha key");
        key_store::save_key_to_home(&beta, home).expect("save beta key");

        handle_key_list_with_home(Some("alpha"), false, home)
            .await
            .expect("list keys filtered by alpha");
        handle_key_list_with_home(None, true, home)
            .await
            .expect("list all keys verbose");
    }

    #[tokio::test]
    async fn test_handle_key_info_missing_key() {
        let result = handle_key_info("nonexistent-key").await;
        assert!(
            result.is_err(),
            "key info for a nonexistent key should return Err"
        );
    }

    #[tokio::test]
    async fn test_handle_key_info_existing_key() {
        let dir = TempDir::new().expect("create temp directory");
        let home = dir.path();

        let stored = key_store::StoredKey {
            key_id: "info-test-key".to_string(),
            algorithm: "ed25519".to_string(),
            hsm_name: "software".to_string(),
            created_at: "2026-01-01T00:00:00Z".to_string(),
            key_material_b64: "dGVzdA==".to_string(),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&stored, home).expect("save test key");
        let result = handle_key_info_with_home("info-test-key", home).await;
        assert!(result.is_ok(), "key info for existing key should succeed");
    }

    #[tokio::test]
    async fn test_handle_key_list_filter_matches_nothing() {
        let dir = TempDir::new().expect("create temp directory for filter matches nothing test");
        let home = dir.path();

        let k = key_store::StoredKey {
            key_id: "only-key".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "LocalSoft".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[1u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&k, home).expect("save only-key");

        handle_key_list_with_home(Some("nomatch-xyz"), false, home)
            .await
            .expect("list with filter matching nothing");
    }

    #[tokio::test]
    async fn test_handle_key_list_hsm_filter_is_case_insensitive() {
        let dir = TempDir::new().expect("create temp directory for case-insensitive filter test");
        let home = dir.path();
        let k = key_store::StoredKey {
            key_id: "k1".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "MySoftHsm".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[1u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&k, home).expect("save k1 for case test");
        handle_key_list_with_home(Some("soft"), false, home)
            .await
            .expect("list with lowercase soft filter");
    }
}
