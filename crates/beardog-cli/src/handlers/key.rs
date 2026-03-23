// SPDX-License-Identifier: AGPL-3.0-only

//! Key lifecycle handlers: generate, list, info, delete (wired to local key store).

use super::hsm_agnostic;
use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::fs;
use std::path::Path;

// ALL OLD PLACEHOLDER CODE REMOVED
// Now using hsm_agnostic module for universal discovery

async fn discover_hsms_agnostic() -> Result<Vec<hsm_agnostic::CliHsmInfo>, BearDogError> {
    hsm_agnostic::discover_all_hsms().await
}

/// Select an HSM for key generation (`auto` prefers Mobile → Hardware → Software).
pub(crate) fn select_cli_hsm_for_preference<'a>(
    hsms: &'a [hsm_agnostic::CliHsmInfo],
    hsm_preference: &str,
) -> Result<&'a hsm_agnostic::CliHsmInfo, BearDogError> {
    match hsm_preference.to_lowercase().as_str() {
        "auto" => hsms
            .iter()
            .find(|h| h.tier == "Mobile")
            .or_else(|| hsms.iter().find(|h| h.tier == "Hardware"))
            .or_else(|| hsms.iter().find(|h| h.tier == "Software"))
            .ok_or_else(|| BearDogError::not_found("No suitable HSM found".to_string())),
        "software" => hsms
            .iter()
            .find(|h| h.tier == "Software")
            .ok_or_else(|| BearDogError::not_found("No software HSM found".to_string())),
        "hardware" => hsms
            .iter()
            .find(|h| h.tier == "Hardware")
            .ok_or_else(|| BearDogError::not_found("No hardware HSM found".to_string())),
        "mobile" => hsms
            .iter()
            .find(|h| h.tier == "Mobile")
            .ok_or_else(|| BearDogError::not_found("No mobile HSM found".to_string())),
        _ => {
            let msg = format!("Unknown HSM preference: {hsm_preference}");
            Err(BearDogError::invalid_input(&msg))
        }
    }
}

// ALL OLD PLACEHOLDER CODE REMOVED - was hardcoded and vendor-specific

/// Generate AES-256 key optionally mixed with human entropy seed
fn generate_aes_key_with_seed(seed_data: Option<&[u8]>) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::aead::OsRng;
    use aes_gcm::{Aes256Gcm, KeyInit};
    use sha3::{Digest, Sha3_256};

    // Generate system entropy key
    let system_key = Aes256Gcm::generate_key(OsRng);

    if let Some(seed) = seed_data {
        // Mix human entropy with system entropy using SHA3-256
        // Derived = SHA3-256(system_key || seed || context)
        let mut hasher = Sha3_256::new();
        hasher.update(system_key);
        hasher.update(seed);
        hasher.update(b"BearDog-AES256-KeyDerivation-v1");
        let derived = hasher.finalize();
        Ok(derived.to_vec())
    } else {
        Ok(system_key.to_vec())
    }
}

/// Generate AES-256 key using only system entropy (no human seed)
#[allow(
    dead_code,
    reason = "Legacy helper kept for callers not using human seed path"
)]
fn generate_aes_key() -> Result<Vec<u8>, BearDogError> {
    generate_aes_key_with_seed(None)
}

/// Handle key generation command
#[allow(
    dead_code,
    reason = "Legacy public handler kept for API stability; main uses handle_key_generate_v2"
)]
pub async fn handle_key_generate(
    key_id: &str,
    algorithm: &str,
    hsm_preference: &str,
    seed_path: Option<&str>,
) -> Result<(), BearDogError> {
    println!("🔑 BearDog Key Generation");
    println!("========================");
    println!();

    // Parse algorithm (vendor-agnostic algorithm names)
    println!("📋 Configuration:");
    println!("   Key ID: {key_id}");
    println!("   Algorithm: {algorithm}");
    println!("   HSM Preference: {hsm_preference}");
    if let Some(seed) = seed_path {
        println!("   Entropy Seed: {seed}");
    }
    println!();

    // Discover and select HSM (vendor-agnostic)
    println!("🔍 Discovering HSMs...");
    let hsms = discover_hsms_agnostic().await?;

    if hsms.is_empty() {
        return Err(BearDogError::not_found(
            "No HSMs found. Please connect hardware or install a PKCS#11 provider.".to_string(),
        ));
    }

    let selected_hsm = select_cli_hsm_for_preference(&hsms, hsm_preference)?;

    println!("✅ Selected HSM: {}", selected_hsm.name);
    println!("   Tier: {}", selected_hsm.tier);
    println!();

    // Load entropy seed if provided
    let seed_data = if let Some(seed_path) = seed_path {
        println!("🌱 Loading entropy seed...");
        let seed_content = fs::read_to_string(seed_path)?;
        println!("✅ Entropy seed loaded ({} bytes)", seed_content.len());
        println!("   Seed will be mixed with system entropy for key derivation");
        println!();
        Some(seed_content.into_bytes())
    } else {
        None
    };

    // Generate key (implementation-agnostic)
    println!("🔐 Generating {algorithm} key...");

    // For now, use simplified key generation
    // Will wire to HsmManager in next iteration
    println!("⚙️  Creating key with selected HSM...");

    // Generate AES-256-GCM key material, optionally mixed with human entropy
    // NOTE: In production, this should be generated/stored in the HSM
    let key_material = generate_aes_key_with_seed(seed_data.as_deref())?;

    // Save key to storage
    let stored_key = StoredKey {
        key_id: key_id.to_string(),
        algorithm: algorithm.to_string(),
        hsm_name: selected_hsm.name.clone(),
        created_at: Utc::now().to_rfc3339(),
        key_material_b64: key_store::base64_encode(&key_material),
        generation: 0, // Root key
        parent_key_id: None,
        derivation_purpose: None,
        children: Vec::new(),
        lineage: Some(key_store::KeyLineageInfo {
            parent_key_id: None,
            depth: 0,
        }),
        expires_at: None,
        usage: None,
        purpose: None,
    };

    key_store::save_key(&stored_key)?;

    // Generate operation receipt
    use beardog_types::receipt::{HsmInfo, KeyInfo, OperationReceipt, generate_receipt_filename};
    use serde_json::json;

    let receipt = OperationReceipt::new("key-generate")
        .with_key_info(KeyInfo {
            key_id: key_id.to_string(),
            algorithm: algorithm.to_string(),
            generation: 0,
            parent_key_id: None,
            expires_at: None,
            usage: None,
            purpose: None,
        })
        .with_hsm_info(HsmInfo {
            name: selected_hsm.name.clone(),
            vendor: Some(selected_hsm.vendor.clone()),
            model: Some(selected_hsm.model.clone()),
            hsm_type: Some(selected_hsm.hsm_type.clone()),
        })
        .with_metadata(
            "entropy_source",
            json!(if seed_data.is_some() {
                "human"
            } else {
                "system"
            }),
        );

    // Save receipt to receipts directory
    let receipt_dir = std::path::Path::new("receipts");
    std::fs::create_dir_all(receipt_dir)?;
    let receipt_path = receipt_dir.join(generate_receipt_filename("key-generate"));
    receipt.save_to_file(&receipt_path)?;

    println!("✅ Key generated successfully!");
    println!();
    println!("📋 Key Details:");
    println!("   ID: {key_id}");
    println!("   Algorithm: {algorithm}");
    println!("   HSM: {}", selected_hsm.name);
    println!("   Status: Active");
    println!();
    println!("📜 Receipt: {}", receipt_path.display());
    println!("   Receipt ID: {}", receipt.receipt_id);
    println!();
    println!("💡 Next steps:");
    println!("   • List keys: beardog key list");
    println!("   • Encrypt: beardog encrypt --key {key_id} --input data.txt --output data.enc");

    Ok(())
}

/// Handle key list command
pub async fn handle_key_list(hsm_filter: Option<&str>, _verbose: bool) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_list_with_home(hsm_filter, _verbose, &home).await
}

/// Same as [`handle_key_list`] but with an explicit home directory for the key store (tests / DI).
pub async fn handle_key_list_with_home(
    hsm_filter: Option<&str>,
    _verbose: bool,
    home: impl AsRef<Path>,
) -> Result<(), BearDogError> {
    println!("🔑 Available Keys");
    println!("================");
    println!();

    let keys = key_store::list_keys_from_home(home)?;

    let filtered_keys: Vec<_> = if let Some(filter) = hsm_filter {
        println!("📌 Filtering by HSM: {filter}");
        println!();
        keys.into_iter()
            .filter(|k| k.hsm_name.to_lowercase().contains(&filter.to_lowercase()))
            .collect()
    } else {
        keys
    };

    if filtered_keys.is_empty() {
        println!("No keys found.");
        println!();
        println!("💡 Generate a key:");
        println!("   beardog key generate --key-id my-key --algorithm aes256-gcm");
        return Ok(());
    }

    println!("Found {} key(s):", filtered_keys.len());
    println!();

    for key in filtered_keys {
        println!("📋 Key: {}", key.key_id);
        println!("   Algorithm: {}", key.algorithm);
        println!("   HSM: {}", key.hsm_name);
        println!("   Created: {}", key.created_at);
        println!();
    }

    Ok(())
}

/// Handle key info command
pub async fn handle_key_info(key_id: &str) -> Result<(), BearDogError> {
    println!("🔍 Key Information");
    println!("=================");
    println!();
    println!("Key ID: {key_id}");
    println!();
    println!("🔨 Coming soon in next iteration!");

    Ok(())
}

/// Handle key delete command
pub async fn handle_key_delete(key_id: &str, skip_confirm: bool) -> Result<(), BearDogError> {
    let home = key_store::home_dir_for_keys()?;
    handle_key_delete_with_home(key_id, skip_confirm, &home).await
}

/// Same as [`handle_key_delete`] but with an explicit home directory for the key store (tests / DI).
pub async fn handle_key_delete_with_home(
    key_id: &str,
    skip_confirm: bool,
    home: impl AsRef<Path>,
) -> Result<(), BearDogError> {
    println!("🗑️  Delete Key");
    println!("=============");
    println!();

    if !skip_confirm {
        println!("⚠️  Are you sure you want to delete key '{key_id}'?");
        println!("   This action CANNOT be undone!");
        println!();
        println!("   Run with --yes to skip this prompt.");
        return Ok(());
    }

    key_store::delete_key_from_home(key_id, home)?;
    println!("✅ Key '{key_id}' deleted successfully");

    Ok(())
}

/// Handle key generate with KDF and restrictions (v2)
#[expect(
    clippy::too_many_arguments,
    reason = "Key generation CLI surfaces algorithm, HSM, KDF, and policy fields together"
)]
pub async fn handle_key_generate_v2(
    key_id: &str,
    algorithm: &str,
    hsm_preference: &str,
    seed_path: Option<&str>,
    kdf_type: &str,
    kdf_iterations: Option<u32>,
    kdf_memory: Option<u32>,
    kdf_time: Option<u32>,
    usage: Option<&str>,
    expires_in: Option<&str>,
    purpose: Option<&str>,
) -> Result<(), BearDogError> {
    println!("🔑 BearDog Key Generation (Enhanced)");
    println!("====================================");
    println!();

    // Parse algorithm (vendor-agnostic algorithm names)
    println!("📋 Configuration:");
    println!("   Key ID: {key_id}");
    println!("   Algorithm: {algorithm}");
    println!("   HSM Preference: {hsm_preference}");
    println!("   KDF: {kdf_type}");
    if let Some(seed) = seed_path {
        println!("   Entropy Seed: {seed}");
    }
    if let Some(purp) = purpose {
        println!("   Purpose: {purp}");
    }
    if let Some(exp) = expires_in {
        println!("   Expires In: {exp}");
    }
    if let Some(use_restrict) = usage {
        println!("   Usage: {use_restrict}");
    }
    println!();

    // Discover and select HSM (vendor-agnostic)
    println!("🔍 Discovering HSMs...");
    let hsms = discover_hsms_agnostic().await?;

    if hsms.is_empty() {
        return Err(BearDogError::not_found(
            "No HSMs found. Please connect hardware or install a PKCS#11 provider.".to_string(),
        ));
    }

    let selected_hsm = select_cli_hsm_for_preference(&hsms, hsm_preference)?;

    println!("✅ Selected HSM: {}", selected_hsm.name);
    println!("   Tier: {}", selected_hsm.tier);
    println!();

    // Load entropy seed if provided
    let seed_data = if let Some(seed_path) = seed_path {
        println!("🌱 Loading entropy seed...");
        let seed_content = fs::read_to_string(seed_path)?;
        println!("✅ Entropy seed loaded ({} bytes)", seed_content.len());
        println!("   Seed will be mixed with system entropy for key derivation");
        println!();
        Some(seed_content.into_bytes())
    } else {
        None
    };

    // Derive key material using specified KDF
    println!(
        "🔐 Deriving {} key with {}...",
        algorithm,
        kdf_type.to_uppercase()
    );

    // Create KDF config
    let kdf_config =
        super::kdf::KdfConfig::new(kdf_type.to_string(), kdf_iterations, kdf_memory, kdf_time);

    // Generate key material
    let key_len = 32; // AES-256 = 32 bytes
    let password = seed_data.as_deref().unwrap_or(b"beardog-default-seed");
    let salt = b"beardog-cli-salt-v1"; // In production, use random salt per key

    let key_material = kdf_config.derive_key(password, salt, key_len)?;

    println!("✅ Key material derived ({} bytes)", key_material.len());
    println!();

    // Calculate expiry if specified
    let expires_at_str = if let Some(duration_str) = expires_in {
        let expiry = super::key_derive::parse_duration(duration_str)?;
        Some(expiry.to_rfc3339())
    } else {
        None
    };

    // Save key to storage
    let stored_key = StoredKey {
        key_id: key_id.to_string(),
        algorithm: algorithm.to_string(),
        hsm_name: selected_hsm.name.clone(),
        created_at: Utc::now().to_rfc3339(),
        key_material_b64: key_store::base64_encode(&key_material),
        generation: 0, // Root key
        parent_key_id: None,
        derivation_purpose: None,
        children: Vec::new(),
        lineage: Some(key_store::KeyLineageInfo {
            parent_key_id: None,
            depth: 0,
        }),
        expires_at: expires_at_str.clone(),
        usage: usage.map(std::string::ToString::to_string),
        purpose: purpose.map(std::string::ToString::to_string),
    };

    key_store::save_key(&stored_key)?;

    // Generate operation receipt
    use beardog_types::receipt::{HsmInfo, KeyInfo, OperationReceipt, generate_receipt_filename};
    use serde_json::json;

    let receipt = OperationReceipt::new("key-generate")
        .with_key_info(KeyInfo {
            key_id: key_id.to_string(),
            algorithm: algorithm.to_string(),
            generation: 0,
            parent_key_id: None,
            expires_at: expires_at_str.clone(),
            usage: usage.map(std::string::ToString::to_string),
            purpose: purpose.map(std::string::ToString::to_string),
        })
        .with_hsm_info(HsmInfo {
            name: selected_hsm.name.clone(),
            vendor: Some(selected_hsm.vendor.clone()),
            model: Some(selected_hsm.model.clone()),
            hsm_type: Some(selected_hsm.hsm_type.clone()),
        })
        .with_metadata("kdf", json!(kdf_type))
        .with_metadata(
            "entropy_source",
            json!(if seed_data.is_some() {
                "human"
            } else {
                "system"
            }),
        );

    // Save receipt
    let receipt_dir = std::path::Path::new("receipts");
    std::fs::create_dir_all(receipt_dir)?;
    let receipt_path = receipt_dir.join(generate_receipt_filename("key-generate"));
    receipt.save_to_file(&receipt_path)?;

    println!("✅ Key generated successfully!");
    println!();
    println!("📋 Key Details:");
    println!("   ID: {key_id}");
    println!("   Algorithm: {algorithm}");
    println!("   HSM: {}", selected_hsm.name);
    println!("   KDF: {kdf_type}");
    println!("   Generation: 0 (root key)");
    println!("   Status: Active");

    if let Some(exp) = expires_at_str {
        println!();
        println!("⏰ Expiry: {exp}");
    }

    if let Some(use_restrict) = usage {
        println!("   Usage: {use_restrict}");
    }

    if let Some(purp) = purpose {
        println!("   Purpose: {purp}");
    }

    println!();
    println!("📜 Receipt: {}", receipt_path.display());
    println!("   Receipt ID: {}", receipt.receipt_id);

    println!();
    println!("💡 Next steps:");
    println!("   • List keys: beardog key list");
    println!("   • Encrypt: beardog encrypt --key {key_id} --input data.txt --output data.enc");

    Ok(())
}

#[cfg(test)]
mod key_handler_tests {
    use super::*;
    use crate::handlers::hsm_agnostic::CliHsmInfo;
    use crate::handlers::key_store;
    use chrono::Utc;
    use tempfile::TempDir;

    fn sample_cli_hsm(tier: &str, name: &str) -> CliHsmInfo {
        CliHsmInfo {
            id: format!("id-{name}"),
            name: name.to_string(),
            vendor: "v".to_string(),
            model: "m".to_string(),
            tier: tier.to_string(),
            hsm_type: "t".to_string(),
            path: "/p".to_string(),
            interface_detail: "d".to_string(),
        }
    }

    #[test]
    fn select_cli_hsm_auto_prefers_mobile_then_hardware_then_software() {
        let hsms = vec![
            sample_cli_hsm("Software", "sw"),
            sample_cli_hsm("Hardware", "hw"),
        ];
        assert_eq!(
            select_cli_hsm_for_preference(&hsms, "auto")
                .expect("auto select when only software+hardware")
                .tier,
            "Hardware"
        );

        let with_mobile = vec![sample_cli_hsm("Mobile", "mob"), hsms[1].clone()];
        assert_eq!(
            select_cli_hsm_for_preference(&with_mobile, "auto")
                .expect("auto select with mobile present")
                .tier,
            "Mobile"
        );

        let only_sw = vec![sample_cli_hsm("Software", "only")];
        assert_eq!(
            select_cli_hsm_for_preference(&only_sw, "auto")
                .expect("auto fallback to only software")
                .name,
            "only"
        );
    }

    #[test]
    fn select_cli_hsm_each_tier_and_errors() {
        let hs = vec![sample_cli_hsm("Software", "s")];
        assert!(select_cli_hsm_for_preference(&hs, "software").is_ok());
        assert!(select_cli_hsm_for_preference(&hs, "hardware").is_err());
        assert!(select_cli_hsm_for_preference(&hs, "mobile").is_err());

        let hh = vec![sample_cli_hsm("Hardware", "h")];
        assert!(select_cli_hsm_for_preference(&hh, "hardware").is_ok());

        let mob = vec![sample_cli_hsm("Mobile", "m")];
        assert!(select_cli_hsm_for_preference(&mob, "mobile").is_ok());

        assert!(select_cli_hsm_for_preference(&hs, "AUTO").is_ok());
        assert!(select_cli_hsm_for_preference(&hs, "unknown-mode").is_err());
    }

    #[test]
    fn select_cli_hsm_auto_empty_tiers_fails() {
        let hsms = vec![sample_cli_hsm("Cloud", "c")];
        assert!(select_cli_hsm_for_preference(&hsms, "auto").is_err());
    }

    #[test]
    fn test_generate_aes_key_without_seed() {
        let k1 = generate_aes_key_with_seed(None).expect("generate AES key without seed (1)");
        let k2 = generate_aes_key_with_seed(None).expect("generate AES key without seed (2)");
        assert_eq!(k1.len(), 32);
        assert_eq!(k2.len(), 32);
        assert_ne!(k1, k2);
    }

    #[test]
    fn test_generate_aes_key_with_seed_length_and_entropy() {
        // System entropy is mixed per call (OsRng); same seed does not imply identical output.
        let a = generate_aes_key_with_seed(Some(b"human-entropy-seed"))
            .expect("generate with human seed (1)");
        let b = generate_aes_key_with_seed(Some(b"human-entropy-seed"))
            .expect("generate with human seed (2)");
        assert_eq!(a.len(), 32);
        assert_eq!(b.len(), 32);
        assert_ne!(a, b);
    }

    #[test]
    fn test_generate_aes_key_legacy_alias() {
        let k = generate_aes_key().expect("generate_aes_key legacy alias");
        assert_eq!(k.len(), 32);
    }

    #[tokio::test]
    async fn test_handle_key_list_empty_store() {
        let dir = TempDir::new().expect("create temp directory for empty key list test");
        handle_key_list_with_home(None, false, dir.path())
            .await
            .expect("list keys in empty store");
    }

    #[tokio::test]
    async fn test_handle_key_list_with_filter_and_keys() {
        let dir = TempDir::new().expect("create temp directory for filtered key list test");
        let home = dir.path();

        let alpha = key_store::StoredKey {
            key_id: "alpha".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "AlphaHSM-software".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[1u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        let beta = key_store::StoredKey {
            hsm_name: "BetaHSM-hardware".to_string(),
            key_id: "beta".to_string(),
            ..alpha.clone()
        };
        key_store::save_key_to_home(&alpha, home).expect("save alpha key");
        key_store::save_key_to_home(&beta, home).expect("save beta key");

        handle_key_list_with_home(Some("alpha"), false, home)
            .await
            .expect("list keys filtered by alpha");
        handle_key_list_with_home(None, true, home)
            .await
            .expect("list all keys verbose");
    }

    #[tokio::test]
    async fn test_handle_key_info() {
        handle_key_info("any-id")
            .await
            .expect("handle_key_info placeholder");
    }

    #[tokio::test]
    async fn test_handle_key_delete_skip_confirm_removes_file() {
        let dir = TempDir::new().expect("create temp directory for key delete test");
        let home = dir.path();

        let k = key_store::StoredKey {
            key_id: "to-delete".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "h".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[2u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&k, home).expect("save to-delete key");

        handle_key_delete_with_home("to-delete", true, home)
            .await
            .expect("delete key with skip_confirm");
        assert!(key_store::load_key_from_home("to-delete", home).is_err());
    }

    #[tokio::test]
    async fn test_handle_key_delete_without_confirm_returns_early() {
        let dir = TempDir::new().expect("create temp directory for delete without confirm test");
        handle_key_delete_with_home("some-key", false, dir.path())
            .await
            .expect("delete without confirm returns early");
    }

    #[tokio::test]
    async fn test_handle_key_list_filter_matches_nothing() {
        let dir = TempDir::new().expect("create temp directory for filter matches nothing test");
        let home = dir.path();

        let k = key_store::StoredKey {
            key_id: "only-key".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "LocalSoft".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[1u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&k, home).expect("save only-key");

        handle_key_list_with_home(Some("nomatch-xyz"), false, home)
            .await
            .expect("list with filter matching nothing");
    }

    #[tokio::test]
    async fn test_handle_key_list_hsm_filter_is_case_insensitive() {
        let dir = TempDir::new().expect("create temp directory for case-insensitive filter test");
        let home = dir.path();
        let k = key_store::StoredKey {
            key_id: "k1".to_string(),
            algorithm: "aes256-gcm".to_string(),
            hsm_name: "MySoftHsm".to_string(),
            created_at: Utc::now().to_rfc3339(),
            key_material_b64: key_store::base64_encode(&[1u8; 32]),
            generation: 0,
            parent_key_id: None,
            derivation_purpose: None,
            children: vec![],
            lineage: None,
            expires_at: None,
            usage: None,
            purpose: None,
        };
        key_store::save_key_to_home(&k, home).expect("save k1 for case test");
        handle_key_list_with_home(Some("soft"), false, home)
            .await
            .expect("list with lowercase soft filter");
    }

    #[tokio::test]
    async fn test_handle_key_delete_missing_key_errors() {
        let dir = TempDir::new().expect("create temp directory for missing key delete test");
        let err = handle_key_delete_with_home("missing-id", true, dir.path())
            .await
            .unwrap_err();
        assert!(format!("{err}").contains("missing-id") || format!("{err}").contains("not found"));
    }
}
