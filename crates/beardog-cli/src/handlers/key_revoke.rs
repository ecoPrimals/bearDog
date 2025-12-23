// Key Revocation Handler
// Sovereign revocation system (no phone home required)

use beardog_errors::BearDogError;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Revocation list entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationEntry {
    pub key_id: String,
    pub revoked_at: String,
    pub effective_at: Option<String>, // When revocation becomes effective (future revocation)
    pub reason: Option<String>,
    pub revoked_by: String, // Who revoked it
    #[serde(default)] // Default to false if not present (backward compatibility)
    pub cascade: bool, // Whether to cascade to child keys
}

/// Revocation list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationList {
    pub revoked_keys: HashMap<String, RevocationEntry>,
    pub last_updated: String,
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

    /// Load revocation list from disk
    pub fn load() -> Result<Self, BearDogError> {
        let path = Self::get_revocation_file_path()?;

        if !path.exists() {
            return Ok(Self::new());
        }

        let json = fs::read_to_string(path)?;
        let list: RevocationList = serde_json::from_str(&json).map_err(|e| {
            BearDogError::serialization(&format!("Failed to parse revocation list: {}", e))
        })?;

        Ok(list)
    }

    /// Save revocation list to disk
    pub fn save(&self) -> Result<(), BearDogError> {
        let path = Self::get_revocation_file_path()?;

        // Create directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize revocation list: {}", e))
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
    pub fn export(&self, path: &str) -> Result<(), BearDogError> {
        let json = serde_json::to_string_pretty(self).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize revocation list: {}", e))
        })?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Import revocation list from file
    pub fn import(path: &str) -> Result<Self, BearDogError> {
        let json = fs::read_to_string(path)?;
        let list: RevocationList = serde_json::from_str(&json).map_err(|e| {
            BearDogError::serialization(&format!("Failed to parse revocation list: {}", e))
        })?;
        Ok(list)
    }

    /// Merge another revocation list into this one
    pub fn merge(&mut self, other: &RevocationList) {
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
    pub fn unrevoke(&mut self, key_id: &str) -> bool {
        let removed = self.revoked_keys.remove(key_id).is_some();
        if removed {
            self.last_updated = Utc::now().to_rfc3339();
            self.version += 1;
        }
        removed
    }

    /// Get revocation file path
    fn get_revocation_file_path() -> Result<PathBuf, BearDogError> {
        let home = std::env::var("HOME")
            .map_err(|_| BearDogError::system("HOME environment variable not set".to_string()))?;

        Ok(PathBuf::from(home)
            .join(".beardog")
            .join("revocation_list.json"))
    }
}

impl Default for RevocationList {
    fn default() -> Self {
        Self::new()
    }
}

/// Handle key revocation command
pub async fn handle_key_revoke(
    key_id: &str,
    reason: Option<&str>,
    effective_at: Option<&str>,
    cascade: bool,
) -> Result<(), BearDogError> {
    println!("🚫 BearDog Key Revocation");
    println!("========================\n");

    // Load revocation list
    let mut revocation_list = RevocationList::load()?;

    // Check if already revoked
    if revocation_list.is_revoked(key_id) {
        println!("⚠️  Key '{}' is already revoked", key_id);
        if let Some(entry) = revocation_list.revoked_keys.get(key_id) {
            println!("   Revoked at: {}", entry.revoked_at);
            if let Some(r) = &entry.reason {
                println!("   Reason: {}", r);
            }
        }
        return Ok(());
    }

    // Revoke the key
    println!("🔒 Revoking key: {}", key_id);
    revocation_list.revoke(
        key_id.to_string(),
        reason.map(|s| s.to_string()),
        effective_at.map(|s| s.to_string()),
        cascade,
    );

    // If cascade is enabled, revoke child keys
    if cascade {
        let child_keys = get_child_keys(key_id)?;
        if !child_keys.is_empty() {
            println!(
                "\n🔗 Cascading revocation to {} child keys:",
                child_keys.len()
            );
            for child_key in &child_keys {
                println!("   • {}", child_key);
                revocation_list.revoke(
                    child_key.clone(),
                    Some(format!("Cascaded from parent: {}", key_id)),
                    effective_at.map(|s| s.to_string()),
                    false, // Don't cascade recursively (already handled)
                );
            }
        }
    }

    // Save updated list
    revocation_list.save()?;

    println!("\n✅ Key revoked successfully!\n");

    println!("📋 Revocation Details:");
    println!("   Key ID: {}", key_id);
    println!(
        "   Revoked At: {}",
        Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
    if let Some(eff) = effective_at {
        println!("   Effective At: {}", eff);
    }
    if let Some(r) = reason {
        println!("   Reason: {}", r);
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

/// Get all child keys of a parent key
fn get_child_keys(parent_key_id: &str) -> Result<Vec<String>, BearDogError> {
    use super::key_store;

    // Load parent key
    let parent = key_store::load_key(parent_key_id)?;

    // Return children list
    Ok(parent.children)
}

/// Handle revocation list export
pub async fn handle_revocation_export(output_path: &str) -> Result<(), BearDogError> {
    println!("📤 BearDog Revocation List Export");
    println!("==================================\n");

    let revocation_list = RevocationList::load()?;

    println!(
        "📋 Exporting {} revoked keys",
        revocation_list.revoked_keys.len()
    );
    revocation_list.export(output_path)?;

    println!("✅ Revocation list exported to: {}", output_path);
    println!("\n💡 Share this file with other towers to propagate revocations");

    Ok(())
}

/// Handle revocation list import
pub async fn handle_revocation_import(input_path: &str) -> Result<(), BearDogError> {
    println!("📥 BearDog Revocation List Import");
    println!("==================================\n");

    let imported_list = RevocationList::import(input_path)?;

    println!(
        "📋 Imported list contains {} revoked keys",
        imported_list.revoked_keys.len()
    );

    // Load current list
    let mut current_list = RevocationList::load()?;
    let before_count = current_list.revoked_keys.len();

    // Merge
    current_list.merge(&imported_list);
    let after_count = current_list.revoked_keys.len();

    // Save
    current_list.save()?;

    println!("✅ Revocation list merged successfully!");
    println!("   Before: {} revoked keys", before_count);
    println!("   After: {} revoked keys", after_count);
    println!("   Added: {} new revocations", after_count - before_count);

    Ok(())
}

/// Handle revocation check command
pub async fn handle_key_check_revocation(key_id: &str) -> Result<(), BearDogError> {
    println!("🔍 BearDog Revocation Check");
    println!("========================\n");

    let revocation_list = RevocationList::load()?;

    println!("Checking key: {}\n", key_id);

    if revocation_list.is_revoked(key_id) {
        if let Some(entry) = revocation_list.revoked_keys.get(key_id) {
            println!("❌ Key IS REVOKED\n");
            println!("📋 Revocation Details:");
            println!("   Revoked At: {}", entry.revoked_at);
            println!("   Revoked By: {}", entry.revoked_by);
            if let Some(reason) = &entry.reason {
                println!("   Reason: {}", reason);
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
    println!("📋 BearDog Revocation List");
    println!("========================\n");

    let revocation_list = RevocationList::load()?;

    if revocation_list.revoked_keys.is_empty() {
        println!("✅ No keys are currently revoked");
        return Ok(());
    }

    println!("🚫 Revoked Keys: {}\n", revocation_list.revoked_keys.len());

    for (key_id, entry) in &revocation_list.revoked_keys {
        println!("  • {}", key_id);
        println!("    Revoked: {}", entry.revoked_at);
        println!("    By: {}", entry.revoked_by);
        if let Some(reason) = &entry.reason {
            println!("    Reason: {}", reason);
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
}
