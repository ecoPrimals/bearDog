// SPDX-License-Identifier: AGPL-3.0-only

// Agnostic HSM Discovery
// Uses beardog-tunnel universal discovery - NO hardcoded paths or vendor IDs

use beardog_errors::BearDogError;
use beardog_tunnel::tunnel::hsm::types::HsmTier;
use beardog_tunnel::{DiscoveredHsm, DiscoveryEngine};

/// Convert discovered HSM to CLI-friendly format
#[derive(Debug, Clone)]
pub struct CliHsmInfo {
    pub id: String,
    pub name: String,
    pub vendor: String,
    pub model: String,
    pub tier: String,
    pub hsm_type: String,
    pub path: String,
    pub interface_detail: String,
}

impl From<DiscoveredHsm> for CliHsmInfo {
    fn from(hsm: DiscoveredHsm) -> Self {
        let tier_str = match hsm.assigned_tier {
            HsmTier::Hardware => "Hardware",
            HsmTier::Software => "Software",
            HsmTier::Cloud => "Cloud",
            HsmTier::Mobile => "Mobile",
            _ => "Unknown",
        };

        let (hsm_type, path) = match hsm.interface_type {
            beardog_tunnel::HsmInterfaceType::CloudKms { provider, .. } => {
                ("CloudKMS".to_string(), provider)
            }
            beardog_tunnel::HsmInterfaceType::NetworkHsm { endpoint, .. } => {
                ("NetworkHSM".to_string(), endpoint)
            }
            beardog_tunnel::HsmInterfaceType::UsbHsm { device_id } => {
                ("USB Token".to_string(), device_id)
            }
            beardog_tunnel::HsmInterfaceType::SoftwareHsm { implementation } => {
                ("Software".to_string(), implementation)
            }
            beardog_tunnel::HsmInterfaceType::Tpm { version } => ("TPM".to_string(), version),
            beardog_tunnel::HsmInterfaceType::SmartCard { reader } => {
                ("Smart Card".to_string(), reader)
            }
            beardog_tunnel::HsmInterfaceType::MobileHsm { platform, .. } => {
                ("Mobile HSM".to_string(), platform)
            }
            beardog_tunnel::HsmInterfaceType::CustomApi { api_type, endpoint } => {
                (api_type, endpoint)
            }
        };

        // Generate deterministic unique ID based on HSM properties
        // This ensures discovery is reproducible and testable
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();
        hasher.update(hsm.vendor.as_bytes());
        hasher.update(hsm.model.as_bytes());
        hasher.update(hsm_type.as_bytes());
        hasher.update(path.as_bytes());
        let hash = hasher.finalize();

        let id = format!(
            "{}-{:x}",
            hsm.vendor.to_lowercase(),
            u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]])
        );

        Self {
            id,
            name: format!("{} {}", hsm.vendor, hsm.model),
            vendor: hsm.vendor.clone(),
            model: hsm.model,
            tier: tier_str.to_string(),
            hsm_type: hsm_type.clone(),
            path: path.clone(),
            interface_detail: format!("{} via {}", hsm_type, path),
        }
    }
}

/// Discover all available HSMs using universal discovery
pub async fn discover_all_hsms() -> Result<Vec<CliHsmInfo>, BearDogError> {
    let engine = DiscoveryEngine::new()?;

    let mut all_hsms = Vec::new();

    // Discover PKCS#11 HSMs (any vendor)
    if let Ok(pkcs11_hsms) = engine.discover_pkcs11_hsms() {
        all_hsms.extend(pkcs11_hsms);
    }

    // Discover USB HSMs (any FIDO2/CTAP2 device)
    if let Ok(usb_hsms) = engine.discover_usb_hsms() {
        all_hsms.extend(usb_hsms);
    }

    // Discover Mobile HSMs (StrongBox, Secure Enclave)
    if let Ok(mobile_hsms) = engine.discover_mobile_hsms() {
        all_hsms.extend(mobile_hsms);
    }

    // Discover Software HSMs
    if let Ok(software_hsms) = engine.discover_software_hsms() {
        all_hsms.extend(software_hsms);
    }

    // Discover TPMs
    if let Ok(tpm_hsms) = engine.discover_tpm_hsms() {
        all_hsms.extend(tpm_hsms);
    }

    // Discover Smart Cards
    if let Ok(smartcard_hsms) = engine.discover_smartcard_hsms() {
        all_hsms.extend(smartcard_hsms);
    }

    // Convert to CLI format
    Ok(all_hsms.into_iter().map(CliHsmInfo::from).collect())
}

/// Select best HSM based on user preference
pub fn select_hsm<'a>(
    hsms: &'a [CliHsmInfo],
    preference: &str,
) -> Result<&'a CliHsmInfo, BearDogError> {
    if hsms.is_empty() {
        return Err(BearDogError::not_found(
            "No HSMs found. Please install a PKCS#11 provider or connect hardware.".to_string(),
        ));
    }

    match preference.to_lowercase().as_str() {
        "hardware" | "hw" => {
            // Prefer hardware HSMs
            hsms.iter()
                .filter(|h| h.tier == "Hardware")
                .max_by_key(|h| {
                    // Prioritize by type: USB Token > TPM > Other
                    match h.hsm_type.as_str() {
                        "USB Token" => 100,
                        "TPM" => 50,
                        _ => 10,
                    }
                })
                .ok_or_else(|| BearDogError::not_found("No hardware HSMs found".to_string()))
        }
        "software" | "sw" => {
            // Software HSMs only
            hsms.iter()
                .find(|h| h.tier == "Software")
                .ok_or_else(|| BearDogError::not_found("No software HSMs found".to_string()))
        }
        "mobile" => {
            // Mobile HSMs only
            hsms.iter()
                .find(|h| h.tier == "Mobile")
                .ok_or_else(|| BearDogError::not_found("No mobile HSMs found".to_string()))
        }
        "auto" => {
            // Smart selection: Hardware > Software > Mobile > Cloud
            hsms.iter()
                .max_by_key(|h| match h.tier.as_str() {
                    "Hardware" => 100,
                    "Software" => 50,
                    "Mobile" => 40,
                    "Cloud" => 30,
                    _ => 1,
                })
                .ok_or_else(|| BearDogError::not_found("No HSMs available".to_string()))
        }
        _ => {
            // Unknown preference - default to auto selection
            hsms.iter()
                .max_by_key(|h| match h.tier.as_str() {
                    "Hardware" => 100,
                    "Software" => 50,
                    "Mobile" => 25,
                    _ => 1,
                })
                .ok_or_else(|| BearDogError::not_found("No HSMs available".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discover_finds_hsms() {
        // This test will pass if ANY HSM is found (any vendor)
        let result = discover_all_hsms().await;

        // Should succeed (even if empty - no error)
        assert!(result.is_ok(), "Discovery should not error");

        // If any PKCS#11 provider is installed, we should find at least one
        if let Ok(hsms) = result {
            println!("Found {} HSMs", hsms.len());
            for hsm in &hsms {
                println!("  - {} ({})", hsm.name, hsm.tier);
            }
        }
    }

    #[test]
    fn test_select_hsm_auto() {
        let hsms = vec![
            CliHsmInfo {
                id: "sw-1".to_string(),
                name: "SoftHSM2".to_string(),
                vendor: "OpenDNSSEC".to_string(),
                model: "SoftHSM".to_string(),
                tier: "Software".to_string(),
                hsm_type: "PKCS#11".to_string(),
                path: "/usr/lib/softhsm/libsofthsm2.so".to_string(),
                interface_detail: "PKCS#11 Software HSM".to_string(),
            },
            CliHsmInfo {
                id: "hw-1".to_string(),
                name: "Solo V2".to_string(),
                vendor: "SoloKeys".to_string(),
                model: "Solo 2".to_string(),
                tier: "Hardware".to_string(),
                hsm_type: "USB Token".to_string(),
                path: "USB".to_string(),
                interface_detail: "FIDO2/CTAP2 Hardware Token".to_string(),
            },
        ];

        let selected = select_hsm(&hsms, "auto").unwrap();

        // Should prefer hardware
        assert_eq!(selected.tier, "Hardware");
        assert_eq!(selected.vendor, "SoloKeys");
    }

    #[test]
    fn test_select_hsm_software() {
        let hsms = vec![CliHsmInfo {
            id: "sw-1".to_string(),
            name: "PKCS#11 Provider".to_string(),
            vendor: "Generic".to_string(),
            model: "Software HSM".to_string(),
            tier: "Software".to_string(),
            hsm_type: "PKCS#11".to_string(),
            path: "discovered-via-pkcs11-scan".to_string(),
            interface_detail: "PKCS#11".to_string(),
        }];

        let selected = select_hsm(&hsms, "software").unwrap();
        assert_eq!(selected.tier, "Software");
    }

    #[test]
    fn test_select_hsm_no_hsms() {
        let hsms: Vec<CliHsmInfo> = vec![];
        let result = select_hsm(&hsms, "auto");
        assert!(result.is_err());
    }
}
