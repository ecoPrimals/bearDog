// SPDX-License-Identifier: AGPL-3.0-or-later

//! BirdSong lineage-based encryption CLI handlers
//!
//! Implements privacy-preserving encryption where only lineage members can decrypt.

use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{
    encryption::BirdSongEncryption,
    key_derivation::LineageKeyDerivation,
    types::{
        BirdSongBroadcast, BirdSongDecryptRequest, BirdSongEncryptRequest, LineageHint,
        LineageProof, LineageRelationship,
    },
};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info};

use super::key_store;

/// Handle `BirdSong` encrypt command
///
/// Encrypts a message for a specific lineage only. Only nodes within the
/// lineage (based on hint) can decrypt the message.
///
/// # Arguments
///
/// * `message` - Plaintext message to encrypt
/// * `hint_type` - Type of lineage hint ("`DirectAncestors`", "`AllDescendants`", "Depth:0-2", etc.)
/// * `root_id` - Root lineage ID
/// * `output` - Output file path for encrypted broadcast
///
/// # Errors
///
/// Returns an error if the key home directory cannot be resolved, the lineage hint is invalid,
/// the root key cannot be loaded or decoded, lineage KDF setup or encryption fails, serialization
/// fails, or the output file cannot be written.
pub async fn handle_birdsong_encrypt(
    message: &str,
    hint_type: &str,
    root_id: &str,
    output: Option<&str>,
) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_birdsong_encrypt_with_home(message, hint_type, root_id, output, &home).await
}

/// Same as [`handle_birdsong_encrypt`] but with an explicit home directory for the key store (tests / DI).
///
/// # Errors
///
/// Returns an error if the lineage hint is invalid, the root key cannot be loaded or decoded,
/// lineage KDF setup or encryption fails, serialization fails, or the output file cannot be written.
pub async fn handle_birdsong_encrypt_with_home(
    message: &str,
    hint_type: &str,
    root_id: &str,
    output: Option<&str>,
    home: impl AsRef<Path>,
) -> Result<(), BearDogError> {
    let home = home.as_ref();
    info!("🎵 BirdSong Lineage Encryption");
    println!("================================");
    println!();
    println!("📋 Configuration:");
    println!("   Message: {} bytes", message.len());
    println!("   Lineage Hint: {hint_type}");
    println!("   Root ID: {root_id}");
    println!();

    // Parse hint type into LineageHint
    let hint = parse_lineage_hint(hint_type, root_id)?;

    debug!(
        "Parsed lineage hint: root={}, depth={}-{}",
        hint.root_id, hint.min_depth, hint.max_depth
    );

    // Create encryption request
    let request = BirdSongEncryptRequest {
        plaintext: message.as_bytes().to_vec(),
        lineage_hint: hint.clone(),
        associated_data: None,
    };

    // Load root key to get master secret for lineage-based key derivation
    println!("🔑 Loading root key {root_id}...");
    let root_key = key_store::load_key_from_home(root_id, home)?;
    let root_key_material = key_store::base64_decode(&root_key.key_material_b64)?;

    // Use root key material as master secret for lineage derivation
    // All nodes in the lineage will derive from this same root
    let master_secret = if root_key_material.len() >= 32 {
        root_key_material[..32].to_vec()
    } else {
        // If key is smaller, pad with HKDF expand
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&root_key_material);
        hasher.update(b"beardog-birdsong-master-secret-v1");
        hasher.finalize().to_vec()
    };

    // Initialize BirdSong encryption with root-derived master secret
    let kdf = Arc::new(LineageKeyDerivation::new(master_secret)?);
    let encryption = BirdSongEncryption::new(kdf);

    // Encrypt the broadcast
    println!("🔒 Encrypting broadcast for lineage {root_id}...");
    let broadcast = encryption.encrypt(&request)?;

    // Serialize broadcast to file
    let output_path: &str = match output {
        Some(p) => p,
        None => "encrypted.birdsong",
    };
    let serialized = serde_json::to_vec_pretty(&broadcast)
        .map_err(|e| BearDogError::system(format!("Serialization failed: {e}")))?;

    fs::write(output_path, &serialized)
        .map_err(|e| BearDogError::system(format!("Failed to write file: {e}")))?;

    println!("✅ Broadcast encrypted successfully!");
    println!();
    println!("📁 Output:");
    println!("   File: {output_path}");
    println!("   Size: {} bytes", serialized.len());
    println!("   Ciphertext: {} bytes", broadcast.ciphertext.len());
    println!();
    println!("🔐 Privacy:");
    println!("   Only nodes in lineage '{root_id}' can decrypt");
    println!(
        "   Allowed depth range: {}-{}",
        hint.min_depth, hint.max_depth
    );
    println!("   Strangers will see: 'Cannot decrypt: not in lineage'");
    println!();

    Ok(())
}

/// Handle `BirdSong` decrypt command
///
/// Attempts to decrypt a `BirdSong` broadcast. Returns success only if the
/// current node is authorized based on its lineage proof.
///
/// # Arguments
///
/// * `input` - Input file path (encrypted broadcast)
/// * `key_id` - Key ID to use for decryption (determines lineage proof)
///
/// # Errors
///
/// Returns an error if the key home directory cannot be resolved, or decryption fails (including
/// lineage mismatch, invalid broadcast, I/O, or crypto errors).
pub async fn handle_birdsong_decrypt(input: &str, key_id: &str) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_birdsong_decrypt_with_home(input, key_id, &home).await
}

/// Same as [`handle_birdsong_decrypt`] but with an explicit home directory for the key store (tests / DI).
///
/// # Errors
///
/// Returns an error if the broadcast file cannot be read or parsed, lineage proof lookup fails,
/// lineages mismatch, or decryption fails.
pub async fn handle_birdsong_decrypt_with_home(
    input: &str,
    key_id: &str,
    home: impl AsRef<Path>,
) -> Result<(), BearDogError> {
    let home = home.as_ref();
    info!("🎵 BirdSong Lineage Decryption");
    println!("=================================");
    println!();
    println!("📋 Configuration:");
    println!("   Input: {input}");
    println!("   Key ID: {key_id}");
    println!();

    // Read encrypted broadcast
    let encrypted_data =
        fs::read(input).map_err(|e| BearDogError::system(format!("Failed to read file: {e}")))?;

    let broadcast: BirdSongBroadcast = serde_json::from_slice(&encrypted_data)
        .map_err(|e| BearDogError::system(format!("Invalid broadcast format: {e}")))?;

    debug!(
        "Loaded broadcast: root={}, ciphertext={} bytes",
        broadcast.hint.root_id,
        broadcast.ciphertext.len()
    );

    // Get lineage proof for this key
    println!("🔍 Looking up lineage proof for {key_id}...");
    let proof = get_lineage_proof_for_key_with_home(key_id, home).await?;

    println!("📜 Lineage proof:");
    println!("   Node ID: {}", proof.node_id);
    println!("   Root ID: {}", proof.root_id);
    println!("   Path depth: {}", proof.path.len() - 1);
    println!();

    // Check if root IDs match
    if proof.root_id != broadcast.hint.root_id {
        println!("❌ Cannot decrypt: not in lineage");
        println!();
        println!("🔐 Privacy enforced:");
        println!("   Your lineage: {}", proof.root_id);
        println!("   Broadcast for: {}", broadcast.hint.root_id);
        println!("   Result: You see only noise (strangers cannot read)");
        println!();
        return Err(BearDogError::security(
            "Lineage mismatch: cannot decrypt broadcast for different lineage".to_string(),
        ));
    }

    // Create decryption request
    let request = BirdSongDecryptRequest {
        broadcast: broadcast.clone(),
        proof: proof.clone(),
    };

    // Load root key to get master secret (must match encryption!)
    println!("🔑 Loading root key {}...", proof.root_id);
    let root_key = key_store::load_key_from_home(&proof.root_id, home)?;
    let root_key_material = key_store::base64_decode(&root_key.key_material_b64)?;

    // Use same root key material as master secret (must match encryption)
    let master_secret = if root_key_material.len() >= 32 {
        root_key_material[..32].to_vec()
    } else {
        // If key is smaller, pad with HKDF expand (same as encrypt)
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&root_key_material);
        hasher.update(b"beardog-birdsong-master-secret-v1");
        hasher.finalize().to_vec()
    };

    // Initialize BirdSong encryption with root-derived master secret
    let kdf = Arc::new(LineageKeyDerivation::new(master_secret)?);
    let encryption = BirdSongEncryption::new(kdf);

    // Attempt decryption
    println!("🔓 Attempting to decrypt...");
    match encryption.decrypt(&request) {
        Ok(plaintext) => {
            let message = String::from_utf8_lossy(&plaintext);
            println!("✅ Decrypted successfully!");
            println!();
            println!("📄 Decrypted Message:");
            println!("{message}");
            println!();
            println!("🔐 Privacy verified:");
            println!("   You are in lineage '{}'", proof.root_id);
            println!("   Your depth: {}", proof.path.len() - 1);
            println!(
                "   Allowed range: {}-{}",
                broadcast.hint.min_depth, broadcast.hint.max_depth
            );
            println!();
            Ok(())
        }
        Err(e) => {
            println!("❌ Cannot decrypt: {e}");
            println!();
            println!("🔐 Privacy enforced:");
            println!("   Either you're not in the allowed depth range,");
            println!("   or the key is expired/invalid.");
            println!("   Strangers see only noise.");
            println!();
            Err(e)
        }
    }
}

/// Parse lineage hint from string format
///
/// Supported formats:
/// - "`DirectAncestors`" -> `min_depth=0`, `max_depth=1`
/// - "`AllDescendants`" -> `min_depth=0`, `max_depth=100`
/// - "Depth:2-5" -> `min_depth=2`, `max_depth=5`
/// - "`RootOnly`" -> `min_depth=0`, `max_depth=0`
fn parse_lineage_hint(hint_type: &str, root_id: &str) -> Result<LineageHint, BearDogError> {
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
///
/// # Arguments
///
/// * `key_id` - Key identifier
///
/// # Returns
///
/// A lineage proof showing the path from root to this node
async fn get_lineage_proof_for_key_with_home(
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
fn compute_merkle_root(leaves: &[Vec<u8>]) -> Vec<u8> {
    use sha2::{Digest, Sha256};

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

#[cfg(test)]
mod tests {
    use super::{
        handle_birdsong_decrypt, handle_birdsong_decrypt_with_home,
        handle_birdsong_encrypt_with_home, parse_lineage_hint,
    };
    use crate::handlers::key_store;
    use tempfile::TempDir;

    fn sample_stored_key(id: &str, material: &[u8]) -> key_store::StoredKey {
        key_store::StoredKey {
            key_id: id.to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "test-hsm".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(material),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: Some(key_store::KeyLineageInfo {
                parent_key_id: None,
                depth: 0,
            }),
            expires_at: None,
            usage: None,
            purpose: None,
        }
    }

    #[test]
    fn test_parse_lineage_hint_direct_ancestors() {
        let hint =
            parse_lineage_hint("DirectAncestors", "root-123").expect("parse DirectAncestors hint");
        assert_eq!(hint.root_id, "root-123");
        assert_eq!(hint.min_depth, 0);
        assert_eq!(hint.max_depth, 1);
    }

    #[test]
    fn test_parse_lineage_hint_all_descendants() {
        let hint =
            parse_lineage_hint("AllDescendants", "root-456").expect("parse AllDescendants hint");
        assert_eq!(hint.root_id, "root-456");
        assert_eq!(hint.min_depth, 0);
        assert_eq!(hint.max_depth, 100);
    }

    #[test]
    fn test_parse_lineage_hint_depth_range() {
        let hint = parse_lineage_hint("Depth:2-5", "root-789").expect("parse Depth:2-5 hint");
        assert_eq!(hint.root_id, "root-789");
        assert_eq!(hint.min_depth, 2);
        assert_eq!(hint.max_depth, 5);
    }

    #[test]
    fn test_parse_lineage_hint_root_only() {
        let hint = parse_lineage_hint("RootOnly", "root-000").expect("parse RootOnly hint");
        assert_eq!(hint.root_id, "root-000");
        assert_eq!(hint.min_depth, 0);
        assert_eq!(hint.max_depth, 0);
    }

    #[test]
    fn test_parse_lineage_hint_invalid() {
        let result = parse_lineage_hint("InvalidHint", "root-123");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lineage_hint_invalid_depth_format() {
        let result = parse_lineage_hint("Depth:invalid", "root-123");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_lineage_hint_depth_too_many_parts() {
        assert!(parse_lineage_hint("Depth:0-1-2", "r").is_err());
    }

    #[test]
    fn test_compute_merkle_root_empty() {
        assert!(super::compute_merkle_root(&[]).is_empty());
    }

    #[test]
    fn test_compute_merkle_root_single_leaf() {
        let leaf = vec![1u8; 32];
        let root = super::compute_merkle_root(std::slice::from_ref(&leaf));
        assert_eq!(root, leaf);
    }

    #[test]
    fn test_compute_merkle_root_two_leaves() {
        let a = vec![1u8; 32];
        let b = vec![2u8; 32];
        let root = super::compute_merkle_root(&[a, b]);
        assert_eq!(root.len(), 32);
    }

    #[test]
    fn test_compute_merkle_root_three_leaves() {
        let leaves: Vec<Vec<u8>> = (0u8..3).map(|i| vec![i; 32]).collect();
        let root = super::compute_merkle_root(&leaves);
        assert_eq!(root.len(), 32);
    }

    #[tokio::test]
    async fn test_birdsong_encrypt_decrypt_roundtrip_full_key_material() {
        let dir = TempDir::new().expect("create temp directory for birdsong roundtrip test");
        let home = dir.path();

        let root = sample_stored_key("root-lineage-1", &[9u8; 32]);
        key_store::save_key_to_home(&root, home).expect("save root-lineage-1");

        let out = dir.path().join("out.birdsong");
        handle_birdsong_encrypt_with_home(
            "hello-roundtrip",
            "DirectAncestors",
            "root-lineage-1",
            Some(out.to_str().expect("out.birdsong path must be valid UTF-8")),
            home,
        )
        .await
        .expect("encrypt");

        handle_birdsong_decrypt_with_home(
            out.to_str().expect("out.birdsong path must be valid UTF-8"),
            "root-lineage-1",
            home,
        )
        .await
        .expect("decrypt");
    }

    #[tokio::test]
    async fn test_birdsong_encrypt_uses_short_key_material_hkdf_branch() {
        let dir =
            TempDir::new().expect("create temp directory for short key material birdsong test");
        let home = dir.path();

        let root = sample_stored_key("short-root", &[1u8; 16]);
        key_store::save_key_to_home(&root, home).expect("save short-root");

        let out = dir.path().join("short.birdsong");
        handle_birdsong_encrypt_with_home(
            "m",
            "RootOnly",
            "short-root",
            Some(
                out.to_str()
                    .expect("short.birdsong path must be valid UTF-8"),
            ),
            home,
        )
        .await
        .expect("encrypt");

        handle_birdsong_decrypt_with_home(
            out.to_str()
                .expect("short.birdsong path must be valid UTF-8"),
            "short-root",
            home,
        )
        .await
        .expect("decrypt");
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_lineage_mismatch() {
        let dir = TempDir::new().expect("create temp directory for birdsong lineage mismatch test");
        let home = dir.path();

        let a = sample_stored_key("root-a", &[2u8; 32]);
        let b = sample_stored_key("root-b", &[3u8; 32]);
        key_store::save_key_to_home(&a, home).expect("save root-a");
        key_store::save_key_to_home(&b, home).expect("save root-b");

        let out = dir.path().join("mismatch.birdsong");
        handle_birdsong_encrypt_with_home(
            "x",
            "AllDescendants",
            "root-a",
            Some(
                out.to_str()
                    .expect("mismatch.birdsong path must be valid UTF-8"),
            ),
            home,
        )
        .await
        .expect("encrypt for lineage mismatch test");

        let err = handle_birdsong_decrypt_with_home(
            out.to_str()
                .expect("mismatch.birdsong path must be valid UTF-8"),
            "root-b",
            home,
        )
        .await
        .expect_err("lineage mismatch");
        assert!(
            err.to_string().to_lowercase().contains("lineage")
                || err.to_string().contains("Lineage")
        );
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_missing_file() {
        assert!(
            handle_birdsong_decrypt("/no/such/file.birdsong", "k")
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_invalid_json() {
        let dir = TempDir::new().expect("create temp directory for invalid birdsong JSON test");
        let p = dir.path().join("bad.json");
        std::fs::write(&p, b"not json").expect("write invalid JSON fixture");
        assert!(
            handle_birdsong_decrypt(p.to_str().expect("bad.json path must be valid UTF-8"), "k",)
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_key_without_lineage() {
        let dir = TempDir::new().expect("create temp directory for decrypt without lineage test");
        let home = dir.path();

        let mut k = sample_stored_key("no-lineage", &[5u8; 32]);
        k.lineage = None;
        key_store::save_key_to_home(&k, home).expect("save no-lineage key");

        let out = dir.path().join("x.birdsong");
        handle_birdsong_encrypt_with_home(
            "z",
            "RootOnly",
            "no-lineage",
            Some(out.to_str().expect("x.birdsong path must be valid UTF-8")),
            home,
        )
        .await
        .expect("encrypt for no-lineage decrypt test");

        let err = handle_birdsong_decrypt_with_home(
            out.to_str().expect("x.birdsong path must be valid UTF-8"),
            "no-lineage",
            home,
        )
        .await
        .expect_err("no lineage on decrypt key");
        assert!(err.to_string().contains("lineage") || err.to_string().contains("Lineage"));
    }

    #[tokio::test]
    async fn test_birdsong_decrypt_with_child_key_lineage_chain() {
        let dir = TempDir::new().expect("temp dir for child lineage decrypt test");
        let home = dir.path();

        let root = sample_stored_key("lineage-root", &[9u8; 32]);
        let mut child = sample_stored_key("lineage-child", &[9u8; 32]);
        child.lineage = Some(key_store::KeyLineageInfo {
            parent_key_id: Some("lineage-root".to_string()),
            depth: 1,
        });
        key_store::save_key_to_home(&root, home).expect("save lineage-root");
        key_store::save_key_to_home(&child, home).expect("save lineage-child");

        let out = dir.path().join("chain.birdsong");
        handle_birdsong_encrypt_with_home(
            "child-lineage-msg",
            "AllDescendants",
            "lineage-root",
            Some(
                out.to_str()
                    .expect("chain.birdsong path must be valid UTF-8"),
            ),
            home,
        )
        .await
        .expect("encrypt for child lineage test");

        handle_birdsong_decrypt_with_home(
            out.to_str()
                .expect("chain.birdsong path must be valid UTF-8"),
            "lineage-child",
            home,
        )
        .await
        .expect("decrypt with child key in lineage");
    }
}
