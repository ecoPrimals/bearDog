// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{
    encryption::BirdSongEncryption, key_derivation::LineageKeyDerivation,
    types::BirdSongEncryptRequest,
};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info};

use crate::handlers::key_store;

use super::lineage::parse_lineage_hint;

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
