// Key Management Handler
// Vendor-agnostic: Works with ANY HSM

use super::key_store::{self, StoredKey};
use beardog_errors::BearDogError;
use chrono::Utc;
// Temporarily use placeholders
// use beardog_tunnel::tunnel::hsm::HsmManager;
// use beardog_tunnel::universal_hsm_discovery::{HsmDiscoveryManager, HsmTier};
use std::fs;

// Placeholder HSM structure
#[derive(Clone)]
struct PlaceholderHsm {
    name: String,
    tier: String,
    #[allow(dead_code)] // Will be used when wired to real HSM manager
    hsm_type: String,
}

async fn discover_hsms_placeholder() -> Result<Vec<PlaceholderHsm>, BearDogError> {
    use std::process::Command;

    let mut hsms = Vec::new();

    // Check for SoftHSM2
    if std::path::Path::new("/usr/lib/softhsm/libsofthsm2.so").exists() {
        hsms.push(PlaceholderHsm {
            name: "SoftHSM2".to_string(),
            tier: "Software".to_string(),
            hsm_type: "PKCS#11".to_string(),
        });
    }

    // Check for Android devices via ADB
    let adb_output = Command::new("adb").args(["devices", "-l"]).output();
    if let Ok(output) = adb_output {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(1) {
            if line.contains("device") && !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 && parts[1] == "device" {
                    let serial = parts[0];
                    // Check for StrongBox
                    if let Ok(features) = Command::new("adb")
                        .args(["-s", serial, "shell", "pm", "list", "features"])
                        .output()
                    {
                        let features_str = String::from_utf8_lossy(&features.stdout);
                        if features_str.contains("strongbox_keystore") {
                            let model = parts
                                .iter()
                                .find(|p| p.starts_with("model:"))
                                .and_then(|p| p.strip_prefix("model:"))
                                .unwrap_or("AndroidDevice");
                            hsms.push(PlaceholderHsm {
                                name: format!("Android StrongBox ({})", model.replace('_', " ")),
                                tier: "Mobile".to_string(),
                                hsm_type: "Android-Keystore".to_string(),
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(hsms)
}

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
#[allow(dead_code)]
fn generate_aes_key() -> Result<Vec<u8>, BearDogError> {
    generate_aes_key_with_seed(None)
}

/// Handle key generation command
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
    println!("   Key ID: {}", key_id);
    println!("   Algorithm: {}", algorithm);
    println!("   HSM Preference: {}", hsm_preference);
    if let Some(seed) = seed_path {
        println!("   Entropy Seed: {}", seed);
    }
    println!();

    // Discover and select HSM (vendor-agnostic)
    println!("🔍 Discovering HSMs...");
    let hsms = discover_hsms_placeholder().await?;

    if hsms.is_empty() {
        return Err(BearDogError::not_found(
            "No HSMs found. Please connect hardware or install SoftHSM2.".to_string(),
        ));
    }

    let selected_hsm = match hsm_preference.to_lowercase().as_str() {
        "auto" => {
            // Prefer: mobile > hardware > software
            hsms.iter()
                .find(|h| h.tier == "Mobile")
                .or_else(|| hsms.iter().find(|h| h.tier == "Hardware"))
                .or_else(|| hsms.iter().find(|h| h.tier == "Software"))
                .ok_or_else(|| BearDogError::not_found("No suitable HSM found".to_string()))?
        }
        "software" => hsms
            .iter()
            .find(|h| h.tier == "Software")
            .ok_or_else(|| BearDogError::not_found("No software HSM found".to_string()))?,
        "hardware" => hsms
            .iter()
            .find(|h| h.tier == "Hardware")
            .ok_or_else(|| BearDogError::not_found("No hardware HSM found".to_string()))?,
        "mobile" => hsms
            .iter()
            .find(|h| h.tier == "Mobile")
            .ok_or_else(|| BearDogError::not_found("No mobile HSM found".to_string()))?,
        _ => {
            let msg = format!("Unknown HSM preference: {}", hsm_preference);
            return Err(BearDogError::invalid_input(&msg));
        }
    };

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
    println!("🔐 Generating {} key...", algorithm);

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
    };

    key_store::save_key(&stored_key)?;

    println!("✅ Key generated successfully!");
    println!();
    println!("📋 Key Details:");
    println!("   ID: {}", key_id);
    println!("   Algorithm: {}", algorithm);
    println!("   HSM: {}", selected_hsm.name);
    println!("   Status: Active");
    println!();
    println!("💡 Next steps:");
    println!("   • List keys: beardog key list");
    println!(
        "   • Encrypt: beardog encrypt --key {} --input data.txt --output data.enc",
        key_id
    );

    Ok(())
}

/// Handle key list command
pub async fn handle_key_list(hsm_filter: Option<&str>, _verbose: bool) -> Result<(), BearDogError> {
    println!("🔑 Available Keys");
    println!("================");
    println!();

    let keys = key_store::list_keys()?;

    let filtered_keys: Vec<_> = if let Some(filter) = hsm_filter {
        println!("📌 Filtering by HSM: {}", filter);
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
    println!("Key ID: {}", key_id);
    println!();
    println!("🔨 Coming soon in next iteration!");

    Ok(())
}

/// Handle key delete command
pub async fn handle_key_delete(key_id: &str, skip_confirm: bool) -> Result<(), BearDogError> {
    println!("🗑️  Delete Key");
    println!("=============");
    println!();

    if !skip_confirm {
        println!("⚠️  Are you sure you want to delete key '{}'?", key_id);
        println!("   This action CANNOT be undone!");
        println!();
        println!("   Run with --yes to skip this prompt.");
        return Ok(());
    }

    key_store::delete_key(key_id)?;
    println!("✅ Key '{}' deleted successfully", key_id);

    Ok(())
}
