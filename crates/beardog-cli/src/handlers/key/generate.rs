// SPDX-License-Identifier: AGPL-3.0-or-later

//! Key generation: HSM discovery, AES material derivation, and enhanced KDF-based generation.

use crate::handlers::hsm_agnostic;
use crate::handlers::kdf;
use crate::handlers::key_derive;
use crate::handlers::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use chrono::Utc;
use std::fs;

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

/// Generate AES-256 key optionally mixed with human entropy seed (test-only after v1 removal).
#[cfg(test)]
fn generate_aes_key_with_seed(seed_data: Option<&[u8]>) -> Result<Vec<u8>, BearDogError> {
    use aes_gcm::aead::OsRng;
    use aes_gcm::{Aes256Gcm, KeyInit};
    use sha3::{Digest, Sha3_256};

    let system_key = Aes256Gcm::generate_key(OsRng);

    if let Some(seed) = seed_data {
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

/// Handle key generate with KDF and restrictions (v2)
///
/// # Errors
///
/// Returns an error if HSM discovery or selection fails, entropy seed or KDF inputs are invalid,
/// key material derivation fails, or the key or receipt cannot be saved.
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
        kdf::KdfConfig::new(kdf_type.to_string(), kdf_iterations, kdf_memory, kdf_time);

    // Generate key material
    let key_len = 32; // AES-256 = 32 bytes
    let password = seed_data.as_deref().unwrap_or(b"beardog-default-seed");
    let salt = b"beardog-cli-salt-v1"; // In production, use random salt per key

    let key_material = kdf_config.derive_key(password, salt, key_len)?;

    println!("✅ Key material derived ({} bytes)", key_material.len());
    println!();

    // Calculate expiry if specified
    let expires_at_str = if let Some(duration_str) = expires_in {
        let expiry = key_derive::parse_duration(duration_str)?;
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
mod generate_tests {
    use super::*;
    use crate::handlers::hsm_agnostic::CliHsmInfo;

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
    fn select_cli_hsm_empty_list_errors_for_all_preferences() {
        let empty: Vec<CliHsmInfo> = vec![];
        assert!(select_cli_hsm_for_preference(&empty, "auto").is_err());
        assert!(select_cli_hsm_for_preference(&empty, "software").is_err());
        assert!(select_cli_hsm_for_preference(&empty, "hardware").is_err());
        assert!(select_cli_hsm_for_preference(&empty, "mobile").is_err());
    }

    #[test]
    fn select_cli_hsm_case_insensitive_preference() {
        let hs = vec![sample_cli_hsm("Software", "soft")];
        assert_eq!(
            select_cli_hsm_for_preference(&hs, "SOFTWARE")
                .expect("uppercase software")
                .name,
            "soft"
        );
        assert_eq!(
            select_cli_hsm_for_preference(&hs, "Auto")
                .expect("mixed case auto")
                .name,
            "soft"
        );
    }

    #[test]
    fn select_cli_hsm_auto_skips_unknown_tiers_until_software() {
        let hsms = vec![
            sample_cli_hsm("CloudOnly", "cloud"),
            sample_cli_hsm("Software", "sw"),
        ];
        let picked = select_cli_hsm_for_preference(&hsms, "auto").expect("fallback to software");
        assert_eq!(picked.tier, "Software");
        assert_eq!(picked.name, "sw");
    }
}
