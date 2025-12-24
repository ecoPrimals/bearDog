// Key Lineage Handler
// Query and display key parent-child relationships

use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use serde::{Deserialize, Serialize};

/// Lineage node for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    pub key_id: String,
    pub algorithm: String,
    pub generation: u32,
    pub parent_key_id: Option<String>,
    pub derivation_purpose: Option<String>,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub children: Vec<LineageNode>,
}

/// Lineage summary for JSON output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageSummary {
    pub requested_key: String,
    pub root_key: String,
    pub total_keys: usize,
    pub max_depth: usize,
    pub lineage_tree: LineageNode,
}

/// Handle key lineage query command
pub async fn handle_key_lineage(key_id: &str, json: bool) -> Result<(), BearDogError> {
    // Load the requested key
    let key = key_store::load_key(key_id)?;

    // Find root ancestor
    let root = find_root(&key)?;

    if json {
        // JSON output (machine-readable)
        let lineage_tree = build_lineage_tree(&root)?;
        let (total_keys, max_depth) = count_lineage(&root, 0);

        let summary = LineageSummary {
            requested_key: key_id.to_string(),
            root_key: root.key_id.clone(),
            total_keys,
            max_depth,
            lineage_tree,
        };

        let json = serde_json::to_string_pretty(&summary).map_err(|e| {
            BearDogError::serialization(&format!("Failed to serialize lineage: {}", e))
        })?;
        println!("{}", json);
    } else {
        // Human-readable output
        println!("🌳 BearDog Key Lineage");
        println!("========================\n");
        println!("📋 Lineage for key: {}\n", key_id);

        // Display full lineage tree
        display_lineage_tree(&root, key_id, 0)?;

        println!("\n📊 Lineage Summary:");
        let (total_keys, max_depth) = count_lineage(&root, 0);
        println!("   Total keys in lineage: {}", total_keys);
        println!("   Maximum depth: {}", max_depth);
        println!("   Root key: {}", root.key_id);
    }

    Ok(())
}

/// Find the root ancestor of a key
fn find_root(key: &StoredKey) -> Result<StoredKey, BearDogError> {
    let mut current = key.clone();

    // Walk up the parent chain until we find a root (no parent)
    while let Some(parent_id) = &current.parent_key_id {
        // Handle mixed keys (parent format: "key1+key2")
        let first_parent = if parent_id.contains('+') {
            parent_id.split('+').next().unwrap_or(parent_id)
        } else {
            parent_id.as_str()
        };

        match key_store::load_key(first_parent) {
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
        println!("{}   Purpose: {}", indent, purpose);
    }
    if let Some(parent) = &key.parent_key_id {
        println!("{}   Parent: {}", indent, parent);
    }
    if let Some(expires) = &key.expires_at {
        println!("{}   Expires: {}", indent, expires);
    }

    // Display children recursively
    if !key.children.is_empty() {
        println!("{}   Children:", indent);
        for child_id in &key.children {
            match key_store::load_key(child_id) {
                Ok(child) => {
                    display_lineage_tree(&child, highlight_key, depth + 1)?;
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
fn count_lineage(key: &StoredKey, current_depth: usize) -> (usize, usize) {
    let mut total = 1; // Count this key
    let mut max_depth = current_depth;

    // Recursively count children
    for child_id in &key.children {
        if let Ok(child) = key_store::load_key(child_id) {
            let (child_total, child_depth) = count_lineage(&child, current_depth + 1);
            total += child_total;
            max_depth = max_depth.max(child_depth);
        }
    }

    (total, max_depth)
}

/// Build lineage tree for JSON output
fn build_lineage_tree(key: &StoredKey) -> Result<LineageNode, BearDogError> {
    let mut children = Vec::new();

    // Recursively build children
    for child_id in &key.children {
        if let Ok(child) = key_store::load_key(child_id) {
            children.push(build_lineage_tree(&child)?);
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
    use super::*;
    use chrono::Utc;

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
        let root = create_test_key("root", 0, None);
        let found_root = find_root(&root).unwrap();
        assert_eq!(found_root.key_id, "root");
    }

    #[test]
    fn test_count_lineage_single_key() {
        let key = create_test_key("single", 0, None);
        let (total, depth) = count_lineage(&key, 0);
        assert_eq!(total, 1);
        assert_eq!(depth, 0);
    }

    #[test]
    fn test_count_lineage_with_children() {
        let mut parent = create_test_key("parent", 0, None);
        parent.children = vec!["child1".to_string(), "child2".to_string()];

        // Note: In real scenario, children would need to exist in key_store
        // This just tests the counting logic
        let (total, depth) = count_lineage(&parent, 0);
        assert_eq!(total, 1); // Only parent counts (children not in store)
        assert_eq!(depth, 0);
    }
}
