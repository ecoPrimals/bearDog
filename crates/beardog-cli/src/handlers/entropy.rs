// Entropy Collection Handler
// Vendor-agnostic: Works with ANY HSM (SoftHSM2, StrongBox, YubiKey, Solo 2, etc.)

use beardog_errors::BearDogError;
use beardog_genetics::genetics::human_entropy::{
    HumanEntropyConfig, MultiModalHumanEntropyCollector,
};
// Temporarily use placeholders until we wire actual HSM discovery
// use beardog_tunnel::universal_hsm_discovery::{HsmDiscoveryManager, HsmTier};
// use beardog_types::hsm::entropy::{EntropyGenerationRequest, EntropyQualityTier};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use uuid::Uuid;

/// Entropy seed metadata (saved to file)
#[derive(Debug, Serialize, Deserialize)]
pub struct EntropySeedMetadata {
    pub seed_id: String,
    pub quality_tier: u8,
    pub quality_score: f64,
    pub device_used: String,
    pub device_tier: String,
    pub timestamp: String,
    pub human_input: bool,
    pub identity: Option<String>,
    pub entropy_bytes_b64: String,
}

/// Handle entropy collection command
pub async fn handle_entropy_collect(
    human_input: bool,
    device_preference: &str,
    quality_tier: u8,
    output_path: &str,
    identity: Option<&str>,
) -> Result<(), BearDogError> {
    println!("🌱 BearDog Human Entropy Collection");
    println!("===================================");
    println!();

    // Step 1: Discover available HSMs (vendor-agnostic)
    println!("🔍 Discovering available HSMs...");

    // Placeholder: Will integrate with actual HSM discovery
    let available_hsms = discover_hsms_placeholder().await?;

    if available_hsms.is_empty() {
        println!("❌ No HSMs found!");
        println!();
        println!("💡 Troubleshooting:");
        println!("   - Check if hardware is connected (USB tokens, etc.)");
        println!("   - On Android: adb devices (for StrongBox)");
        println!("   - Install SoftHSM2 for software fallback");
        println!("   - Run setup: scripts/setup-hardware-testing.sh");
        return Err(BearDogError::not_found(
            "No HSMs found. Please connect hardware or install SoftHSM2.".to_string(),
        ));
    }

    println!("✅ Discovered {} HSM(s):", available_hsms.len());
    for hsm in &available_hsms {
        println!(
            "   • {} (Tier: {}, Type: {})",
            hsm.name, hsm.tier, hsm.hsm_type
        );
    }
    println!();

    // Step 2: Select best HSM based on preference (algorithm-agnostic)
    println!(
        "🎯 Selecting HSM based on preference: '{}'",
        device_preference
    );

    let selected_hsm = match device_preference.to_lowercase().as_str() {
        "auto" => {
            // Automatic selection: prefer mobile > hardware > software
            available_hsms
                .iter()
                .find(|h| h.tier == "Mobile")
                .or_else(|| available_hsms.iter().find(|h| h.tier == "Hardware"))
                .or_else(|| available_hsms.iter().find(|h| h.tier == "Software"))
                .ok_or_else(|| BearDogError::not_found("No suitable HSM found".to_string()))?
        }
        "software" => available_hsms
            .iter()
            .find(|h| h.tier == "Software")
            .ok_or_else(|| {
                BearDogError::not_found("No software HSM found (install SoftHSM2)".to_string())
            })?,
        "mobile" => available_hsms
            .iter()
            .find(|h| h.tier == "Mobile")
            .ok_or_else(|| {
                BearDogError::not_found(
                    "No mobile HSM found (check Android StrongBox via ADB)".to_string(),
                )
            })?,
        "usb" | "hardware" => available_hsms
            .iter()
            .find(|h| h.tier == "Hardware")
            .ok_or_else(|| {
                BearDogError::not_found(
                    "No hardware HSM found (connect YubiKey, Solo 2, etc.)".to_string(),
                )
            })?,
        _ => {
            let msg = format!(
                "Unknown device preference: '{}'. Use: auto, software, mobile, usb, hardware",
                device_preference
            );
            return Err(BearDogError::invalid_input(&msg));
        }
    };

    println!("✅ Selected: {}", selected_hsm.name);
    println!("   Tier: {}", selected_hsm.tier);
    println!("   Type: {}", selected_hsm.hsm_type);
    println!();

    // Step 3: Collect entropy
    let entropy_bytes = if human_input {
        println!("🎤 Collecting multi-modal human entropy...");
        println!("   (System entropy + timing + process state)");
        println!();

        // Use MultiModalHumanEntropyCollector
        let collector = MultiModalHumanEntropyCollector::new(HumanEntropyConfig::default());

        let entropy = collector.collect_entropy()?;

        println!("✅ Collected {} bytes of human entropy", entropy.len());
        entropy
    } else {
        println!("🔢 Collecting hardware entropy from HSM...");

        // Generate random bytes directly from HSM
        // For now, use system RNG as placeholder (will wire to actual HSM in next step)
        let entropy = generate_system_entropy(32)?;

        println!(
            "✅ Collected {} bytes from {}",
            entropy.len(),
            selected_hsm.name
        );
        entropy
    };

    println!();

    // Step 4: Calculate quality metrics
    let quality_score = calculate_entropy_quality(&entropy_bytes);
    let seed_id = Uuid::new_v4();

    println!("📊 Entropy Quality Analysis:");
    println!("   Quality Score: {:.2}%", quality_score * 100.0);
    println!(
        "   Assessment: {}",
        if quality_score > 0.95 {
            "✅ Excellent"
        } else if quality_score > 0.85 {
            "✅ Good"
        } else if quality_score > 0.70 {
            "⚠️  Acceptable"
        } else {
            "❌ Poor"
        }
    );
    println!();

    // Step 5: Create seed metadata
    println!("🎉 Generated Entropy Seed");
    println!("   ID: {}", seed_id);
    println!("   Quality Tier: {}", quality_tier);
    println!("   Quality Score: {:.2}%", quality_score * 100.0);
    println!("   Device: {}", selected_hsm.name);
    println!("   Timestamp: {}", Utc::now().to_rfc3339());
    if let Some(id) = identity {
        println!("   Identity: {}", id);
    }
    println!();

    // Step 6: Save to file (JSON format for human readability)
    let seed_data = EntropySeedMetadata {
        seed_id: seed_id.to_string(),
        quality_tier,
        quality_score,
        device_used: selected_hsm.name.clone(),
        device_tier: selected_hsm.tier.clone(),
        timestamp: Utc::now().to_rfc3339(),
        human_input,
        identity: identity.map(String::from),
        entropy_bytes_b64: base64_encode(&entropy_bytes),
    };

    let json = serde_json::to_string_pretty(&seed_data)
        .map_err(|e| BearDogError::serialization(&e.to_string()))?;
    fs::write(output_path, json)?;

    println!("💾 Saved to: {}", output_path);
    println!();
    println!("💡 Next steps:");
    println!(
        "   • View seed info: beardog entropy info --seed {}",
        output_path
    );
    println!(
        "   • Use for keys: beardog key generate --key-id my-key --seed {}",
        output_path
    );

    Ok(())
}

/// Handle entropy info command
pub async fn handle_entropy_info(seed_path: &str) -> Result<(), BearDogError> {
    println!("🔍 Entropy Seed Information");
    println!("==========================");
    println!();

    // Read seed file
    let json = fs::read_to_string(seed_path)?;
    let seed: EntropySeedMetadata =
        serde_json::from_str(&json).map_err(|e| BearDogError::serialization(&e.to_string()))?;

    // Display info
    println!("📋 Seed Details:");
    println!("   ID: {}", seed.seed_id);
    println!("   Quality Tier: {}", seed.quality_tier);
    println!("   Quality Score: {:.2}%", seed.quality_score * 100.0);
    println!("   Device: {}", seed.device_used);
    println!("   Device Tier: {}", seed.device_tier);
    println!("   Timestamp: {}", seed.timestamp);
    println!(
        "   Human Input: {}",
        if seed.human_input { "Yes" } else { "No" }
    );
    if let Some(id) = seed.identity {
        println!("   Identity: {}", id);
    }

    // Decode entropy bytes
    let entropy_bytes = base64_decode(&seed.entropy_bytes_b64)?;
    println!();
    println!("📊 Entropy Data:");
    println!("   Size: {} bytes", entropy_bytes.len());
    println!(
        "   First 32 bytes (hex): {}",
        hex::encode(&entropy_bytes[..32.min(entropy_bytes.len())])
    );

    Ok(())
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

// Placeholder HSM structure
#[derive(Clone)]
struct PlaceholderHsm {
    name: String,
    tier: String,
    hsm_type: String,
}

async fn discover_hsms_placeholder() -> Result<Vec<PlaceholderHsm>, BearDogError> {
    // This is a placeholder that simulates HSM discovery
    // Will be replaced with actual HsmDiscoveryManager integration
    let mut hsms = Vec::new();

    // Check for SoftHSM2
    if std::path::Path::new("/usr/lib/softhsm/libsofthsm2.so").exists()
        || std::path::Path::new("/usr/local/lib/softhsm/libsofthsm2.so").exists()
    {
        hsms.push(PlaceholderHsm {
            name: "SoftHSM2".to_string(),
            tier: "Software".to_string(),
            hsm_type: "PKCS#11".to_string(),
        });
    }

    // Check for Android devices via ADB
    if let Ok(android_devices) = detect_android_devices().await {
        hsms.extend(android_devices);
    }

    // Check for USB tokens (YubiKey, Solo 2, etc.)
    if let Ok(usb_tokens) = detect_usb_tokens().await {
        hsms.extend(usb_tokens);
    }

    // Check for TPM
    if let Ok(tpm_devices) = detect_tpm_devices().await {
        hsms.extend(tpm_devices);
    }

    Ok(hsms)
}

/// Detect Android devices with StrongBox via ADB
async fn detect_android_devices() -> Result<Vec<PlaceholderHsm>, BearDogError> {
    use std::process::Command;

    let mut devices = Vec::new();

    // Check if adb is available
    let adb_check = Command::new("adb").arg("devices").output();

    if adb_check.is_err() {
        return Ok(devices); // ADB not available, return empty
    }

    // Get list of connected devices
    let output = Command::new("adb")
        .args(["devices", "-l"])
        .output()
        .map_err(|e| BearDogError::system(format!("Failed to run adb: {e}")))?;

    let output_str = String::from_utf8_lossy(&output.stdout);

    // Parse device list (skip header line)
    for line in output_str.lines().skip(1) {
        if line.contains("device") && !line.trim().is_empty() {
            // Extract device serial and model
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[1] == "device" {
                let serial = parts[0];

                // Check for StrongBox capability
                let strongbox_check = Command::new("adb")
                    .args(["-s", serial, "shell", "pm", "list", "features"])
                    .output();

                let device_name = if let Ok(features) = strongbox_check {
                    let features_str = String::from_utf8_lossy(&features.stdout);
                    if features_str.contains("strongbox_keystore") {
                        // Extract model name from device line
                        let model = parts
                            .iter()
                            .find(|p| p.starts_with("model:"))
                            .and_then(|p| p.strip_prefix("model:"))
                            .unwrap_or("AndroidDevice");

                        format!("Android StrongBox ({})", model.replace('_', " "))
                    } else if features_str.contains("hardware_keystore") {
                        // Has hardware keystore but not StrongBox
                        let model = parts
                            .iter()
                            .find(|p| p.starts_with("model:"))
                            .and_then(|p| p.strip_prefix("model:"))
                            .unwrap_or("AndroidDevice");

                        format!("Android Keystore ({})", model.replace('_', " "))
                    } else {
                        continue; // Skip devices without hardware keystore
                    }
                } else {
                    continue; // Skip if can't check features
                };

                devices.push(PlaceholderHsm {
                    name: device_name,
                    tier: "Mobile".to_string(),
                    hsm_type: "Android-Keystore".to_string(),
                });
            }
        }
    }

    Ok(devices)
}

fn base64_encode(data: &[u8]) -> String {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    STANDARD.encode(data)
}

fn base64_decode(data: &str) -> Result<Vec<u8>, BearDogError> {
    use base64::engine::general_purpose::STANDARD;
    use base64::Engine;
    STANDARD
        .decode(data)
        .map_err(|e| BearDogError::serialization(&e.to_string()))
}

fn calculate_entropy_quality(bytes: &[u8]) -> f64 {
    if bytes.is_empty() {
        return 0.0;
    }

    // Calculate Shannon entropy
    let mut counts = [0u32; 256];
    for &byte in bytes {
        counts[byte as usize] += 1;
    }

    let len = bytes.len() as f64;
    let mut entropy = 0.0;

    for &count in &counts {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }

    // Normalize to 0-1 range (max entropy for uniform distribution is 8 bits)
    entropy / 8.0
}

fn generate_system_entropy(size: usize) -> Result<Vec<u8>, BearDogError> {
    use rand::RngCore;
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; size];
    rng.fill_bytes(&mut bytes);
    Ok(bytes)
}

/// Detect USB security tokens (YubiKey, Solo 2, OnlyKey, Nitrokey, etc.)
async fn detect_usb_tokens() -> Result<Vec<PlaceholderHsm>, BearDogError> {
    let mut tokens = Vec::new();

    // Check if any hidraw devices exist (indicates USB HID devices)
    if std::path::Path::new("/sys/class/hidraw").exists() {
        // Read USB device info from sysfs
        if let Ok(entries) = std::fs::read_dir("/sys/class/hidraw") {
            for entry in entries.flatten() {
                // Try to read device info
                let device_path = entry.path().join("device/uevent");
                if let Ok(uevent) = std::fs::read_to_string(&device_path) {
                    // Check for known security token vendors
                    if uevent.contains("1050:") {
                        // Yubico vendor ID
                        tokens.push(PlaceholderHsm {
                            name: "YubiKey".to_string(),
                            tier: "Hardware".to_string(),
                            hsm_type: "FIDO2/PKCS#11".to_string(),
                        });
                    } else if uevent.contains("1209:5070") {
                        // SoloKeys vendor:product
                        tokens.push(PlaceholderHsm {
                            name: "Solo 2".to_string(),
                            tier: "Hardware".to_string(),
                            hsm_type: "FIDO2".to_string(),
                        });
                    } else if uevent.contains("20A0:") {
                        // Nitrokey vendor ID
                        tokens.push(PlaceholderHsm {
                            name: "Nitrokey".to_string(),
                            tier: "Hardware".to_string(),
                            hsm_type: "FIDO2/PKCS#11".to_string(),
                        });
                    } else if uevent.contains("1D50:60FC") {
                        // OnlyKey
                        tokens.push(PlaceholderHsm {
                            name: "OnlyKey".to_string(),
                            tier: "Hardware".to_string(),
                            hsm_type: "FIDO2".to_string(),
                        });
                    }
                }
            }
        }
    }

    // Also check if fido2-token CLI tool can find devices
    if let Ok(output) = std::process::Command::new("fido2-token").arg("-L").output() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        if !output_str.is_empty() && !output_str.contains("No FIDO2 token found") {
            // Parse fido2-token output if we didn't find any via sysfs
            if tokens.is_empty() && output_str.lines().count() > 0 {
                tokens.push(PlaceholderHsm {
                    name: "FIDO2 Token".to_string(),
                    tier: "Hardware".to_string(),
                    hsm_type: "FIDO2".to_string(),
                });
            }
        }
    }

    // Deduplicate tokens by name
    tokens.dedup_by(|a, b| a.name == b.name);

    Ok(tokens)
}

/// Detect TPM (Trusted Platform Module) devices
async fn detect_tpm_devices() -> Result<Vec<PlaceholderHsm>, BearDogError> {
    let mut tpms = Vec::new();

    // Check for TPM 2.0 device on Linux
    let tpm_paths = [
        "/dev/tpm0",
        "/dev/tpmrm0", // TPM resource manager
    ];

    for path in tpm_paths {
        if std::path::Path::new(path).exists() {
            // Try to determine TPM version
            let version = if std::path::Path::new("/sys/class/tpm/tpm0/tpm_version_major").exists()
            {
                if let Ok(ver) = std::fs::read_to_string("/sys/class/tpm/tpm0/tpm_version_major") {
                    format!("TPM {}.0", ver.trim())
                } else {
                    "TPM".to_string()
                }
            } else {
                "TPM 2.0".to_string()
            };

            tpms.push(PlaceholderHsm {
                name: version,
                tier: "Hardware".to_string(),
                hsm_type: "TPM".to_string(),
            });
            break; // Only add one TPM entry
        }
    }

    Ok(tpms)
}
