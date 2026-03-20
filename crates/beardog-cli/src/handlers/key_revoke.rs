// SPDX-License-Identifier: AGPL-3.0-only

//! Sovereign key revocation: local revocation list in `~/.beardog/revocation_list.json`.

use beardog_errors::BearDogError;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Revocation list entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationEntry {
    /// Revoked key id
    pub key_id: String,
    /// When the revocation was recorded (RFC 3339)
    pub revoked_at: String,
    /// When the revocation becomes effective, if scheduled for the future
    pub effective_at: Option<String>,
    /// Optional human-readable reason
    pub reason: Option<String>,
    /// Local username that performed the revocation
    pub revoked_by: String,
    /// Whether child keys should be treated as revoked
    #[serde(default)]
    pub cascade: bool,
}

/// Revocation list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationList {
    /// Map of key id → revocation entry
    pub revoked_keys: HashMap<String, RevocationEntry>,
    /// Last modification time (RFC 3339)
    pub last_updated: String,
    /// Monotonic schema / merge generation
    pub version: u32,
}

impl RevocationList {
    /// Create new empty revocation list
    pub fn new() -> Self {
        Self {
            revoked_keys: HashMap::new(),
            last_updated: Utc::now().to_rfc3339(),
            version: 1,
        }
    }

    /// Load revocation list from disk (`$HOME/.beardog/revocation_list.json`).
    #[allow(dead_code)] // Stable public API; in-crate paths use `load_from_home`.
    pub fn load() -> Result<Self, BearDogError> {
        let home = std::env::var("HOME")
            .map_err(|_| BearDogError::system("HOME environment variable not set".to_string()))?;
        Self::load_from_home(home)
    }

    /// Load from a specific home directory (tests; no global env).
    pub fn load_from_home(home: impl AsRef<Path>) -> Result<Self, BearDogError> {
        Self::load_from_path(&Self::revocation_file_path_for_home(home))
    }

    fn load_from_path(path: &Path) -> Result<Self, BearDogError> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let json = fs::read_to_string(path)?;
        let list: Self = serde_json::from_str(&json).map_err(|e| {
            BearDogError::serialization(&format!("Failed to parse revocation list: {e}"))
        })?;

        Ok(list)
    }

    /// Save revocation list to disk (`$HOME/.beardog/revocation_list.json`).
    #[allow(dead_code)] // Stable public API; in-crate paths use `save_to_home`.
    pub fn save(&self) -> Result<(), BearDogError> {
        let home = std::env::var("HOME")
            .map_err(|_| BearDogError::system("HOME environment variable not set".to_string()))?;
        self.save_to_home(home)
    }

    /// Save under a specific home directory (tests).
    pub fn save_to_home(&self, home: impl AsRef<Path>) -> Result<(), BearDogError> {
        let path = Self::revocation_file_path_for_home(home);
        self.write_to_path(&path)
    }

    fn write_to_path(&self, path: &Path) -> Result<(), BearDogError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize revocation list: {e}"))
        })?;

        fs::write(path, json)?;
        Ok(())
    }

    /// Check if a key is revoked
    pub fn is_revoked(&self, key_id: &str) -> bool {
        self.revoked_keys.contains_key(key_id)
    }

    /// Revoke a key
    pub fn revoke(
        &mut self,
        key_id: String,
        reason: Option<String>,
        effective_at: Option<String>,
        cascade: bool,
    ) {
        let entry = RevocationEntry {
            key_id: key_id.clone(),
            revoked_at: Utc::now().to_rfc3339(),
            effective_at,
            reason,
            revoked_by: whoami::username(),
            cascade,
        };

        self.revoked_keys.insert(key_id, entry);
        self.last_updated = Utc::now().to_rfc3339();
        self.version += 1;
    }

    /// Export revocation list to file
    #[allow(dead_code)] // Used by handle_revocation_export (planned CLI command)
    pub fn export(&self, path: &str) -> Result<(), BearDogError> {
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize revocation list: {e}"))
        })?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Import revocation list from file
    #[allow(dead_code)] // Used by handle_revocation_import (planned CLI command)
    pub fn import(path: &str) -> Result<Self, BearDogError> {
        let json = fs::read_to_string(path)?;
        let list: Self = serde_json::from_str(&json).map_err(|e| {
            BearDogError::serialization(&format!("Failed to parse revocation list: {e}"))
        })?;
        Ok(list)
    }

    /// Merge another revocation list into this one
    #[allow(dead_code)] // Used by handle_revocation_import (planned CLI command)
    pub fn merge(&mut self, other: &Self) {
        for (key_id, entry) in &other.revoked_keys {
            // Only add if not already present or if other entry is newer
            if !self.revoked_keys.contains_key(key_id) {
                self.revoked_keys.insert(key_id.clone(), entry.clone());
            } else if let Some(existing) = self.revoked_keys.get(key_id) {
                // Compare timestamps - keep the newer one
                if entry.revoked_at > existing.revoked_at {
                    self.revoked_keys.insert(key_id.clone(), entry.clone());
                }
            }
        }
        self.last_updated = Utc::now().to_rfc3339();
        self.version += 1;
    }

    /// Unrevoke a key (for testing or if revocation was mistake)
    #[allow(dead_code)] // Admin/testing use
    pub fn unrevoke(&mut self, key_id: &str) -> bool {
        let removed = self.revoked_keys.remove(key_id).is_some();
        if removed {
            self.last_updated = Utc::now().to_rfc3339();
            self.version += 1;
        }
        removed
    }

    /// Revocation file path under a home directory (testable).
    pub(crate) fn revocation_file_path_for_home(home: impl AsRef<std::path::Path>) -> PathBuf {
        home.as_ref().join(".beardog").join("revocation_list.json")
    }
}

impl Default for RevocationList {
    fn default() -> Self {
        Self::new()
    }
}

fn revocation_home_from_env() -> Result<PathBuf, BearDogError> {
    std::env::var("HOME")
        .map(PathBuf::from)
        .map_err(|_| BearDogError::system("HOME environment variable not set".to_string()))
}

/// Handle key revocation command
pub async fn handle_key_revoke(
    key_id: &str,
    reason: Option<&str>,
    effective_at: Option<&str>,
    cascade: bool,
) -> Result<(), BearDogError> {
    let home = revocation_home_from_env()?;
    handle_key_revoke_with_home(key_id, reason, effective_at, cascade, &home).await
}

/// Like [`handle_key_revoke`] but uses an explicit home directory (tests).
pub async fn handle_key_revoke_with_home(
    key_id: &str,
    reason: Option<&str>,
    effective_at: Option<&str>,
    cascade: bool,
    home: &Path,
) -> Result<(), BearDogError> {
    println!("🚫 BearDog Key Revocation");
    println!("========================\n");

    // Load revocation list
    let mut revocation_list = RevocationList::load_from_home(home)?;

    // Check if already revoked
    if revocation_list.is_revoked(key_id) {
        println!("⚠️  Key '{key_id}' is already revoked");
        if let Some(entry) = revocation_list.revoked_keys.get(key_id) {
            println!("   Revoked at: {}", entry.revoked_at);
            if let Some(r) = &entry.reason {
                println!("   Reason: {r}");
            }
        }
        return Ok(());
    }

    // Revoke the key
    println!("🔒 Revoking key: {key_id}");
    revocation_list.revoke(
        key_id.to_string(),
        reason.map(std::string::ToString::to_string),
        effective_at.map(std::string::ToString::to_string),
        cascade,
    );

    // If cascade is enabled, revoke child keys
    if cascade {
        let child_keys = get_child_keys_in_home(key_id, home)?;
        if !child_keys.is_empty() {
            println!(
                "\n🔗 Cascading revocation to {} child keys:",
                child_keys.len()
            );
            for child_key in &child_keys {
                println!("   • {child_key}");
                revocation_list.revoke(
                    child_key.clone(),
                    Some(format!("Cascaded from parent: {key_id}")),
                    effective_at.map(std::string::ToString::to_string),
                    false, // Don't cascade recursively (already handled)
                );
            }
        }
    }

    // Save updated list
    revocation_list.save_to_home(home)?;

    println!("\n✅ Key revoked successfully!\n");

    println!("📋 Revocation Details:");
    println!("   Key ID: {key_id}");
    println!(
        "   Revoked At: {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
    if let Some(eff) = effective_at {
        println!("   Effective At: {eff}");
    }
    if let Some(r) = reason {
        println!("   Reason: {r}");
    }
    println!("   Revoked By: {}", whoami::username());
    println!("   Cascade: {}", if cascade { "Yes" } else { "No" });

    println!("\n🔐 What This Means:");
    println!("   ✅ Your tower: Will REFUSE to mix with this key immediately");
    println!("   ✅ Your operations: Protected right now");
    println!("   ⏱️  Other towers: Will learn via Songbird propagation (~1-5 minutes)");
    println!("   ⏱️  Offline keys: Will expire naturally (based on key's expiry)");

    println!("\n💡 Sovereign Revocation:");
    println!("   • Can't remote-delete the key (sovereignty!)");
    println!("   • Can refuse to cooperate with it ✅");
    println!("   • Propagates via Songbird (no central server)");
    println!("   • Bounded risk via key expiry");

    Ok(())
}

fn get_child_keys_in_home(parent_key_id: &str, home: &Path) -> Result<Vec<String>, BearDogError> {
    use super::key_store;

    let parent = key_store::load_key_from_home(parent_key_id, home)?;

    Ok(parent.children)
}

/// Handle revocation list export
#[allow(dead_code)] // Planned for CLI: beardog key revoke --export
pub async fn handle_revocation_export(output_path: &str) -> Result<(), BearDogError> {
    let home = revocation_home_from_env()?;
    handle_revocation_export_with_home(output_path, &home).await
}

#[allow(dead_code)]
pub async fn handle_revocation_export_with_home(
    output_path: &str,
    home: &Path,
) -> Result<(), BearDogError> {
    println!("📤 BearDog Revocation List Export");
    println!("==================================\n");

    let revocation_list = RevocationList::load_from_home(home)?;

    println!(
        "📋 Exporting {} revoked keys",
        revocation_list.revoked_keys.len()
    );
    revocation_list.export(output_path)?;

    println!("✅ Revocation list exported to: {output_path}");
    println!("\n💡 Share this file with other towers to propagate revocations");

    Ok(())
}

/// Handle revocation list import
#[allow(dead_code)] // Planned for CLI: beardog key revoke --import
pub async fn handle_revocation_import(input_path: &str) -> Result<(), BearDogError> {
    let home = revocation_home_from_env()?;
    handle_revocation_import_with_home(input_path, &home).await
}

#[allow(dead_code)]
pub async fn handle_revocation_import_with_home(
    input_path: &str,
    home: &Path,
) -> Result<(), BearDogError> {
    println!("📥 BearDog Revocation List Import");
    println!("==================================\n");

    let imported_list = RevocationList::import(input_path)?;

    println!(
        "📋 Imported list contains {} revoked keys",
        imported_list.revoked_keys.len()
    );

    // Load current list
    let mut current_list = RevocationList::load_from_home(home)?;
    let before_count = current_list.revoked_keys.len();

    // Merge
    current_list.merge(&imported_list);
    let after_count = current_list.revoked_keys.len();

    // Save
    current_list.save_to_home(home)?;

    println!("✅ Revocation list merged successfully!");
    println!("   Before: {before_count} revoked keys");
    println!("   After: {after_count} revoked keys");
    println!("   Added: {} new revocations", after_count - before_count);

    Ok(())
}

/// Handle revocation check command
pub async fn handle_key_check_revocation(key_id: &str) -> Result<(), BearDogError> {
    let home = revocation_home_from_env()?;
    handle_key_check_revocation_with_home(key_id, &home).await
}

pub async fn handle_key_check_revocation_with_home(
    key_id: &str,
    home: &Path,
) -> Result<(), BearDogError> {
    println!("🔍 BearDog Revocation Check");
    println!("========================\n");

    let revocation_list = RevocationList::load_from_home(home)?;

    println!("Checking key: {key_id}\n");

    if revocation_list.is_revoked(key_id) {
        if let Some(entry) = revocation_list.revoked_keys.get(key_id) {
            println!("❌ Key IS REVOKED\n");
            println!("📋 Revocation Details:");
            println!("   Revoked At: {}", entry.revoked_at);
            println!("   Revoked By: {}", entry.revoked_by);
            if let Some(reason) = &entry.reason {
                println!("   Reason: {reason}");
            }

            println!("\n⚠️  This key should NOT be used!");
            return Ok(());
        }
    }

    println!("✅ Key is NOT revoked\n");
    println!("   Safe to use for operations");
    println!(
        "   Last revocation list update: {}",
        revocation_list.last_updated
    );

    Ok(())
}

/// Handle listing all revoked keys
pub async fn handle_key_list_revocations() -> Result<(), BearDogError> {
    let home = revocation_home_from_env()?;
    handle_key_list_revocations_with_home(&home).await
}

pub async fn handle_key_list_revocations_with_home(home: &Path) -> Result<(), BearDogError> {
    println!("📋 BearDog Revocation List");
    println!("========================\n");

    let revocation_list = RevocationList::load_from_home(home)?;

    if revocation_list.revoked_keys.is_empty() {
        println!("✅ No keys are currently revoked");
        return Ok(());
    }

    println!("🚫 Revoked Keys: {}\n", revocation_list.revoked_keys.len());

    for (key_id, entry) in &revocation_list.revoked_keys {
        println!("  • {key_id}");
        println!("    Revoked: {}", entry.revoked_at);
        println!("    By: {}", entry.revoked_by);
        if let Some(reason) = &entry.reason {
            println!("    Reason: {reason}");
        }
        println!();
    }

    println!("📊 Summary:");
    println!("   Total revoked: {}", revocation_list.revoked_keys.len());
    println!("   Last updated: {}", revocation_list.last_updated);
    println!("   Version: {}", revocation_list.version);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use tempfile::TempDir;

    #[test]
    fn test_revocation_list_new() {
        let list = RevocationList::new();
        assert_eq!(list.revoked_keys.len(), 0);
        assert_eq!(list.version, 1);
    }

    #[test]
    fn test_revoke_key() {
        let mut list = RevocationList::new();

        assert!(!list.is_revoked("test-key"));

        list.revoke(
            "test-key".to_string(),
            Some("Testing".to_string()),
            None,
            false,
        );

        assert!(list.is_revoked("test-key"));
        assert_eq!(list.version, 2); // Incremented
        assert_eq!(list.revoked_keys.len(), 1);
    }

    #[test]
    fn test_unrevoke_key() {
        let mut list = RevocationList::new();

        list.revoke("test-key".to_string(), None, None, false);
        assert!(list.is_revoked("test-key"));

        let removed = list.unrevoke("test-key");
        assert!(removed);
        assert!(!list.is_revoked("test-key"));
        assert_eq!(list.version, 3); // Incremented twice
    }

    #[test]
    fn test_merge_revocation_lists() {
        let mut list1 = RevocationList::new();
        list1.revoke("key1".to_string(), None, None, false);

        let mut list2 = RevocationList::new();
        list2.revoke("key2".to_string(), None, None, false);

        list1.merge(&list2);

        assert!(list1.is_revoked("key1"));
        assert!(list1.is_revoked("key2"));
        assert_eq!(list1.revoked_keys.len(), 2);
    }

    #[test]
    fn test_unrevoke_nonexistent() {
        let mut list = RevocationList::new();
        let removed = list.unrevoke("nonexistent");
        assert!(!removed);
    }

    #[test]
    fn test_revocation_list_load_save_roundtrip_via_home() {
        let dir = TempDir::new().unwrap();

        let mut list = RevocationList::new();
        list.revoke(
            "roundtrip-key".to_string(),
            Some("test".to_string()),
            None,
            false,
        );
        list.save_to_home(dir.path()).expect("save");

        let loaded = RevocationList::load_from_home(dir.path()).expect("load");
        assert!(loaded.is_revoked("roundtrip-key"));
        assert_eq!(
            loaded.revoked_keys["roundtrip-key"].reason,
            Some("test".to_string())
        );
    }

    #[test]
    fn test_revocation_export_import_merge() {
        let dir = TempDir::new().unwrap();
        let export_a = dir.path().join("a.json");
        let mut a = RevocationList::new();
        a.revoke("ka".to_string(), None, None, false);
        a.export(export_a.to_str().unwrap()).unwrap();

        let mut b = RevocationList::new();
        b.revoke("kb".to_string(), None, None, false);
        b.merge(&RevocationList::import(export_a.to_str().unwrap()).unwrap());
        assert!(b.is_revoked("ka"));
        assert!(b.is_revoked("kb"));
    }

    #[test]
    fn test_merge_prefers_newer_timestamp() {
        let mut older = RevocationList::new();
        older.revoked_keys.insert(
            "k".to_string(),
            RevocationEntry {
                key_id: "k".to_string(),
                revoked_at: "2020-01-01T00:00:00Z".to_string(),
                effective_at: None,
                reason: Some("old".to_string()),
                revoked_by: "tester".to_string(),
                cascade: false,
            },
        );

        let mut newer = RevocationList::new();
        newer.revoked_keys.insert(
            "k".to_string(),
            RevocationEntry {
                key_id: "k".to_string(),
                revoked_at: "2025-01-01T00:00:00Z".to_string(),
                effective_at: None,
                reason: Some("newer".to_string()),
                revoked_by: "tester".to_string(),
                cascade: false,
            },
        );

        older.merge(&newer);
        assert_eq!(
            older.revoked_keys.get("k").unwrap().reason,
            Some("newer".to_string())
        );
    }

    #[tokio::test]
    async fn test_handle_key_check_revocation_not_revoked() {
        let dir = TempDir::new().unwrap();
        handle_key_check_revocation_with_home("fresh-key-id", dir.path())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_handle_key_list_revocations_empty() {
        let dir = TempDir::new().unwrap();
        handle_key_list_revocations_with_home(dir.path())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_handle_key_revoke_idempotent() {
        let dir = TempDir::new().unwrap();

        handle_key_revoke_with_home("dup-key", Some("r1"), None, false, dir.path())
            .await
            .unwrap();
        handle_key_revoke_with_home("dup-key", Some("r2"), None, false, dir.path())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_handle_key_revoke_with_cascade() {
        let dir = TempDir::new().unwrap();

        use crate::handlers::key_store;

        let parent = key_store::StoredKey {
            key_id: "parent-k".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "h".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[1u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec!["child-k".to_string()],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&parent, dir.path()).unwrap();

        handle_key_revoke_with_home("parent-k", Some("rotate"), None, true, dir.path())
            .await
            .unwrap();

        let list = RevocationList::load_from_home(dir.path()).unwrap();
        assert!(list.is_revoked("parent-k"));
        assert!(list.is_revoked("child-k"));
    }

    #[tokio::test]
    async fn test_handle_revocation_export_and_import_handlers() {
        let dir = TempDir::new().unwrap();

        let mut list = RevocationList::new();
        list.revoke("export-me".to_string(), None, None, false);
        list.save_to_home(dir.path()).unwrap();

        let export_path = dir.path().join("rev.json");
        handle_revocation_export_with_home(export_path.to_str().unwrap(), dir.path())
            .await
            .unwrap();

        let home2 = TempDir::new().unwrap();
        handle_revocation_import_with_home(export_path.to_str().unwrap(), home2.path())
            .await
            .unwrap();
        let merged = RevocationList::load_from_home(home2.path()).unwrap();
        assert!(merged.is_revoked("export-me"));
    }
}
