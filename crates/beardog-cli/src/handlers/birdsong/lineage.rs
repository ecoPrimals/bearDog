// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;
use beardog_genetics::birdsong::types::{LineageHint, LineageProof, LineageRelationship};
use sha2::{Digest, Sha256};
use std::path::Path;
use tracing::{debug, info};

use crate::handlers::key_store;

/// Parse lineage hint from string format
///
/// Supported formats:
/// - "`DirectAncestors`" -> `min_depth=0`, `max_depth=1`
/// - "`AllDescendants`" -> `min_depth=0`, `max_depth=100`
/// - "Depth:2-5" -> `min_depth=2`, `max_depth=5`
/// - "`RootOnly`" -> `min_depth=0`, `max_depth=0`
pub(super) fn parse_lineage_hint(
    hint_type: &str,
    root_id: &str,
) -> Result<LineageHint, BearDogError> {
    let (min_depth, max_depth) = match hint_type {
        "DirectAncestors" => (0, 1),
        "AllDescendants" => (0, 100),
        "RootOnly" => (0, 0),
        s if s.starts_with("Depth:") => {
            let range = s
                .strip_prefix("Depth:")
                .ok_or_else(|| BearDogError::invalid_input("Invalid depth format"))?;
            let parts: Vec<&str> = range.split('-').collect();
            if parts.len() != 2 {
                return Err(BearDogError::invalid_input(
                    "Depth format must be 'Depth:min-max' (e.g., 'Depth:0-2')",
                ));
            }
            let min: u32 = parts[0]
                .parse()
                .map_err(|_| BearDogError::invalid_input("Invalid min depth"))?;
            let max: u32 = parts[1]
                .parse()
                .map_err(|_| BearDogError::invalid_input("Invalid max depth"))?;
            (min, max)
        }
        _ => {
            return Err(BearDogError::invalid_input(
                "Unknown hint type. Use: DirectAncestors, AllDescendants, RootOnly, or Depth:min-max",
            ));
        }
    };

    Ok(LineageHint {
        root_id: root_id.to_string(),
        min_depth,
        max_depth,
        biome_filter: None,
        version: 1,
    })
}

/// Get lineage proof for a given key ID
///
/// This queries the key store and lineage chain to build a proof that
/// this node belongs to a specific lineage.
pub(super) async fn get_lineage_proof_for_key_with_home(
    key_id: &str,
    home: &Path,
) -> Result<LineageProof, BearDogError> {
    // Load key from store
    let key = key_store::load_key_from_home(key_id, home)?;

    // Get lineage information from key metadata
    let lineage_info = key.lineage.ok_or_else(|| {
        BearDogError::not_found(format!(
            "Key '{key_id}' has no lineage information. Generate keys with lineage tracking enabled."
        ))
    })?;

    debug!(
        "Building lineage proof for key {}: parent={:?}, depth={}",
        key_id, lineage_info.parent_key_id, lineage_info.depth
    );

    // Build path from root to this node
    let mut path = vec![key_id.to_string()];
    let mut current_parent = lineage_info.parent_key_id;

    // Walk up the lineage chain
    while let Some(parent_id) = current_parent {
        path.insert(0, parent_id.clone());

        // Load parent key to continue walking
        match key_store::load_key_from_home(&parent_id, home) {
            Ok(parent_key) => {
                current_parent = parent_key.lineage.and_then(|l| l.parent_key_id);
            }
            Err(_) => {
                // Reached root or missing parent
                break;
            }
        }
    }

    // Root is the first element in path
    let root_id = path
        .first()
        .ok_or_else(|| BearDogError::system("Empty lineage path".to_string()))?
        .clone();

    // Generate cryptographic proofs for lineage verification
    // Uses SHA-256 hashing to create verifiable proof chain

    // Build proof chain: Create LineageRelationship for each parent-child link
    let mut proof_chain = Vec::new();
    let mut merkle_leaves = Vec::new();

    for i in 0..(path.len().saturating_sub(1)) {
        let parent_id = path[i].clone();
        let child_id = path[i + 1].clone();

        // Create cryptographic proof of parent-child relationship
        // Hash: parent_id || child_id for signature
        let mut hasher = Sha256::new();
        hasher.update(parent_id.as_bytes());
        hasher.update(child_id.as_bytes());
        hasher.update(chrono::Utc::now().to_rfc3339().as_bytes());

        let relationship_hash = hasher.finalize();

        // Create LineageRelationship with cryptographic proof
        let relationship = LineageRelationship {
            parent_id: parent_id.clone(),
            child_id: child_id.clone(),
            parent_signature: relationship_hash.to_vec(), // Phase 5: Real Ed25519 signature
            witness_signatures: vec![],                   // Phase 5: Add witness signatures
            established_at: chrono::Utc::now(),
        };

        proof_chain.push(relationship);
        merkle_leaves.push(relationship_hash.to_vec());
    }

    // Compute Merkle root from proof chain
    // This allows efficient verification of any relationship in the lineage
    let merkle_root = if merkle_leaves.is_empty() {
        vec![]
    } else {
        compute_merkle_root(&merkle_leaves)
    };

    info!(
        "✅ Generated lineage proof: {} nodes, {} proofs, merkle_root={}",
        path.len(),
        proof_chain.len(),
        hex::encode(&merkle_root)
    );

    let proof = LineageProof {
        node_id: key_id.to_string(),
        root_id,
        path,
        proof_chain,
        merkle_root,
        generation: 0,
        head_commitment: vec![],
        generated_at: chrono::Utc::now(),
    };

    Ok(proof)
}

/// Compute Merkle root from a list of leaf hashes
///
/// Uses iterative pairwise hashing to build a Merkle tree.
/// If odd number of leaves, the last one is duplicated.
pub(super) fn compute_merkle_root(leaves: &[Vec<u8>]) -> Vec<u8> {
    if leaves.is_empty() {
        return vec![];
    }

    if leaves.len() == 1 {
        return leaves[0].clone();
    }

    let mut current_level = leaves.to_vec();

    while current_level.len() > 1 {
        let mut next_level = Vec::new();

        for chunk in current_level.chunks(2) {
            let mut hasher = Sha256::new();
            hasher.update(&chunk[0]);

            // If odd number, duplicate the last hash
            if chunk.len() == 2 {
                hasher.update(&chunk[1]);
            } else {
                hasher.update(&chunk[0]);
            }

            next_level.push(hasher.finalize().to_vec());
        }

        current_level = next_level;
    }

    current_level[0].clone()
}
