// SPDX-License-Identifier: AGPL-3.0-or-later

//! Delete keys from the local key store (with optional confirmation).

use crate::handlers::key_store;
use beardog_errors::BearDogError;
use std::path::Path;

/// Handle key delete command
///
/// # Errors
///
/// Returns an error if the key home directory cannot be resolved or deletion fails.
pub async fn handle_key_delete(key_id: &str, skip_confirm: bool) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_delete_with_home(key_id, skip_confirm, &home).await
}

/// Same as [`handle_key_delete`] but with an explicit home directory for the key store (tests / DI).
///
/// # Errors
///
/// Returns an error if the key cannot be deleted from storage.
pub async fn handle_key_delete_with_home(
    key_id: &str,
    skip_confirm: bool,
    home: impl AsRef<Path>,
) -> Result<(), BearDogError> {
    println!("🗑️  Delete Key");
    println!("=============");
    println!();

    if !skip_confirm {
        println!("⚠️  Are you sure you want to delete key '{key_id}'?");
        println!("   This action CANNOT be undone!");
        println!();
        println!("   Run with --yes to skip this prompt.");
        return Ok(());
    }

    key_store::delete_key_from_home(key_id, home)?;
    println!("✅ Key '{key_id}' deleted successfully");

    Ok(())
}

#[cfg(test)]
mod storage_tests {
    use super::*;
    use crate::handlers::key_store;
    use chrono::Utc;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_handle_key_delete_skip_confirm_removes_file() {
        let dir = TempDir::new().expect("create temp directory for key delete test");
        let home = dir.path();

        let k = key_store::StoredKey {
            key_id: "to-delete".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "h".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[2u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&k, home).expect("save to-delete key");

        handle_key_delete_with_home("to-delete", true, home)
            .await
            .expect("delete key with skip_confirm");
        assert!(key_store::load_key_from_home("to-delete", home).is_err());
    }

    #[tokio::test]
    async fn test_handle_key_delete_without_confirm_returns_early() {
        let dir = TempDir::new().expect("create temp directory for delete without confirm test");
        handle_key_delete_with_home("some-key", false, dir.path())
            .await
            .expect("delete without confirm returns early");
    }

    #[tokio::test]
    async fn test_handle_key_delete_missing_key_errors() {
        let dir = TempDir::new().expect("create temp directory for missing key delete test");
        let err = handle_key_delete_with_home("missing-id", true, dir.path())
            .await
            .unwrap_err();
        assert!(format!("{err}").contains("missing-id") || format!("{err}").contains("not found"));
    }
}
