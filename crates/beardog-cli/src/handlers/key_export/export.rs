// SPDX-License-Identifier: AGPL-3.0-or-later

//! CLI handlers for exporting keys to JSON.

use crate::handlers::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use std::fs;
use std::path::Path;

use super::crypto::encrypt_key_material;
use super::types::ExportedKey;

/// Handle key export command
///
/// # Errors
///
/// Returns an error if the key home cannot be resolved, the key cannot be loaded, password prompts
/// fail, optional encryption fails, serialization fails, or the output file cannot be written.
pub async fn handle_key_export(
    key_id: &str,
    output_path: &str,
    encrypt: bool,
) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_export_with_home(key_id, output_path, encrypt, home.as_path()).await
}

/// Same as [`handle_key_export`] but keys are loaded from `home/.beardog/keys` (tests / DI).
///
/// # Errors
///
/// Returns an error if the key cannot be loaded, password prompts fail, encryption fails,
/// serialization fails, or the output file cannot be written.
pub async fn handle_key_export_with_home(
    key_id: &str,
    output_path: &str,
    encrypt: bool,
    home: &Path,
) -> Result<(), BearDogError> {
    println!("📤 BearDog Key Export");
    println!("====================");
    println!();

    // Load the key from storage
    println!("🔍 Loading key: {key_id}");
    let stored_key = key_store::load_key_from_home(key_id, home)?;

    println!("✅ Key found");
    println!("   Algorithm: {}", stored_key.algorithm);
    println!("   Generation: {}", stored_key.generation);
    if let Some(parent) = &stored_key.parent_key_id {
        println!("   Parent: {parent}");
    }
    println!();

    // Convert to export format (move fields out of `StoredKey` — no redundant string clones)
    let StoredKey {
        key_id,
        algorithm,
        parent_key_id,
        generation,
        created_at,
        derivation_purpose,
        expires_at,
        usage,
        purpose,
        key_material_b64,
        hsm_name,
        ..
    } = stored_key;

    let mut exported = ExportedKey {
        key_id,
        algorithm,
        parent: parent_key_id,
        generation,
        created_at,
        context: derivation_purpose,
        expires_at,
        usage,
        purpose,
        metadata: std::collections::HashMap::new(),
        key_material: key_material_b64,
        encrypted: false,
        version: "1.0".to_string(),
    };

    // Add metadata
    exported.metadata.insert("hsm_name".to_string(), hsm_name);
    exported
        .metadata
        .insert("exported_at".to_string(), chrono::Utc::now().to_rfc3339());
    exported
        .metadata
        .insert("exported_by".to_string(), "beardog-cli".to_string());

    // Optionally encrypt the key material
    if encrypt {
        println!("🔐 Encrypting key material...");
        println!("⚠️  You will need to provide a password");
        println!();

        // Get password from user
        let password = rpassword::prompt_password("Enter encryption password: ")
            .map_err(|e| BearDogError::system(format!("Failed to read password: {e}")))?;

        if password.is_empty() {
            return Err(BearDogError::validation("Password cannot be empty"));
        }

        // Confirm password
        let password_confirm = rpassword::prompt_password("Confirm password: ")
            .map_err(|e| BearDogError::system(format!("Failed to read password: {e}")))?;

        if password != password_confirm {
            return Err(BearDogError::validation("Passwords do not match"));
        }

        // Encrypt the key material using Argon2 + ChaCha20-Poly1305
        let encrypted_material = encrypt_key_material(&exported.key_material, &password)?;
        exported.key_material = encrypted_material;
        exported.encrypted = true;

        println!("✅ Key material encrypted");
    } else {
        println!("⚠️  WARNING: Key material will be exported in plaintext!");
        println!("   Consider using --encrypt flag for secure transmission");
    }
    println!();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&exported)
        .map_err(|e| BearDogError::serialization(&e.to_string()))?;

    // Write to file
    fs::write(output_path, json)?;

    println!("✅ Key exported successfully!");
    println!();
    println!("📋 Export Details:");
    println!("   File: {output_path}");
    println!("   Key ID: {}", exported.key_id);
    println!("   Algorithm: {}", exported.algorithm);
    println!(
        "   Encrypted: {}",
        if exported.encrypted {
            "Yes"
        } else {
            "No ⚠️"
        }
    );
    println!("   Version: {}", exported.version);
    println!();

    if !exported.encrypted {
        println!("⚠️  SECURITY WARNING:");
        println!("   This key is exported in plaintext!");
        println!("   Protect this file and delete it after transmission.");
        println!("   Use --encrypt flag for secure inter-primal sharing.");
        println!();
    }

    println!("💡 Next steps:");
    println!("   1. Securely transmit {output_path} to the destination tower");
    println!("   2. Import on the other tower: beardog key import --input {output_path}");
    if exported.encrypted {
        println!("   3. Provide the same password during import");
    }
    println!();

    Ok(())
}
