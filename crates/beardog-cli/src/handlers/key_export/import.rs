// SPDX-License-Identifier: AGPL-3.0-or-later

//! CLI handlers for importing keys from JSON.

use crate::handlers::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use std::fs;
use std::path::Path;

use super::crypto::decrypt_key_material;
use super::types::ExportedKey;

/// Handle key import command
///
/// # Errors
///
/// Returns an error if the key home cannot be resolved, the import file cannot be read or parsed,
/// decryption or validation fails, or the key cannot be saved.
pub async fn handle_key_import(
    input_path: &str,
    key_id_override: Option<&str>,
    decrypt: bool,
) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_import_with_home(input_path, key_id_override, decrypt, false, home.as_path()).await
}

/// Same as [`handle_key_import`] but keys are stored under `home/.beardog/keys`.
///
/// When `allow_overwrite` is true, an existing key with the same id is replaced without stdin confirmation (tests / automation).
///
/// # Errors
///
/// Returns an error if the import file cannot be read or parsed, password prompts fail, decryption
/// fails, overwrite is refused, or the key cannot be saved.
pub async fn handle_key_import_with_home(
    input_path: &str,
    key_id_override: Option<&str>,
    decrypt: bool,
    allow_overwrite: bool,
    home: &Path,
) -> Result<(), BearDogError> {
    use std::io::{self, Write};

    println!("📥 BearDog Key Import");
    println!("====================");
    println!();

    // Read the exported key file
    println!("🔍 Reading key file: {input_path}");
    let json = fs::read_to_string(input_path)?;

    let mut exported: ExportedKey = serde_json::from_str(&json)
        .map_err(|e| BearDogError::serialization(&format!("Invalid key file format: {e}")))?;

    println!("✅ Key file loaded");
    println!("   Key ID: {}", exported.key_id);
    println!("   Algorithm: {}", exported.algorithm);
    println!("   Generation: {}", exported.generation);
    println!(
        "   Encrypted: {}",
        if exported.encrypted { "Yes" } else { "No" }
    );
    println!("   Version: {}", exported.version);
    println!();

    // Check version compatibility
    if exported.version != "1.0" {
        println!(
            "⚠️  WARNING: Key export version {} may not be fully compatible",
            exported.version
        );
        println!("   Current version: 1.0");
        println!();
    }

    // Decrypt if needed
    if exported.encrypted {
        if !decrypt {
            return Err(BearDogError::validation(
                "Key is encrypted but --decrypt flag not provided. Use --decrypt and provide password.",
            ));
        }

        println!("🔐 Decrypting key material...");
        let password = rpassword::prompt_password("Enter decryption password: ")
            .map_err(|e| BearDogError::system(format!("Failed to read password: {e}")))?;

        if password.is_empty() {
            return Err(BearDogError::validation("Password cannot be empty"));
        }

        // Decrypt the key material
        let decrypted_material = decrypt_key_material(&exported.key_material, &password)?;
        exported.key_material = decrypted_material;
        exported.encrypted = false;

        println!("✅ Key material decrypted");
        println!();
    }

    // Use override key_id if provided
    let final_key_id = key_id_override.unwrap_or(&exported.key_id);

    // Check if key already exists
    if key_store::load_key_from_home(final_key_id, home).is_ok() {
        if allow_overwrite {
            // Tests / automation: proceed without prompting
        } else {
            println!("⚠️  WARNING: Key '{final_key_id}' already exists!");
            println!("   Import will overwrite the existing key.");
            println!();

            print!("Continue? [y/N]: ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if !input.trim().eq_ignore_ascii_case("y") {
                println!("❌ Import cancelled");
                return Ok(());
            }
            println!();
        }
    }

    // Convert to StoredKey format (consume `exported` to avoid cloning each field)
    let ExportedKey {
        algorithm,
        parent,
        generation,
        created_at,
        context,
        expires_at,
        usage,
        purpose,
        key_material,
        metadata,
        ..
    } = exported;

    let hsm_name = metadata
        .get("hsm_name")
        .cloned()
        .unwrap_or_else(|| "imported".to_string());

    let stored_key = StoredKey {
        key_id: final_key_id.to_string(),
        algorithm,
        hsm_name,
        created_at,
        key_material_b64: key_material,
        generation,
        parent_key_id: parent,
        derivation_purpose: context,
        children: Vec::new(), // Will be rebuilt as keys are derived
        lineage: None,        // Imported keys don't have lineage info initially
        expires_at,
        usage,
        purpose,
    };

    // Save to key store
    key_store::save_key_to_home(&stored_key, home)?;

    println!("✅ Key imported successfully!");
    println!();
    println!("📋 Import Details:");
    println!("   Key ID: {final_key_id}");
    println!("   Algorithm: {}", stored_key.algorithm);
    println!("   Generation: {}", stored_key.generation);
    if let Some(parent) = &stored_key.parent_key_id {
        println!("   Parent: {parent}");
    }
    if let Some(expires) = &stored_key.expires_at {
        println!("   Expires: {expires}");
    }
    println!();

    println!("💡 Next steps:");
    println!("   1. Verify key: beardog key info --key-id {final_key_id}");
    println!(
        "   2. Use key: beardog encrypt --key {final_key_id} --input data.txt --output data.enc"
    );
    if stored_key.parent_key_id.is_some() {
        println!("   3. Check lineage: beardog key lineage --key-id {final_key_id}");
    }
    println!();

    Ok(())
}
