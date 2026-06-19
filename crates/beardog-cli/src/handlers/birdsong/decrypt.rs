// SPDX-License-Identifier: AGPL-3.0-or-later

use beardog_errors::BearDogError;
use beardog_genetics::birdsong::{
    encryption::BirdSongEncryption,
    key_derivation::LineageKeyDerivation,
    types::{BirdSongBroadcast, BirdSongDecryptRequest},
};
use std::fs;
use std::path::Path;
use std::sync::Arc;
use tracing::{debug, info};

use crate::handlers::key_store;

use super::lineage::get_lineage_proof_for_key_with_home;

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
