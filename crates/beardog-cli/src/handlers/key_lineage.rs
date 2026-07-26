// SPDX-License-Identifier: AGPL-3.0-or-later

//! Display key parent/child relationships as text or JSON.

use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Lineage node for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    /// Key identifier
    pub key_id: String,
    /// Cryptographic algorithm name
    pub algorithm: String,
    /// Derivation generation (0 = root)
    pub generation: u32,
    /// Parent key id, if derived
    pub parent_key_id: Option<String>,
    /// HKDF / derivation context label
    pub derivation_purpose: Option<String>,
    /// Creation time (RFC 3339)
    pub created_at: String,
    /// Optional expiry (RFC 3339)
    pub expires_at: Option<String>,
    /// Child keys in the tree
    pub children: Vec<Self>,
}

/// Lineage summary for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageSummary {
    /// Key id passed to the CLI
    pub requested_key: String,
    /// Root ancestor key id
    pub root_key: String,
    /// Total keys in the subtree
    pub total_keys: usize,
    /// Maximum depth from root
    pub max_depth: usize,
    /// Recursive tree rooted at `root_key`
    pub lineage_tree: LineageNode,
}

/// Handle key lineage query command
///
/// # Errors
///
/// Returns an error if the key home cannot be resolved, or lineage tree construction fails.
pub async fn handle_key_lineage(key_id: &str, json: bool) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_lineage_for_home(key_id, json, home.as_path())
}

/// Same as [`handle_key_lineage`], but uses keys under `home/.beardog/keys` (tests / DI).
///
/// # Errors
///
/// Returns an error if the key cannot be loaded, the lineage root cannot be found, or the tree
/// cannot be built.
pub fn handle_key_lineage_for_home(
    key_id: &str,
    json: bool,
    home: &Path,
) -> Result<(), BearDogError> {
    // Load the requested key
    let key = key_store::load_key_from_home(key_id, home)?;

    // Find root ancestor
    let root = find_root(&key, home)?;

    if json {
        // JSON output (machine-readable)
        let lineage_tree = build_lineage_tree(&root, home)?;
        let (total_keys, max_depth) = count_lineage(&root, 0, home);

        let summary = LineageSummary {
            requested_key: key_id.to_string(),
            root_key: root.key_id,
            total_keys,
            max_depth,
            lineage_tree,
        };

        let json = serde_json::to_string_pretty(&summary).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize lineage: {e}"))
        })?;
        println!("{json}");
    } else {
        // Human-readable output
        println!("🌳 BearDog Key Lineage");
        println!("========================\n");
        println!("📋 Lineage for key: {key_id}\n");

        // Display full lineage tree
        display_lineage_tree(&root, key_id, 0, home)?;

        println!("\n📊 Lineage Summary:");
        let (total_keys, max_depth) = count_lineage(&root, 0, home);
        println!("   Total keys in lineage: {total_keys}");
        println!("   Maximum depth: {max_depth}");
        println!("   Root key: {}", root.key_id);
    }

    Ok(())
}

/// Find the root ancestor of a key
fn find_root(key: &StoredKey, home: &Path) -> Result<StoredKey, BearDogError> {
    let mut current = key.clone();

    // Walk up the parent chain until we find a root (no parent)
    while let Some(parent_id) = &current.parent_key_id {
        // Handle mixed keys (parent format: "key1+key2")
        let first_parent = if parent_id.contains('+') {
            parent_id.split('+').next().unwrap_or(parent_id.as_str())
        } else {
            parent_id.as_str()
        };

        match key_store::load_key_from_home(first_parent, home) {
            Ok(parent) => current = parent,
            Err(_) => break, // Parent not found, treat current as root
        }
    }

    Ok(current)
}

/// Display lineage tree recursively
fn display_lineage_tree(
    key: &StoredKey,
    highlight_key: &str,
    depth: usize,
    home: &Path,
) -> Result<(), BearDogError> {
    let indent = "  ".repeat(depth);
    let branch = if depth > 0 { "├─ " } else { "" };

    // Highlight the requested key
    let highlight = if key.key_id == highlight_key {
        " ← YOU ARE HERE"
    } else {
        ""
    };

    // Display key info
    println!(
        "{}{}Gen {}: {}{}",
        indent, branch, key.generation, key.key_id, highlight
    );

    // Show metadata
    if let Some(purpose) = &key.derivation_purpose {
        println!("{indent}   Purpose: {purpose}");
    }
    if let Some(parent) = &key.parent_key_id {
        println!("{indent}   Parent: {parent}");
    }
    if let Some(expires) = &key.expires_at {
        println!("{indent}   Expires: {expires}");
    }

    // Display children recursively
    if !key.children.is_empty() {
        println!("{indent}   Children:");
        for child_id in &key.children {
            match key_store::load_key_from_home(child_id, home) {
                Ok(child) => {
                    display_lineage_tree(&child, highlight_key, depth + 1, home)?;
                }
                Err(_) => {
                    // Child key not found (maybe deleted)
                    println!("{}  ├─ {} (not found)", "  ".repeat(depth + 1), child_id);
                }
            }
        }
    }

    Ok(())
}

/// Count total keys and maximum depth in lineage
fn count_lineage(key: &StoredKey, current_depth: usize, home: &Path) -> (usize, usize) {
    let mut total = 1; // Count this key
    let mut max_depth = current_depth;

    // Recursively count children
    for child_id in &key.children {
        if let Ok(child) = key_store::load_key_from_home(child_id, home) {
            let (child_total, child_depth) = count_lineage(&child, current_depth + 1, home);
            total += child_total;
            max_depth = max_depth.max(child_depth);
        }
    }

    (total, max_depth)
}

/// Build lineage tree for JSON output
fn build_lineage_tree(key: &StoredKey, home: &Path) -> Result<LineageNode, BearDogError> {
    let mut children = Vec::new();

    // Recursively build children
    for child_id in &key.children {
        if let Ok(child) = key_store::load_key_from_home(child_id, home) {
            children.push(build_lineage_tree(&child, home)?);
        }
    }

    Ok(LineageNode {
        key_id: key.key_id.clone(),
        algorithm: key.algorithm.clone(),
        generation: key.generation,
        parent_key_id: key.parent_key_id.clone(),
        derivation_purpose: key.derivation_purpose.clone(),
        created_at: key.created_at.clone(),
        expires_at: key.expires_at.clone(),
        children,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        StoredKey, build_lineage_tree, count_lineage, find_root, handle_key_lineage_for_home,
        key_store,
    };
    use chrono::Utc;
    use tempfile::TempDir;

    fn create_test_key(key_id: &str, generation: u32, parent_id: Option<String>) -> StoredKey {
        StoredKey {
            key_id: key_id.to_string(),
            algorithm: "aes-256-gcm".to_string(),
            hsm_name: "test".to_string(),
            key_material_b64: "dGVzdA==".to_string(), // "test" in base64
            created_at: Utc::now().to_rfc3339(),
            generation,
            parent_key_id: parent_id.clone(),
            derivation_purpose: Some("test".to_string()),
            children: Vec::new(),
            lineage: Some(key_store::KeyLineageInfo {
                parent_key_id: parent_id,
                depth: generation,
            }),
            expires_at: None,
            usage: None,
            purpose: None,
        }
    }

    #[test]
    fn test_find_root_already_root() {
        let dir = TempDir::new().expect("create temp directory for find_root root test");
        let home = dir.path();
        let root = create_test_key("root", 0, None);
        let found_root = find_root(&root, home).expect("find_root when key is already root");
        assert_eq!(found_root.key_id, "root");
    }

    #[test]
    fn test_find_root_walks_single_parent() {
        let dir = TempDir::new().expect("create temp directory for find_root walk test");
        let home = dir.path();
        let mut root = create_test_key("line-root", 0, None);
        root.children = vec!["line-child".to_string()];
        let child = create_test_key("line-child", 1, Some("line-root".to_string()));
        key_store::save_key_to_home(&root, home).expect("save line-root");
        key_store::save_key_to_home(&child, home).expect("save line-child");

        let found = find_root(&child, home).expect("walk parent chain to root");
        assert_eq!(found.key_id, "line-root");
    }

    #[test]
    fn test_find_root_mixed_parent_uses_first_segment() {
        let dir = TempDir::new().expect("create temp directory for mixed parent test");
        let home = dir.path();
        let k1 = create_test_key("mix-a", 0, None);
        let k2 = create_test_key("mix-b", 0, None);
        key_store::save_key_to_home(&k1, home).expect("save mix-a");
        key_store::save_key_to_home(&k2, home).expect("save mix-b");

        let mixed = create_test_key("mixed-child", 2, Some("mix-a+mix-b".to_string()));
        key_store::save_key_to_home(&mixed, home).expect("save mixed-child");

        let found = find_root(&mixed, home).expect("resolve mixed parent to first segment root");
        assert_eq!(found.key_id, "mix-a");
    }

    #[test]
    fn test_count_lineage_single_key() {
        let dir = TempDir::new().expect("create temp directory for count_lineage single key");
        let home = dir.path();
        let key = create_test_key("single", 0, None);
        let (total, depth) = count_lineage(&key, 0, home);
        assert_eq!(total, 1);
        assert_eq!(depth, 0);
    }

    #[test]
    fn test_count_lineage_with_children() {
        let dir = TempDir::new().expect("create temp directory for count_lineage children test");
        let home = dir.path();
        let mut parent = create_test_key("parent", 0, None);
        let c1 = create_test_key("child1", 1, Some("parent".to_string()));
        let c2 = create_test_key("child2", 1, Some("parent".to_string()));
        parent.children = vec!["child1".to_string(), "child2".to_string()];
        key_store::save_key_to_home(&parent, home).expect("save parent");
        key_store::save_key_to_home(&c1, home).expect("save child1");
        key_store::save_key_to_home(&c2, home).expect("save child2");

        let (total, depth) = count_lineage(&parent, 0, home);
        assert_eq!(total, 3);
        assert_eq!(depth, 1);
    }

    #[tokio::test]
    async fn test_handle_key_lineage_json_includes_tree() {
        let dir = TempDir::new().expect("create temp directory for lineage JSON test");
        let home = dir.path();
        let root = create_test_key("json-root", 0, None);
        let child = create_test_key("json-child", 1, Some("json-root".to_string()));
        key_store::save_key_to_home(&root, home).expect("save json-root");
        key_store::save_key_to_home(&child, home).expect("save json-child");

        handle_key_lineage_for_home("json-child", true, home)
            .expect("handle_key_lineage JSON output");
    }

    #[tokio::test]
    async fn test_handle_key_lineage_human_output_smoke() {
        let dir = TempDir::new().expect("create temp directory for lineage human output test");
        let home = dir.path();
        let k = create_test_key("human-only", 0, None);
        key_store::save_key_to_home(&k, home).expect("save human-only key");

        handle_key_lineage_for_home("human-only", false, home)
            .expect("handle_key_lineage human-readable output");
    }

    #[test]
    fn test_build_lineage_tree_nested() {
        let dir = TempDir::new().expect("create temp directory for build_lineage_tree test");
        let home = dir.path();
        let mut root = create_test_key("bt-root", 0, None);
        let child = create_test_key("bt-child", 1, Some("bt-root".to_string()));
        root.children = vec!["bt-child".to_string()];
        key_store::save_key_to_home(&root, home).expect("save bt-root");
        key_store::save_key_to_home(&child, home).expect("save bt-child");

        let node = build_lineage_tree(&root, home).expect("build nested lineage tree");
        assert_eq!(node.key_id, "bt-root");
        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0].key_id, "bt-child");
    }
}
